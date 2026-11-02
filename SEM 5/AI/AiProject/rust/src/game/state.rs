//! The whole simulation: economy, waves, entities and player actions.
//!
//! `Game` is a plain struct with no rendering concerns. The Bevy layer wraps it
//! in a resource and mirrors its collections into entities.

use std::collections::VecDeque;

use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

use super::cell::Cell;
use super::config::{
    COLS, CORE_LEAK_SCALE, MAX_FRAME_SECONDS, MAX_STEP_SECONDS, MESSAGE_SECONDS, ROWS,
    START_CORE_HP, START_GOLD, TOTAL_WAVES,
};
use super::enemy::Enemy;
use super::grid::Grid;
use super::maze::generate_level;
use super::pathfinding::{Walkability, astar};
use super::projectile::Projectile;
use super::tower::{Tower, TowerKind};
use super::wave::{EnemyKind, build_wave, spawn_indices, wave_scaling};

/// Semantic tone of the transient status message, mapped to a colour by the UI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageTone {
    Info,
    Accent,
    Good,
    Bad,
    Gold,
    Muted,
}

/// Why a tower cannot be placed on a cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlacementError {
    OutsideMap,
    Wall,
    SpawnOrGoal,
    Occupied,
    EnemyHere,
    NotEnoughGold,
    WouldBlockPath,
}

impl PlacementError {
    pub const fn message(self) -> &'static str {
        match self {
            PlacementError::OutsideMap => "Outside the map",
            PlacementError::Wall => "Wall",
            PlacementError::SpawnOrGoal => "Spawn / goal",
            PlacementError::Occupied => "Occupied",
            PlacementError::EnemyHere => "Enemy here",
            PlacementError::NotEnoughGold => "Not enough gold",
            PlacementError::WouldBlockPath => "Would block the path",
        }
    }
}

/// Complete game state.
pub struct Game {
    pub grid: Grid,
    pub nav: Walkability,
    pub core: Cell,
    /// Gate cells ordered left, top, right, bottom.
    pub spawns: [Cell; 4],
    /// Gates currently spawning enemies for the active wave.
    pub active_spawns: Vec<Cell>,

    pub towers: Vec<Tower>,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,

    pub gold: i32,
    pub core_hp: i32,
    pub core_max_hp: i32,
    pub wave: u32,
    pub wave_active: bool,
    pub spawn_queue: VecDeque<EnemyKind>,
    pub spawn_timer: f32,
    pub spawn_interval: f32,
    pub hp_mult: f32,
    pub speed_mult: f32,

    pub selected: TowerKind,
    /// Existing tower selected for upgrade/sell, if any.
    pub selected_tower: Option<Cell>,
    pub show_path: bool,
    pub speed: f32,
    pub game_over: bool,
    pub victory: bool,

    pub message: String,
    pub message_tone: MessageTone,
    pub message_timer: f32,

    /// One A* preview per active gate.
    pub preview_paths: Vec<Vec<Cell>>,

    /// Bumped whenever the maze/towers change so views can rebuild.
    pub level_version: u64,
    pub towers_version: u64,

    rng: StdRng,
    next_id: u64,
}

impl Game {
    pub fn new() -> Self {
        Self::new_seeded(rand::random::<u64>())
    }

    pub fn new_seeded(seed: u64) -> Self {
        let mut game = Self {
            grid: Grid::with_size(),
            nav: Walkability::empty(),
            core: Cell::new(COLS / 2, ROWS / 2),
            spawns: [Cell::new(0, 0); 4],
            active_spawns: Vec::new(),
            towers: Vec::new(),
            enemies: Vec::new(),
            projectiles: Vec::new(),
            gold: START_GOLD,
            core_hp: START_CORE_HP,
            core_max_hp: START_CORE_HP,
            wave: 0,
            wave_active: false,
            spawn_queue: VecDeque::new(),
            spawn_timer: 0.0,
            spawn_interval: 0.8,
            hp_mult: 1.0,
            speed_mult: 1.0,
            selected: TowerKind::Basic,
            selected_tower: None,
            show_path: true,
            speed: 1.0,
            game_over: false,
            victory: false,
            message: String::new(),
            message_tone: MessageTone::Info,
            message_timer: 0.0,
            preview_paths: Vec::new(),
            level_version: 0,
            towers_version: 0,
            rng: StdRng::seed_from_u64(seed),
            next_id: 1,
        };
        game.regenerate();
        game
    }

