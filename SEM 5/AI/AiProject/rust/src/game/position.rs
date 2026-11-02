use super::cell::Cell;

/// Continuous position in grid space: `(0, 0)` is the top-left corner of the
/// maze and one unit is one cell. `y` grows downwards like the maze grid.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(self, other: Position) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Grid cell containing this position.
    pub fn cell(self) -> Cell {
        Cell::new(self.x.floor() as i32, self.y.floor() as i32)
    }

    /// Move at most `step` towards `target`; returns the remaining step.
    pub fn move_towards(&mut self, target: Position, step: f32) -> f32 {
        let dx = target.x - self.x;
        let dy = target.y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist <= step || dist < f32::EPSILON {
            *self = target;
            0.0
        } else {
            let scale = step / dist;
            self.x += dx * scale;
            self.y += dy * scale;
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_maps_to_cell() {
        assert_eq!(Position::new(0.0, 0.0).cell(), Cell::new(0, 0));
        assert_eq!(Position::new(1.99, 2.01).cell(), Cell::new(1, 2));
    }

    #[test]
    fn distance_is_euclidean() {
        let a = Position::new(0.0, 0.0);
        let b = Position::new(3.0, 4.0);
        assert_eq!(a.distance_to(b), 5.0);
    }
}
