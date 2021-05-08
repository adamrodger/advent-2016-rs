#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// From a given direction, turn left or right to a new direction
    pub fn turn(&self, turn: &Turn) -> Self {
        match turn {
            Turn::Left => match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
            Turn::Right => match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0, 0)
    }
}

impl Point {
    /// Create a new point
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Move one step in the given direction
    pub fn move_direction(&self, direction: &Direction) -> Point {
        self.move_direction_steps(direction, 1)
    }

    /// Move one or more steps in the given direction
    pub fn move_direction_steps(&self, direction: &Direction, steps: i32) -> Point {
        match direction {
            Direction::North => Point {
                x: self.x,
                y: self.y + steps,
            },
            Direction::South => Point {
                x: self.x,
                y: self.y - steps,
            },
            Direction::East => Point {
                x: self.x + steps,
                y: self.y,
            },
            Direction::West => Point {
                x: self.x - steps,
                y: self.y,
            },
        }
    }

    /// Get the 8 directional neighbours of a point
    pub fn _neighbours(&self) -> [Point; 8] {
        [
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn() {
        assert_eq!(Direction::North.turn(&Turn::Left), Direction::West);
        assert_eq!(Direction::North.turn(&Turn::Right), Direction::East);

        assert_eq!(Direction::South.turn(&Turn::Left), Direction::East);
        assert_eq!(Direction::South.turn(&Turn::Right), Direction::West);

        assert_eq!(Direction::East.turn(&Turn::Left), Direction::North);
        assert_eq!(Direction::East.turn(&Turn::Right), Direction::South);

        assert_eq!(Direction::West.turn(&Turn::Left), Direction::South);
        assert_eq!(Direction::West.turn(&Turn::Right), Direction::North);
    }

    #[test]
    fn test_move_direction() {
        // back to original position
        assert_eq!(
            Point::default()
                .move_direction(&Direction::North)
                .move_direction(&Direction::South)
                .move_direction(&Direction::East)
                .move_direction(&Direction::West),
            Point::default()
        );
    }

    #[test]
    fn test_move_direction_steps() {
        // back to original position
        assert_eq!(
            Point::default()
                .move_direction_steps(&Direction::North, 2)
                .move_direction_steps(&Direction::South, 2)
                .move_direction_steps(&Direction::East, 2)
                .move_direction_steps(&Direction::West, 2),
            Point::default()
        );
    }
}