    /// Generate a fresh maze and reset all progress.
    pub fn regenerate(&mut self) {
        let level = generate_level(&mut self.rng);
        self.grid = level.grid;
        self.core = level.core;
        self.spawns = level.spawns;
        self.active_spawns.clear();
        self.nav = Walkability::from_grid(&self.grid);

        self.towers.clear();
        self.enemies.clear();
        self.projectiles.clear();
        self.spawn_queue.clear();

        self.gold = START_GOLD;
        self.core_hp = START_CORE_HP;
        self.core_max_hp = START_CORE_HP;
        self.wave = 0;
        self.wave_active = false;
        self.spawn_timer = 0.0;
        self.spawn_interval = 0.8;
        self.hp_mult = 1.0;
        self.speed_mult = 1.0;
        self.selected = TowerKind::Basic;
        self.selected_tower = None;
        self.show_path = true;
        self.speed = 1.0;
        self.game_over = false;
        self.victory = false;
        self.message.clear();
        self.message_timer = 0.0;

        self.refresh_preview();
        self.level_version += 1;
        self.towers_version += 1;
    }

    // ------------------------------------------------------------------
    // player actions
    // ------------------------------------------------------------------

    pub fn select(&mut self, kind: TowerKind) {
        self.selected = kind;
        self.selected_tower = None;
    }

    /// Select an existing tower, or clear the selection when the cell is empty.
    pub fn select_tower(&mut self, cell: Cell) {
        self.selected_tower = self.has_tower(cell).then_some(cell);
    }

    pub fn toggle_speed(&mut self) {
        self.speed = if self.speed >= 3.0 {
            1.0
        } else {
            self.speed + 1.0
        };
    }

    pub fn toggle_path(&mut self) {
        self.show_path = !self.show_path;
    }

    pub fn can_start_wave(&self) -> bool {
        !self.wave_active && !self.game_over && !self.victory && self.wave < TOTAL_WAVES
    }

    /// Validate a build without changing the game.
    pub fn placement_error(&self, cell: Cell) -> Result<(), PlacementError> {
        if !self.grid.in_bounds(cell) {
            return Err(PlacementError::OutsideMap);
        }
        if self.grid.is_wall(cell) {
            return Err(PlacementError::Wall);
        }
        if cell == self.core || self.spawns.contains(&cell) {
            return Err(PlacementError::SpawnOrGoal);
        }
        if self.has_tower(cell) {
            return Err(PlacementError::Occupied);
        }
        if self
            .enemies
            .iter()
            .any(|enemy| enemy.alive && enemy.cell() == cell)
        {
            return Err(PlacementError::EnemyHere);
        }
        if self.gold < self.selected.stats().cost {
            return Err(PlacementError::NotEnoughGold);
        }

        let mut nav = self.nav.clone();
        nav.set_blocked(cell, true);
        if self
            .spawns
            .iter()
            .any(|spawn| astar(*spawn, self.core, &nav).is_none())
        {
            return Err(PlacementError::WouldBlockPath);
        }

        Ok(())
    }

    pub fn has_tower(&self, cell: Cell) -> bool {
        self.towers.iter().any(|tower| tower.cell == cell)
    }

    pub fn try_place(&mut self, cell: Cell) {
        if let Err(error) = self.placement_error(cell) {
            self.flash(error.message(), MessageTone::Bad);
            return;
        }

        let stats = self.selected.stats();
        self.gold -= stats.cost;
        self.nav.set_blocked(cell, true);
        self.towers.push(Tower::new(self.selected, cell));
        self.selected_tower = Some(cell);
        self.repath_enemies();
        self.refresh_preview();
        self.towers_version += 1;
        self.flash(format!("{} built", stats.name), MessageTone::Good);
    }

    pub fn sell_tower(&mut self, cell: Cell) {
        let Some(index) = self.towers.iter().position(|tower| tower.cell == cell) else {
            self.flash("No tower on that cell", MessageTone::Muted);
            return;
        };

        let tower = self.towers.remove(index);
        let refund = tower.sell_value();
        self.gold += refund;
        self.nav.set_blocked(cell, false);
        if self.selected_tower == Some(cell) {
            self.selected_tower = None;
        }
        self.repath_enemies();
        self.refresh_preview();
        self.towers_version += 1;
        self.flash(format!("Sold for {refund}g"), MessageTone::Gold);
    }

