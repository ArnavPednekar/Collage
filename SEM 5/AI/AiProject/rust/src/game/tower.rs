//! Towers: target selection, firing and upgrades.

use std::f32::consts::FRAC_PI_2;

use super::cell::Cell;
use super::config::{MAX_TOWER_LEVEL, SELL_REFUND};
use super::enemy::Enemy;
use super::position::Position;

/// Tower archetypes. Visual models are chosen by the view layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TowerKind {
    /// Cheap all-rounder.
    Basic,
    /// Long range, slow, high damage.
    Sniper,
    /// Short range, very fast, low damage.
    Rapid,
    /// Slow, heavy splash damage.
    Cannon,
    /// Slows enemies with cryo rounds.
    Frost,
    /// Rapid arc bolts that chain between targets.
    Tesla,
}

/// Base tower stats. Ranges and projectile speeds are already in cells.
#[derive(Clone, Copy, Debug)]
pub struct TowerStats {
    pub name: &'static str,
    pub cost: i32,
    pub damage: f32,
    pub fire_rate: f32,
    pub range: f32,
    pub projectile_speed: f32,
    /// Splash radius in cells. Zero for single-target towers.
    pub splash: f32,
    /// Speed multiplier applied on hit. One means no slow.
    pub slow: f32,
    pub slow_duration: f32,
    /// Extra targets hit by a chaining shot.
    pub chain: u8,
    pub chain_range: f32,
}

impl TowerKind {
    pub const ALL: [TowerKind; 6] = [
        TowerKind::Basic,
        TowerKind::Sniper,
        TowerKind::Rapid,
        TowerKind::Cannon,
        TowerKind::Frost,
        TowerKind::Tesla,
    ];

    pub fn stats(self) -> TowerStats {
        let base = |name, cost, damage, fire_rate, range, projectile_speed| TowerStats {
            name,
            cost,
            damage,
            fire_rate,
            range,
            projectile_speed,
            splash: 0.0,
            slow: 1.0,
            slow_duration: 0.0,
            chain: 0,
            chain_range: 0.0,
        };
        match self {
            TowerKind::Basic => base("Pulse", 60, 12.0, 1.25, 3.0, 460.0 / 40.0),
            TowerKind::Sniper => base("Rail", 110, 48.0, 0.55, 6.5, 1100.0 / 40.0),
            TowerKind::Rapid => base("Scatter", 85, 6.0, 4.0, 2.2, 620.0 / 40.0),
            TowerKind::Cannon => TowerStats {
                splash: 1.15,
                ..base("Mortar", 140, 44.0, 0.5, 4.2, 520.0 / 40.0)
            },
            TowerKind::Frost => TowerStats {
                slow: 0.45,
                slow_duration: 1.6,
                ..base("Cryo", 120, 8.0, 1.1, 3.2, 500.0 / 40.0)
            },
            TowerKind::Tesla => TowerStats {
                chain: 2,
                chain_range: 1.6,
                ..base("Arc", 160, 20.0, 1.6, 2.8, 900.0 / 40.0)
            },
        }
    }

    pub fn index(self) -> usize {
        match self {
            TowerKind::Basic => 0,
            TowerKind::Sniper => 1,
            TowerKind::Rapid => 2,
            TowerKind::Cannon => 3,
            TowerKind::Frost => 4,
            TowerKind::Tesla => 5,
        }
    }
}

/// A firing solution produced by a tower this step.
#[derive(Clone, Copy, Debug)]
pub struct Shot {
    pub target_id: u64,
    pub origin: Position,
    pub kind: TowerKind,
    pub damage: f32,
    pub speed: f32,
}

#[derive(Clone, Debug)]
pub struct Tower {
    pub kind: TowerKind,
    pub cell: Cell,
    pub pos: Position,
    pub range: f32,
    pub damage: f32,
    pub fire_rate: f32,
    pub cooldown: f32,
    /// Aim angle in grid space (radians, `y` down).
    pub aim: f32,
    pub level: u8,
    /// Gold spent on the tower including every upgrade.
    pub invested: i32,
}

impl Tower {
    pub fn new(kind: TowerKind, cell: Cell) -> Self {
        let stats = kind.stats();
        Self {
            kind,
            cell,
            pos: cell.center(),
            range: stats.range,
            damage: stats.damage,
            fire_rate: stats.fire_rate,
            cooldown: 0.0,
            aim: -FRAC_PI_2,
            level: 1,
            invested: stats.cost,
        }
    }

    /// Cost of the next upgrade, or `None` at the maximum level.
    pub fn upgrade_cost(&self) -> Option<i32> {
        if self.level >= MAX_TOWER_LEVEL {
            return None;
        }
        let multiplier = match self.level {
            1 => 0.8,
            _ => 1.2,
        };
        Some((self.kind.stats().cost as f32 * multiplier).round() as i32)
    }

    /// Apply one upgrade. Returns the gold spent.
    pub fn upgrade(&mut self) -> Option<i32> {
        let cost = self.upgrade_cost()?;
        self.level += 1;
        self.damage *= 1.4;
        self.range *= 1.1;
        self.fire_rate *= 1.08;
        self.invested += cost;
        Some(cost)
    }

    pub fn sell_value(&self) -> i32 {
        (self.invested as f32 * SELL_REFUND).round() as i32
    }

