//! Engine-independent game simulation.
//!
//! Nothing in this module knows about Bevy, meshes or pixels. The view layer
//! reads this state every frame and mirrors it into entities.

pub mod cell;
pub mod config;
pub mod enemy;
pub mod grid;
pub mod maze;
pub mod pathfinding;
pub mod position;
pub mod projectile;
pub mod state;
pub mod tower;
pub mod wave;

pub use cell::Cell;
pub use config::*;
pub use enemy::Enemy;
pub use grid::Grid;
pub use maze::{Level, generate_level};
pub use pathfinding::{Walkability, astar};
pub use position::Position;
pub use projectile::Projectile;
pub use state::{Game, MessageTone, PlacementError};
pub use tower::{Tower, TowerKind};
pub use wave::EnemyKind;