    /// Upgrade the tower on `cell`, spending gold if possible.
    pub fn upgrade_tower(&mut self, cell: Cell) {
        let Some(index) = self.towers.iter().position(|tower| tower.cell == cell) else {
            self.flash("No tower on that cell", MessageTone::Muted);
            return;
        };
        let Some(cost) = self.towers[index].upgrade_cost() else {
            self.flash("Tower is already max level", MessageTone::Muted);
            return;
        };
        if self.gold < cost {
            self.flash("Not enough gold", MessageTone::Bad);
            return;
        }
        self.gold -= cost;
        let name = self.towers[index].kind.stats().name;
        let level = self.towers[index]
            .upgrade()
            .map_or(0, |_| self.towers[index].level);
        self.selected_tower = Some(cell);
        self.towers_version += 1;
        self.flash(format!("{name} upgraded to Lv{level}"), MessageTone::Good);
    }

    pub fn upgrade_selected(&mut self) {
        if let Some(cell) = self.selected_tower {
            self.upgrade_tower(cell);
        }
    }

    pub fn sell_selected(&mut self) {
        if let Some(cell) = self.selected_tower {
            self.sell_tower(cell);
        }
    }

    // ------------------------------------------------------------------
    // simulation
    // ------------------------------------------------------------------

    pub fn update(&mut self, real_dt: f32) {
        if self.message_timer > 0.0 {
            self.message_timer = (self.message_timer - real_dt).max(0.0);
        }
        if self.game_over || self.victory {
            return;
        }

        let real_dt = real_dt.min(MAX_FRAME_SECONDS);
        let steps = ((real_dt * self.speed / MAX_STEP_SECONDS).ceil() as usize).max(1);
        let step_dt = real_dt * self.speed / steps as f32;
        for _ in 0..steps {
            self.step(step_dt);
        }
    }

    fn step(&mut self, dt: f32) {
        if self.game_over || self.victory {
            return;
        }

        self.spawn_enemies(dt);
        for enemy in &mut self.enemies {
            enemy.update(dt);
        }

        for tower in &mut self.towers {
            if let Some(shot) = tower.update(dt, &self.enemies) {
                let id = self.next_id;
                self.next_id += 1;
                self.projectiles.push(Projectile::new(
                    id,
                    shot.origin,
                    shot.target_id,
                    shot.kind,
                    shot.damage,
                    shot.speed,
                ));
            }
        }

        for projectile in &mut self.projectiles {
            projectile.update(dt, &mut self.enemies);
        }

        self.resolve_combat();

        if self.core_hp <= 0 {
            self.core_hp = 0;
            self.game_over = true;
            self.wave_active = false;
            self.flash("The core has fallen", MessageTone::Bad);
            return;
        }

        self.resolve_wave_end();
    }

    fn spawn_enemies(&mut self, dt: f32) {
        if !self.wave_active {
            return;
        }

        self.spawn_timer -= dt;
        while !self.spawn_queue.is_empty() && self.spawn_timer <= 0.0 {
            let kind = self.spawn_queue.pop_front().unwrap();
            if !self.active_spawns.is_empty() {
                let spawn = self.active_spawns[self.rng.random_range(0..self.active_spawns.len())];
                if let Some(path) = astar(spawn, self.core, &self.nav) {
                    let id = self.next_id;
                    self.next_id += 1;
                    self.enemies.push(Enemy::new(
                        id,
                        kind,
                        path,
                        self.core,
                        self.hp_mult,
                        self.speed_mult,
                    ));
                }
            }
            self.spawn_timer += self.spawn_interval;
        }
    }

    fn resolve_combat(&mut self) {
        let mut core_loss = 0;
        let mut reward = 0;
        let mut survivors = Vec::with_capacity(self.enemies.len());

        for enemy in self.enemies.drain(..) {
            if enemy.alive {
                survivors.push(enemy);
            } else if enemy.leaked {
                core_loss += enemy.leak * CORE_LEAK_SCALE;
            } else {
                reward += enemy.reward;
            }
        }

        self.enemies = survivors;
        self.gold += reward;

        if core_loss > 0 {
            self.core_hp = (self.core_hp - core_loss).max(0);
            self.flash(format!("Core hit  -{core_loss}"), MessageTone::Bad);
        }

        self.projectiles.retain(|projectile| projectile.alive);
    }

