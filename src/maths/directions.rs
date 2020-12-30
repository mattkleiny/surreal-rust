/// Represents an axis (horizontal or vertical).
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Axis {
  Horizontal = 1 << 0,
  Vertical = 1 << 1,
}

/// Represents a cardinal direction in 2-space.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
  North = 1 << 0,
  East = 1 << 1,
  South = 1 << 2,
  West = 1 << 3,
}

impl Direction {
  /// Returns the opposite direction.
  pub fn opposite(&self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::East => Direction::West,
      Direction::South => Direction::North,
      Direction::West => Direction::East,
    }
  }
}
