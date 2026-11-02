//! AI Tower Defense.
//!
//! The crate is split into three layers:
//! - [`game`]: pure simulation (maze generation, A*, waves, combat).
//! - [`view`]: Bevy entities and meshes that mirror the simulation.
//! - [`ui`]: HUD, tower buttons and end-of-run overlay.

pub mod app;
pub mod game;
pub mod ui;
pub mod view;

pub use app::{GamePlugin, GameRes, run};