    fn resolve_wave_end(&mut self) {
        if !self.wave_active || !self.spawn_queue.is_empty() || !self.enemies.is_empty() {
            return;
        }

        self.wave_active = false;
        self.refresh_preview();
        let bonus = wave_scaling(self.wave).clear_bonus;
        self.gold += bonus;

        if self.wave >= TOTAL_WAVES {
            self.victory = true;
            self.flash("All waves cleared!", MessageTone::Good);
        } else {
            self.flash(
                format!("Wave {} cleared  +{bonus}g", self.wave),
                MessageTone::Good,
            );
        }
    }

    pub fn start_wave(&mut self) {
        if self.game_over || self.victory {
            return;
        }
        if self.wave_active {
            self.flash("Wave already in progress", MessageTone::Muted);
            return;
        }
        if self.wave >= TOTAL_WAVES {
            return;
        }

        self.wave += 1;
        self.active_spawns = spawn_indices(self.wave)
            .into_iter()
            .map(|index| self.spawns[index])
            .collect();
        self.spawn_queue = build_wave(self.wave, &mut self.rng).into();
        self.spawn_timer = 0.0;
        self.spawn_interval = wave_scaling(self.wave).spawn_interval;
        self.hp_mult = wave_scaling(self.wave).hp_mult;
        self.speed_mult = wave_scaling(self.wave).speed_mult;
        self.wave_active = true;
        self.refresh_preview();
        self.flash(format!("Wave {} incoming", self.wave), MessageTone::Accent);
    }

    fn repath_enemies(&mut self) {
        for enemy in &mut self.enemies {
            enemy.repath(&self.nav);
        }
    }

    fn refresh_preview(&mut self) {
        let spawns: Vec<Cell> = self.spawns.to_vec();
        self.preview_paths = spawns
            .into_iter()
            .filter_map(|spawn| astar(spawn, self.core, &self.nav))
            .collect();
    }

