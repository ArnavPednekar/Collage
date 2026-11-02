//! Conversions between maze cells and Bevy world space.
//!
//! The maze is centred on the world origin. One cell is one world unit, `+X`
//! runs right and `+Z` runs "down" the maze (matching cell `y`).

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::game::{COLS, Cell, Position, ROWS};

/// Height of the walkable grass plateau.
pub const FLOOR_TOP: f32 = 0.58;
/// Base height of maze wall cliffs.
pub const WALL_TOP: f32 = 1.03;
/// Height of the forest floor around the island.
pub const GROUND_Y: f32 = -0.04;
pub const ENEMY_Y: f32 = FLOOR_TOP + 0.05;
pub const PATH_Y: f32 = FLOOR_TOP + 0.1;
pub const HOVER_Y: f32 = FLOOR_TOP + 0.02;
pub const MARKER_Y: f32 = FLOOR_TOP + 0.04;
pub const HEALTH_BAR_Y: f32 = FLOOR_TOP + 1.05;
pub const PROJECTILE_Y: f32 = FLOOR_TOP + 0.5;
pub const WEAPON_Y: f32 = 0.92;

/// World position of a cell centre at the given height.
pub fn cell_world(cell: Cell, y: f32) -> Vec3 {
    Vec3::new(
        cell.x as f32 + 0.5 - COLS as f32 / 2.0,
        y,
        cell.y as f32 + 0.5 - ROWS as f32 / 2.0,
    )
}

/// World position of a continuous grid position at the given height.
pub fn position_world(position: Position, y: f32) -> Vec3 {
    Vec3::new(
        position.x - COLS as f32 / 2.0,
        y,
        position.y - ROWS as f32 / 2.0,
    )
}

/// Cell under a world-space point, if the point is on the maze.
pub fn world_cell(point: Vec3) -> Option<Cell> {
    let x = (point.x + COLS as f32 / 2.0).floor() as i32;
    let y = (point.z + ROWS as f32 / 2.0).floor() as i32;
    let cell = Cell::new(x, y);
    cell.in_bounds(COLS, ROWS).then_some(cell)
}

/// Yaw for a model whose forward axis is `+Z`, given a grid-space aim angle.
pub fn aim_yaw(aim: f32) -> f32 {
    FRAC_PI_2 - aim
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_round_trips_through_world() {
        for cell in [
            Cell::new(0, 0),
            Cell::new(5, 9),
            Cell::new(COLS - 1, ROWS - 1),
        ] {
            let world = cell_world(cell, 0.0);
            assert_eq!(world_cell(world), Some(cell));
        }
    }

    #[test]
    fn outside_the_maze_has_no_cell() {
        assert_eq!(world_cell(Vec3::new(-100.0, 0.0, 0.0)), None);
    }
}
