/// A set of directions.
pub type DirectionSet = enumflags2::BitFlags<Direction>;

/// Represents a cardinal direction in 2-space.
#[repr(u8)]
#[derive(BitFlags, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
  North = 1 << 0,
  East = 1 << 1,
  South = 1 << 2,
  West = 1 << 3,
}

impl Direction {
  #[inline]
  pub fn all() -> DirectionSet {
    Direction::North | Direction::South | Direction::East | Direction::West
  }

  #[inline]
  pub fn none() -> DirectionSet {
    DirectionSet::empty()
  }

  pub fn opposite(&self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::East => Direction::West,
      Direction::South => Direction::North,
      Direction::West => Direction::East,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_produce_a_valid_set() {
    let ns = Direction::North | Direction::South;
    let ew = Direction::East | Direction::West;

    assert_ne!(ns, ew);

    assert!(ns.contains(Direction::North));
    assert!(!ns.contains(Direction::East));
    assert!(ns.contains(Direction::South));
    assert!(!ns.contains(Direction::West));

    assert!(!ew.contains(Direction::North));
    assert!(ew.contains(Direction::East));
    assert!(!ew.contains(Direction::South));
    assert!(ew.contains(Direction::West));
  }
}