    fn flash(&mut self, text: impl Into<String>, tone: MessageTone) {
        self.message = text.into();
        self.message_tone = tone;
        self.message_timer = MESSAGE_SECONDS;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::wave::EnemyKind;

    fn first_buildable(game: &Game) -> Cell {
        game.grid
            .cells()
            .find(|&cell| game.placement_error(cell).is_ok())
            .expect("a generated maze should have a buildable cell")
    }

    fn enemy_to_core(game: &Game, id: u64, kind: EnemyKind) -> Enemy {
        Enemy::new(
            id,
            kind,
            vec![game.spawns[0], game.core],
            game.core,
            1.0,
            1.0,
        )
    }

    #[test]
    fn new_game_has_resources_core_and_paths() {
        let game = Game::new_seeded(1);
        assert_eq!(game.gold, START_GOLD);
        assert_eq!(game.core_hp, START_CORE_HP);
        assert_eq!(game.core_max_hp, START_CORE_HP);
        assert_eq!(game.wave, 0);
        assert!(!game.preview_paths.is_empty());
        for spawn in game.spawns {
            assert!(
                astar(spawn, game.core, &game.nav).is_some(),
                "every gate starts connected"
            );
        }
    }

    #[test]
    fn placing_a_tower_spends_gold_and_blocks_the_cell() {
        let mut game = Game::new_seeded(2);
        let cell = first_buildable(&game);
        let before = game.gold;
        game.try_place(cell);
        assert_eq!(game.gold, before - 60);
        assert!(game.has_tower(cell));
        assert_eq!(game.selected_tower, Some(cell));
        assert!(game.nav.is_blocked(cell));
        assert!(!game.preview_paths.is_empty());
    }

    #[test]
    fn selling_returns_sixty_percent() {
        let mut game = Game::new_seeded(3);
        let cell = first_buildable(&game);
        game.try_place(cell);
        let after_build = game.gold;
        game.sell_tower(cell);
        assert_eq!(game.gold, after_build + 36);
        assert!(!game.has_tower(cell));
        assert_eq!(game.selected_tower, None);
        assert!(!game.nav.is_blocked(cell));
    }

    #[test]
    fn cannot_build_on_walls_core_or_gates() {
        let game = Game::new_seeded(4);
        let wall = game.grid.cells().find(|&c| game.grid.is_wall(c)).unwrap();
        assert_eq!(game.placement_error(wall), Err(PlacementError::Wall));
        assert_eq!(
            game.placement_error(game.core),
            Err(PlacementError::SpawnOrGoal)
        );
        assert_eq!(
            game.placement_error(game.spawns[0]),
            Err(PlacementError::SpawnOrGoal)
        );
        assert_eq!(
            game.placement_error(Cell::new(-1, 0)),
            Err(PlacementError::OutsideMap)
        );
    }

    #[test]
    fn building_requires_gold() {
        let game = Game::new_seeded(5);
        let cell = first_buildable(&game);
        let mut game = game;
        game.gold = 0;
        assert_eq!(
            game.placement_error(cell),
            Err(PlacementError::NotEnoughGold)
        );
    }

    #[test]
    fn sealing_every_route_is_rejected() {
        let mut game = Game::new_seeded(6);
        let mut grid = Grid::new(5, 3);
        for x in 1..=3 {
            grid.set_floor(Cell::new(x, 1), true);
        }
        game.grid = grid;
        game.core = Cell::new(3, 1);
        game.spawns = [Cell::new(1, 1); 4];
        game.nav = Walkability::from_grid(&game.grid);
        game.gold = 1000;
        assert_eq!(
            game.placement_error(Cell::new(2, 1)),
            Err(PlacementError::WouldBlockPath)
        );
    }

    #[test]
    fn starting_a_wave_puts_enemies_on_the_field() {
        let mut game = Game::new_seeded(7);
        game.start_wave();
        assert_eq!(game.wave, 1);
        assert_eq!(game.active_spawns, vec![game.spawns[0]]);
        assert!(game.wave_active);
        for _ in 0..10 {
            game.update(0.1);
        }
        assert!(!game.enemies.is_empty());
    }

    #[test]
    fn wave_schedule_opens_more_gates() {
        let mut game = Game::new_seeded(8);
        game.start_wave();
        assert_eq!(game.active_spawns, vec![game.spawns[0]]);
        game.wave_active = false;
        game.wave = 3;
        game.start_wave();
        assert_eq!(game.active_spawns, vec![game.spawns[3]]);
        game.wave_active = false;
        game.wave = 7;
        game.start_wave();
        assert_eq!(game.active_spawns, game.spawns[1..4].to_vec());
    }

    #[test]
    fn cannot_start_two_waves_at_once() {
        let mut game = Game::new_seeded(9);
        game.start_wave();
        game.start_wave();
        assert_eq!(game.wave, 1);
        assert_eq!(game.message, "Wave already in progress");
    }

    #[test]
    fn leaked_enemy_damages_the_core_and_can_end_the_game() {
        let mut game = Game::new_seeded(10);
        game.core_hp = 50;
        let mut enemy = enemy_to_core(&game, 100, EnemyKind::Grunt);
        enemy.speed = 1000.0;
        game.enemies.push(enemy);
        game.step(0.05);
        assert_eq!(game.core_hp, 0);
        assert!(game.game_over);
    }

    #[test]
    fn clearing_a_wave_pays_a_bonus() {
        let mut game = Game::new_seeded(11);
        game.wave = 1;
        game.wave_active = true;
        let before = game.gold;
        game.step(0.05);
        assert!(!game.wave_active);
        assert_eq!(game.gold, before + 59);
        assert!(game.message.contains("cleared"));
    }

    #[test]
    fn killing_an_enemy_pays_its_reward() {
        let mut game = Game::new_seeded(12);
        let before = game.gold;
        let mut enemy = enemy_to_core(&game, 100, EnemyKind::Grunt);
        enemy.alive = false;
        enemy.leaked = false;
        game.enemies.push(enemy);
        game.step(0.05);
        assert_eq!(game.gold, before + 8);
    }

    #[test]
    fn upgrading_a_tower_costs_gold_and_raises_its_level() {
        let mut game = Game::new_seeded(13);
        let cell = first_buildable(&game);
        game.try_place(cell);
        game.gold = 500;
        let before = game.gold;
        let cost = game.towers[0].upgrade_cost().unwrap();
        game.upgrade_tower(cell);
        assert_eq!(game.gold, before - cost);
        assert_eq!(game.towers[0].level, 2);
        assert!(game.towers[0].damage > TowerKind::Basic.stats().damage);
    }

    #[test]
    fn regenerate_resets_progress() {
        let mut game = Game::new_seeded(14);
        game.gold = 10;
        game.wave = 5;
        game.start_wave();
        game.regenerate();
        assert_eq!(game.gold, START_GOLD);
        assert_eq!(game.wave, 0);
        assert_eq!(game.core_hp, START_CORE_HP);
        assert!(game.enemies.is_empty());
        assert!(!game.preview_paths.is_empty());
    }
}
