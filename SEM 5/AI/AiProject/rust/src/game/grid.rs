use super::cell::Cell;
use super::config::{COLS, ROWS};

/// Maze tile map. `true` means walkable floor, `false` means wall.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    cols: i32,
    rows: i32,
    floor: Vec<bool>,
}

impl Grid {
    pub fn new(cols: i32, rows: i32) -> Self {
        Self {
            cols,
            rows,
            floor: vec![false; (cols * rows) as usize],
        }
    }

    pub fn with_size() -> Self {
        Self::new(COLS, ROWS)
    }

    pub const fn cols(&self) -> i32 {
        self.cols
    }

    pub const fn rows(&self) -> i32 {
        self.rows
    }

    fn index(&self, cell: Cell) -> usize {
        (cell.y * self.cols + cell.x) as usize
    }

    pub fn in_bounds(&self, cell: Cell) -> bool {
        cell.in_bounds(self.cols, self.rows)
    }

    /// `true` when the cell is walkable floor.
    pub fn is_floor(&self, cell: Cell) -> bool {
        self.in_bounds(cell) && self.floor[self.index(cell)]
    }

    /// `true` when the cell is a wall (or outside the map).
    pub fn is_wall(&self, cell: Cell) -> bool {
        !self.is_floor(cell)
    }

    pub fn set_floor(&mut self, cell: Cell, floor: bool) {
        if self.in_bounds(cell) {
            let index = self.index(cell);
            self.floor[index] = floor;
        }
    }

    pub fn cells(&self) -> impl Iterator<Item = Cell> + '_ {
        (0..self.rows).flat_map(move |y| (0..self.cols).map(move |x| Cell::new(x, y)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid_is_all_walls() {
        let grid = Grid::new(4, 3);
        assert!(grid.is_wall(Cell::new(0, 0)));
        assert!(grid.is_wall(Cell::new(3, 2)));
        assert!(!grid.is_floor(Cell::new(3, 2)));
    }

    #[test]
    fn out_of_bounds_is_wall() {
        let grid = Grid::new(4, 3);
        assert!(grid.is_wall(Cell::new(-1, 0)));
        assert!(grid.is_wall(Cell::new(4, 0)));
    }

    #[test]
    fn set_and_read_floor() {
        let mut grid = Grid::new(4, 3);
        grid.set_floor(Cell::new(1, 2), true);
        assert!(grid.is_floor(Cell::new(1, 2)));
        assert!(grid.is_wall(Cell::new(2, 2)));
    }
}