    /// Tick the tower and return a shot when it is ready and has a target.
    ///
    /// Targets must be inside range and alive; the one closest to the core is
    /// preferred so leaks are prevented rather than trailing enemies farmed.
    pub fn update(&mut self, dt: f32, enemies: &[Enemy]) -> Option<Shot> {
        self.cooldown -= dt;
        if self.cooldown > 0.0 {
            return None;
        }

        let mut target: Option<&Enemy> = None;
        let mut best = f32::INFINITY;
        for enemy in enemies {
            if !enemy.alive || enemy.leaked {
                continue;
            }
            if self.pos.distance_to(enemy.pos) <= self.range + enemy.radius * 0.5 {
                let distance = enemy.remaining_distance();
                if distance < best {
                    best = distance;
                    target = Some(enemy);
                }
            }
        }

        let target = target?;
        self.cooldown = 1.0 / self.fire_rate;
        self.aim = (target.pos.y - self.pos.y).atan2(target.pos.x - self.pos.x);

        Some(Shot {
            target_id: target.id,
            origin: self.pos,
            kind: self.kind,
            damage: self.damage,
            speed: self.kind.stats().projectile_speed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enemy_at(id: u64, path: Vec<Cell>) -> Enemy {
        let goal = *path.last().unwrap();
        Enemy::new(
            id,
            super::super::wave::EnemyKind::Grunt,
            path,
            goal,
            1.0,
            1.0,
        )
    }

    #[test]
    fn fires_at_enemy_in_range() {
        let mut tower = Tower::new(TowerKind::Basic, Cell::new(5, 5));
        let enemies = vec![enemy_at(7, vec![Cell::new(6, 5), Cell::new(7, 5)])];
        let shot = tower.update(1.0 / 60.0, &enemies).unwrap();
        assert_eq!(shot.target_id, 7);
        assert!(tower.cooldown > 0.0);
    }

    #[test]
    fn respects_cooldown() {
        let mut tower = Tower::new(TowerKind::Rapid, Cell::new(5, 5));
        let enemies = vec![enemy_at(7, vec![Cell::new(6, 5), Cell::new(7, 5)])];
        assert!(tower.update(1.0 / 60.0, &enemies).is_some());
        assert!(tower.update(1.0 / 60.0, &enemies).is_none());
    }

    #[test]
    fn ignores_enemies_out_of_range() {
        let mut tower = Tower::new(TowerKind::Rapid, Cell::new(0, 0));
        let enemies = vec![enemy_at(1, vec![Cell::new(10, 10)])];
        assert!(tower.update(1.0, &enemies).is_none());
    }

    #[test]
    fn prefers_the_enemy_closest_to_the_goal() {
        let mut tower = Tower::new(TowerKind::Sniper, Cell::new(5, 5));
        let far = enemy_at(1, vec![Cell::new(4, 5), Cell::new(5, 5), Cell::new(6, 5)]);
        let close = enemy_at(2, vec![Cell::new(3, 3), Cell::new(4, 3)]);
        let shot = tower.update(1.0, &[far, close]).unwrap();
        assert_eq!(shot.target_id, 2);
    }

    #[test]
    fn ignores_dead_and_leaked_enemies() {
        let mut tower = Tower::new(TowerKind::Basic, Cell::new(5, 5));
        let mut enemy = enemy_at(1, vec![Cell::new(6, 5)]);
        enemy.alive = false;
        assert!(tower.update(1.0, &[enemy]).is_none());
    }

    #[test]
    fn aim_points_at_the_target() {
        let mut tower = Tower::new(TowerKind::Basic, Cell::new(5, 5));
        let enemies = vec![enemy_at(1, vec![Cell::new(6, 5), Cell::new(7, 5)])];
        tower.update(1.0, &enemies).unwrap();
        assert!(tower.aim.abs() < 1e-5);
    }

    #[test]
    fn upgrades_raise_stats_and_investment() {
        let mut tower = Tower::new(TowerKind::Basic, Cell::new(5, 5));
        let damage = tower.damage;
        let range = tower.range;
        let cost = tower.upgrade_cost().unwrap();
        assert_eq!(tower.upgrade(), Some(cost));
        assert_eq!(tower.level, 2);
        assert!(tower.damage > damage);
        assert!(tower.range > range);
        assert_eq!(tower.invested, 60 + cost);
        assert!(tower.upgrade_cost().is_some());
        tower.upgrade();
        assert_eq!(tower.level, MAX_TOWER_LEVEL);
        assert_eq!(tower.upgrade_cost(), None);
        assert_eq!(tower.upgrade(), None);
    }

    #[test]
    fn sell_value_tracks_upgrade_investment() {
        let mut tower = Tower::new(TowerKind::Sniper, Cell::new(5, 5));
        tower.upgrade();
        let expected = ((110 + 88) as f32 * SELL_REFUND).round() as i32;
        assert_eq!(tower.sell_value(), expected);
    }

    #[test]
    fn special_towers_have_their_mechanics() {
        assert!(TowerKind::Cannon.stats().splash > 0.0);
        assert!(TowerKind::Frost.stats().slow < 1.0);
        assert!(TowerKind::Frost.stats().slow_duration > 0.0);
        assert!(TowerKind::Tesla.stats().chain > 0);
    }
}
