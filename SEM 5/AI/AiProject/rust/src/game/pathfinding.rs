//! A* pathfinding over the tile grid.
//!
//! The grid is 4-connected with uniform step cost, so Manhattan distance is an
//! admissible heuristic. Towers are added to the blocked set, which forces
//! enemies to recompute their route every time the player builds or sells.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use super::cell::Cell;
use super::config::{COLS, ROWS};
use super::grid::Grid;

/// Static walkability mask: walls are blocked, towers are blocked later.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Walkability {
    cols: i32,
    rows: i32,
    blocked: Vec<bool>,
}

impl Walkability {
    pub fn from_grid(grid: &Grid) -> Self {
        let mut blocked = Vec::with_capacity((grid.cols() * grid.rows()) as usize);
        for cell in grid.cells() {
            blocked.push(grid.is_wall(cell));
        }
        Self {
            cols: grid.cols(),
            rows: grid.rows(),
            blocked,
        }
    }

    pub fn empty() -> Self {
        Self {
            cols: COLS,
            rows: ROWS,
            blocked: vec![false; (COLS * ROWS) as usize],
        }
    }

    fn index(&self, cell: Cell) -> usize {
        (cell.y * self.cols + cell.x) as usize
    }

    pub fn in_bounds(&self, cell: Cell) -> bool {
        cell.in_bounds(self.cols, self.rows)
    }

    pub fn is_blocked(&self, cell: Cell) -> bool {
        !self.in_bounds(cell) || self.blocked[self.index(cell)]
    }

    /// Mark or unmark a cell as blocked. Ignored outside the map.
    pub fn set_blocked(&mut self, cell: Cell, blocked: bool) {
        if self.in_bounds(cell) {
            let index = self.index(cell);
            self.blocked[index] = blocked;
        }
    }
}

fn heuristic(a: Cell, b: Cell) -> u32 {
    a.manhattan(b)
}

/// Shortest path from `start` to `goal`, inclusive, or `None` if unreachable.
pub fn astar(start: Cell, goal: Cell, nav: &Walkability) -> Option<Vec<Cell>> {
    if start == goal {
        return Some(vec![start]);
    }
    if nav.is_blocked(start) || nav.is_blocked(goal) {
        return None;
    }

    let size = (nav.cols * nav.rows) as usize;
    let mut cost = vec![u32::MAX; size];
    let mut came_from: Vec<Option<Cell>> = vec![None; size];
    let mut closed = vec![false; size];
    let mut frontier = BinaryHeap::new();
    let mut sequence = 0u32;

    cost[nav.index(start)] = 0;
    frontier.push(Reverse((heuristic(start, goal), 0, 0, start.x, start.y)));

    while let Some(Reverse((_, g, _, x, y))) = frontier.pop() {
        let current = Cell::new(x, y);
        let current_index = nav.index(current);
        if closed[current_index] {
            continue;
        }
        if current == goal {
            let mut path = vec![current];
            let mut cursor = current;
            while let Some(previous) = came_from[nav.index(cursor)] {
                path.push(previous);
                cursor = previous;
            }
            path.reverse();
            return Some(path);
        }

        closed[current_index] = true;
        for next in current.neighbors() {
            if nav.is_blocked(next) || closed[nav.index(next)] {
                continue;
            }
            let next_cost = g + 1;
            if next_cost < cost[nav.index(next)] {
                cost[nav.index(next)] = next_cost;
                came_from[nav.index(next)] = Some(current);
                sequence = sequence.wrapping_add(1);
                frontier.push(Reverse((
                    next_cost + heuristic(next, goal),
                    next_cost,
                    sequence,
                    next.x,
                    next.y,
                )));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::config::{COLS, ROWS};

    fn open_nav() -> Walkability {
        Walkability::empty()
    }

    #[test]
    fn open_grid_uses_manhattan_distance() {
        let nav = open_nav();
        let path = astar(Cell::new(0, 0), Cell::new(5, 3), &nav).unwrap();
        assert_eq!(
            path.len() as u32,
            1 + Cell::new(0, 0).manhattan(Cell::new(5, 3))
        );
        assert_eq!(path.first(), Some(&Cell::new(0, 0)));
        assert_eq!(path.last(), Some(&Cell::new(5, 3)));
    }

    #[test]
    fn path_is_four_connected() {
        let nav = open_nav();
        let path = astar(Cell::new(2, 2), Cell::new(8, 9), &nav).unwrap();
        for pair in path.windows(2) {
            assert_eq!(pair[0].manhattan(pair[1]), 1);
        }
    }

    #[test]
    fn walls_force_a_detour() {
        let mut nav = open_nav();
        for y in 0..ROWS - 1 {
            nav.set_blocked(Cell::new(3, y), true);
        }
        let start = Cell::new(1, 1);
        let goal = Cell::new(5, 1);
        let path = astar(start, goal, &nav).unwrap();
        assert!(path.len() > start.manhattan(goal) as usize + 1);
        assert!(path.iter().all(|&cell| cell != Cell::new(3, 1)));
    }

    #[test]
    fn sealed_goal_has_no_path() {
        let mut nav = open_nav();
        let goal = Cell::new(5, 5);
        for cell in goal.neighbors() {
            nav.set_blocked(cell, true);
        }
        assert!(astar(Cell::new(0, 0), goal, &nav).is_none());
    }

    #[test]
    fn blocked_start_or_goal_has_no_path() {
        let mut nav = open_nav();
        let start = Cell::new(1, 1);
        let goal = Cell::new(4, 4);
        nav.set_blocked(start, true);
        assert!(astar(start, goal, &nav).is_none());
    }

    #[test]
    fn start_equals_goal_returns_single_cell() {
        let nav = open_nav();
        let cell = Cell::new(2, 3);
        assert_eq!(astar(cell, cell, &nav), Some(vec![cell]));
    }

    #[test]
    fn symmetric_path_length() {
        let nav = open_nav();
        let a = Cell::new(1, 1);
        let b = Cell::new(COLS - 2, ROWS - 2);
        assert_eq!(
            astar(a, b, &nav).unwrap().len(),
            astar(b, a, &nav).unwrap().len()
        );
    }
}
