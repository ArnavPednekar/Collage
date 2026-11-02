use super::position::Position;

/// Integer maze coordinate. `y` grows downwards, matching the maze grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

/// Four-connected neighbourhood, ordered like the original prototype.
pub const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Cell {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn in_bounds(self, cols: i32, rows: i32) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < cols && self.y < rows
    }

    /// Centre of the cell in continuous grid space (one unit per cell).
    pub fn center(self) -> Position {
        Position::new(self.x as f32 + 0.5, self.y as f32 + 0.5)
    }

    pub fn neighbors(self) -> impl Iterator<Item = Cell> {
        DIRS.into_iter()
            .map(move |(dx, dy)| Cell::new(self.x + dx, self.y + dy))
    }

    pub fn manhattan(self, other: Cell) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centers_are_half_offsets() {
        assert_eq!(Cell::new(0, 0).center(), Position::new(0.5, 0.5));
        assert_eq!(Cell::new(4, 7).center(), Position::new(4.5, 7.5));
    }

    #[test]
    fn neighbours_are_four_connected() {
        let cells: Vec<Cell> = Cell::new(3, 3).neighbors().collect();
        assert_eq!(
            cells,
            vec![
                Cell::new(4, 3),
                Cell::new(2, 3),
                Cell::new(3, 4),
                Cell::new(3, 2)
            ]
        );
    }
}
