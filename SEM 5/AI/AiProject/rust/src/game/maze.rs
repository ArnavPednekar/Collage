//! Procedural maze generation.
//!
//! 1. A randomised depth-first search (recursive backtracker) carves a perfect
//!    maze on the odd-coordinate lattice.
//! 2. Braiding removes a fraction of the remaining interior walls, turning dead
//!    ends into loops so enemies can re-route around player-built towers.
//! 3. A central chamber is carved for the core, and four gates are opened on
//!    the map edges. Every gate is connected to the core.
//!
//! The result is always solvable: the carve pass connects every lattice cell
//! and braiding only ever removes walls.

use rand::{Rng, RngExt};

use super::cell::Cell;
use super::config::{BRAID_CHANCE, COLS, ROWS};
use super::grid::Grid;

/// A generated maze plus its entry gates and defended core.
#[derive(Clone, Debug)]
pub struct Level {
    pub grid: Grid,
    /// Core cell at the centre of the map.
    pub core: Cell,
    /// Gates ordered left, top, right, bottom.
    pub spawns: [Cell; 4],
}

/// Generate a maze with the default size and braid chance.
pub fn generate_level<R: Rng + ?Sized>(rng: &mut R) -> Level {
    generate_sized(rng, BRAID_CHANCE, COLS, ROWS)
}

/// Generate a maze of the given size.
pub fn generate_sized<R: Rng + ?Sized>(
    rng: &mut R,
    braid_chance: f64,
    cols: i32,
    rows: i32,
) -> Level {
    let mut grid = Grid::new(cols, rows);

    // --- 1. recursive backtracker on odd cells -------------------------
    let start = Cell::new(1, 1);
    grid.set_floor(start, true);
    let mut stack = vec![start];

    while let Some(&current) = stack.last() {
        let mut neighbours = Vec::with_capacity(4);
        for (dx, dy) in [(2, 0), (-2, 0), (0, 2), (0, -2)] {
            let next = Cell::new(current.x + dx, current.y + dy);
            if next.x >= 1
                && next.y >= 1
                && next.x < cols - 1
                && next.y < rows - 1
                && grid.is_wall(next)
            {
                neighbours.push(next);
            }
        }

        if neighbours.is_empty() {
            stack.pop();
            continue;
        }

        let next = neighbours[rng.random_range(0..neighbours.len())];
        let between = Cell::new((current.x + next.x) / 2, (current.y + next.y) / 2);
        grid.set_floor(between, true);
        grid.set_floor(next, true);
        stack.push(next);
    }

    // --- 2. braid: open some walls to create loops ---------------------
    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            let cell = Cell::new(x, y);
            if grid.is_wall(cell) && rng.random_bool(braid_chance) {
                grid.set_floor(cell, true);
            }
        }
    }

    // --- 3. central core chamber ---------------------------------------
    let core = Cell::new(cols / 2, rows / 2);
    for y in core.y - 1..=core.y + 1 {
        for x in core.x - 1..=core.x + 1 {
            let cell = Cell::new(x, y);
            if cell.x >= 1 && cell.y >= 1 && cell.x < cols - 1 && cell.y < rows - 1 {
                grid.set_floor(cell, true);
            }
        }
    }
    grid.set_floor(core, true);

    // --- 4. four edge gates --------------------------------------------
    let odd_rows: Vec<i32> = (1..rows - 1).step_by(2).collect();
    let odd_cols: Vec<i32> = (1..cols - 1).step_by(2).collect();
    let pick = |values: &[i32], rng: &mut R| values[rng.random_range(0..values.len())];

    let left = Cell::new(0, pick(&odd_rows, rng));
    let right = Cell::new(cols - 1, pick(&odd_rows, rng));
    let top = Cell::new(pick(&odd_cols, rng), 0);
    let bottom = Cell::new(pick(&odd_cols, rng), rows - 1);

    let spawns = [left, top, right, bottom];
    for spawn in spawns {
        grid.set_floor(spawn, true);
    }

    Level { grid, core, spawns }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::pathfinding::{Walkability, astar};
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn every_generated_maze_is_solvable_from_every_gate() {
        for seed in 0..300u64 {
            let mut rng = StdRng::seed_from_u64(seed);
            let level = generate_level(&mut rng);
            let nav = Walkability::from_grid(&level.grid);
            for (index, spawn) in level.spawns.iter().enumerate() {
                assert!(
                    astar(*spawn, level.core, &nav).is_some(),
                    "seed {seed} gate {index} could not reach the core"
                );
            }
        }
    }

    #[test]
    fn core_is_centred_and_gates_sit_on_the_edges() {
        let mut rng = StdRng::seed_from_u64(42);
        let level = generate_level(&mut rng);
        assert_eq!(level.core, Cell::new(COLS / 2, ROWS / 2));
        assert!(level.grid.is_floor(level.core));
        assert_eq!(level.spawns[0].x, 0);
        assert_eq!(level.spawns[1].y, 0);
        assert_eq!(level.spawns[2].x, COLS - 1);
        assert_eq!(level.spawns[3].y, ROWS - 1);
        for spawn in level.spawns {
            assert!(level.grid.is_floor(spawn));
        }
    }

    #[test]
    fn braiding_is_deterministic_for_a_seed() {
        let mut a = StdRng::seed_from_u64(7);
        let mut b = StdRng::seed_from_u64(7);
        assert_eq!(generate_level(&mut a).grid, generate_level(&mut b).grid);
    }
}
