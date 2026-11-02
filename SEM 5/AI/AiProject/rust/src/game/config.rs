//! Balance and layout constants shared by the simulation and the view.

/// Maze width in cells.
pub const COLS: i32 = 23;
/// Maze height in cells.
pub const ROWS: i32 = 17;

/// Probability that a leftover wall is removed while braiding the maze.
pub const BRAID_CHANCE: f64 = 0.32;

pub const START_GOLD: i32 = 320;
pub const START_CORE_HP: i32 = 1000;
pub const TOTAL_WAVES: u32 = 15;
/// Each point of enemy leak damage costs this much core integrity.
pub const CORE_LEAK_SCALE: i32 = 50;
/// Highest tower level reachable through upgrades.
pub const MAX_TOWER_LEVEL: u8 = 3;

/// Reference tile size of the original 2D prototype (pixels per cell).
/// Speeds were authored per second at this scale and are converted to cells.
pub const TILE_PX: f32 = 40.0;

/// Fraction of the build cost returned when a tower is sold.
pub const SELL_REFUND: f32 = 0.6;

pub const MESSAGE_SECONDS: f32 = 2.0;

/// Simulation sub-step used to keep fast projectiles from tunnelling.
pub const MAX_STEP_SECONDS: f32 = 0.025;
/// Upper bound for a frame delta, so a hitch cannot fast-forward the game.
pub const MAX_FRAME_SECONDS: f32 = 0.1;

/// Conversion from the prototype's pixels-per-second to cells-per-second.
pub const fn px_per_second(value: f32) -> f32 {
    value / TILE_PX
}
