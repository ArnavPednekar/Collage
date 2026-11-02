//! Enemies that walk a precomputed route and re-plan when the maze changes.

use super::cell::Cell;
use super::pathfinding::{Walkability, astar};
use super::position::Position;
use super::wave::EnemyKind;

/// One enemy instance, identified by a stable id for projectile targeting.
#[derive(Clone, Debug)]
pub struct Enemy {
    pub id: u64,
    pub kind: EnemyKind,
    pub max_hp: i32,
    pub hp: f32,
    pub speed: f32,
    pub reward: i32,
    pub leak: i32,
    pub radius: f32,
    pub goal: Cell,
    pub path: Vec<Cell>,
    pub idx: usize,
    pub pos: Position,
    pub travelled: f32,
    pub alive: bool,
    pub leaked: bool,
    /// Remaining seconds of a slow effect.
    pub slow_timer: f32,
    /// Speed multiplier while slowed (1.0 = normal).
    pub slow_factor: f32,
}

impl Enemy {
    pub fn new(
        id: u64,
        kind: EnemyKind,
        path: Vec<Cell>,
        goal: Cell,
        hp_mult: f32,
        speed_mult: f32,
    ) -> Self {
        assert!(!path.is_empty(), "an enemy needs at least one path cell");
        let stats = kind.stats();
        let max_hp = ((stats.hp * hp_mult).round() as i32).max(1);
        Self {
            id,
            kind,
            max_hp,
            hp: max_hp as f32,
            speed: stats.speed * speed_mult,
            reward: stats.reward,
            leak: stats.leak,
            radius: stats.radius,
            goal,
            pos: path[0].center(),
            path,
            idx: 1,
            travelled: 0.0,
            alive: true,
            leaked: false,
            slow_timer: 0.0,
            slow_factor: 1.0,
        }
    }

    /// Apply (or refresh) a slow. Stronger slows win, and the timer is refreshed.
    pub fn apply_slow(&mut self, factor: f32, duration: f32) {
        self.slow_factor = self.slow_factor.min(factor.clamp(0.1, 1.0));
        self.slow_timer = self.slow_timer.max(duration);
    }

    pub fn cell(&self) -> Cell {
        self.pos.cell()
    }

    /// Advance along the path, carrying leftover movement into the next cell.
    pub fn update(&mut self, dt: f32) {
        if !self.alive {
            return;
        }

        let speed_factor = if self.slow_timer > 0.0 {
            self.slow_factor
        } else {
            1.0
        };
        let mut remaining = self.speed * speed_factor * dt;
        while remaining > 0.0 && self.idx < self.path.len() {
            let target = self.path[self.idx].center();
            let distance = self.pos.distance_to(target);
            if distance <= remaining {
                self.pos = target;
                self.travelled += distance;
                remaining -= distance;
                self.idx += 1;
            } else {
                let scale = remaining / distance;
                self.pos.x += (target.x - self.pos.x) * scale;
                self.pos.y += (target.y - self.pos.y) * scale;
                self.travelled += remaining;
                remaining = 0.0;
            }
        }

        if self.idx >= self.path.len() {
            self.leaked = true;
            self.alive = false;
        }

        if self.slow_timer > 0.0 {
            self.slow_timer = (self.slow_timer - dt).max(0.0);
            if self.slow_timer == 0.0 {
                self.slow_factor = 1.0;
            }
        }
    }

    /// Recompute the route from the enemy's current cell to the goal.
    /// Returns `false` when no route exists.
    pub fn repath(&mut self, nav: &Walkability) -> bool {
        let Some(path) = astar(self.cell(), self.goal, nav) else {
            return false;
        };
        self.idx = if path.len() > 1 { 1 } else { 0 };
        self.path = path;
        true
    }

    /// Path length left to the goal. Towers prefer the threat closest to it.
    pub fn remaining_distance(&self) -> f32 {
        if self.idx >= self.path.len() {
            return 0.0;
        }
        let mut total = self.pos.distance_to(self.path[self.idx].center());
        for pair in self.path[self.idx..].windows(2) {
            total += pair[0].manhattan(pair[1]) as f32;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn straight_path() -> Vec<Cell> {
        vec![Cell::new(0, 0), Cell::new(1, 0), Cell::new(2, 0)]
    }

    fn enemy() -> Enemy {
        Enemy::new(
            1,
            EnemyKind::Grunt,
            straight_path(),
            Cell::new(2, 0),
            1.0,
            1.0,
        )
    }

    #[test]
    fn starts_at_first_cell_center() {
        let enemy = enemy();
        assert_eq!(enemy.pos, Position::new(0.5, 0.5));
        assert_eq!(enemy.idx, 1);
    }

    #[test]
    fn moves_and_carries_leftover_distance() {
        let mut enemy = enemy();
        enemy.speed = 2.0;
        enemy.update(0.5);
        assert_eq!(enemy.pos, Position::new(1.5, 0.5));
        assert_eq!(enemy.idx, 2);
    }

    #[test]
    fn reaching_the_end_leaks() {
        let mut enemy = enemy();
        enemy.speed = 100.0;
        enemy.update(0.1);
        assert!(enemy.leaked);
        assert!(!enemy.alive);
    }

    #[test]
    fn remaining_distance_uses_the_rest_of_the_path() {
        let enemy = enemy();
        // 1 cell to (1,0), then 1 more cell to the goal.
        assert!((enemy.remaining_distance() - 2.0).abs() < 1e-5);
    }

    #[test]
    fn repath_uses_current_cell() {
        let mut enemy = enemy();
        enemy.update(0.6);
        let nav = Walkability::empty();
        assert!(enemy.repath(&nav));
        assert_eq!(enemy.path.first(), Some(&enemy.cell()));
        assert_eq!(enemy.idx, 1);
    }

    #[test]
    fn hp_scales_with_wave_multiplier() {
        let enemy = Enemy::new(
            1,
            EnemyKind::Tank,
            straight_path(),
            Cell::new(2, 0),
            2.0,
            1.0,
        );
        assert_eq!(enemy.max_hp, 320);
        assert_eq!(enemy.hp, 320.0);
    }

    #[test]
    fn remaining_distance_is_zero_after_leaking() {
        let mut enemy = enemy();
        enemy.speed = 100.0;
        enemy.update(0.1);
        assert_eq!(enemy.remaining_distance(), 0.0);
    }

    #[test]
    fn slow_reduces_movement_and_expires() {
        let mut enemy = enemy();
        enemy.speed = 2.0;
        enemy.apply_slow(0.5, 0.5);
        enemy.update(0.5);
        assert!((enemy.pos.x - 1.0).abs() < 1e-5);
        assert_eq!(enemy.slow_timer, 0.0);
        assert_eq!(enemy.slow_factor, 1.0);
    }
}
