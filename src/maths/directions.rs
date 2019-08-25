//! Directions in 2-space.

use enumflags2::BitFlags;

use super::*;

/// One or more cardinal directions.
pub type Directions = BitFlags<Direction>;

/// Represents a cardinal direction in 2-space.
#[repr(u8)]
#[derive(EnumFlags, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
  North = 1 << 0,
  East = 1 << 1,
  South = 1 << 2,
  West = 1 << 3,
}

impl Direction {
  pub fn opposite(&self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::East => Direction::West,
      Direction::South => Direction::North,
      Direction::West => Direction::East,
    }
  }
}

/// Extensions for doing conversion to/from directions.
pub trait Conversions {
  fn to_direction(&self) -> Direction;
  fn to_directions(&self) -> Directions;
  fn from_direction(direction: Direction) -> Self;
  fn from_directions(direction: Directions) -> Self;
}

/// Direction conversion for integral vectors.
impl Conversions for Vec2i {
  fn to_direction(&self) -> Direction {
    let horizontal = self.x.abs();
    let vertical = self.y.abs();

    if horizontal > vertical {
      if self.x > 0 { return Direction::East; }
      if self.x < 0 { return Direction::West; }
    } else {
      if self.y > 0 { return Direction::North; }
      if self.y < 0 { return Direction::South; }
    }

    Direction::North
  }

  fn to_directions(&self) -> BitFlags<Direction> {
    let mut result = Directions::empty();

    if self.x > 0 { result |= Direction::East; }
    if self.x < 0 { result |= Direction::West; }
    if self.y > 0 { result |= Direction::North; }
    if self.y < 0 { result |= Direction::South; }

    result
  }

  fn from_direction(direction: Direction) -> Self {
    match direction {
      Direction::North => Vec2i::unit_y(),
      Direction::East => Vec2i::unit_x(),
      Direction::South => -Vec2i::unit_y(),
      Direction::West => -Vec2i::unit_x(),
    }
  }

  fn from_directions(direction: BitFlags<Direction>) -> Self {
    let mut x = 0;
    let mut y = 0;

    if direction.contains(Direction::North) { y += 1; }
    if direction.contains(Direction::East) { x += 1; }
    if direction.contains(Direction::South) { y -= 1; }
    if direction.contains(Direction::West) { x -= 1; }

    Vec2i::new(x, y)
  }
}

/// Direction conversion for standard vectors.
impl Conversions for Vec2 {
  fn to_direction(&self) -> Direction {
    let horizontal = self.x().abs();
    let vertical = self.y().abs();

    if horizontal > vertical {
      if self.x() > 0. { return Direction::East; }
      if self.x() < 0. { return Direction::West; }
    } else {
      if self.y() > 0. { return Direction::North; }
      if self.y() < 0. { return Direction::South; }
    }

    Direction::North
  }

  fn to_directions(&self) -> BitFlags<Direction> {
    let mut result = Directions::empty();

    if self.x() > 0. { result |= Direction::East; }
    if self.x() < 0. { result |= Direction::West; }
    if self.y() > 0. { result |= Direction::North; }
    if self.y() < 0. { result |= Direction::South; }

    result
  }

  fn from_direction(direction: Direction) -> Self {
    match direction {
      Direction::North => Vec2::unit_y(),
      Direction::East => Vec2::unit_x(),
      Direction::South => -Vec2::unit_y(),
      Direction::West => -Vec2::unit_x(),
    }
  }

  fn from_directions(direction: BitFlags<Direction>) -> Self {
    let mut x = 0.;
    let mut y = 0.;

    if direction.contains(Direction::North) { y += 1.; }
    if direction.contains(Direction::East) { x += 1.; }
    if direction.contains(Direction::South) { y -= 1.; }
    if direction.contains(Direction::West) { x -= 1.; }

    Vec2::new(x, y)
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::Vec2i;

  use super::*;

  #[test]
  fn it_should_produce_a_valid_mask() {
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

  #[test]
  fn it_should_compute_direction_from_vec2() {
    assert_eq!(Direction::North, Vec2::new(0., 1.).to_direction());
    assert_eq!(Direction::East, Vec2::new(1., 0.).to_direction());
    assert_eq!(Direction::South, Vec2::new(0., -1.).to_direction());
    assert_eq!(Direction::West, Vec2::new(-1., 0.).to_direction());
  }

  #[test]
  fn it_should_compute_direction_from_vec2i() {
    assert_eq!(Direction::North, Vec2i::new(0, 1).to_direction());
    assert_eq!(Direction::East, Vec2i::new(1, 0).to_direction());
    assert_eq!(Direction::South, Vec2i::new(0, -1).to_direction());
    assert_eq!(Direction::West, Vec2i::new(-1, 0).to_direction());
  }

  #[test]
  fn it_should_compute_directions_from_vec2i() {
    assert_eq!(Direction::North | Direction::East, Vec2i::new(1, 1).to_directions());
    assert_eq!(Direction::South | Direction::West, Vec2i::new(-1, -1).to_directions());
  }

  #[test]
  fn it_should_compute_vec2i_from_direction() {
    assert_eq!(Vec2i::from_direction(Direction::North), Vec2i::new(0, 1));
    assert_eq!(Vec2i::from_direction(Direction::East), Vec2i::new(1, 0));
    assert_eq!(Vec2i::from_direction(Direction::South), Vec2i::new(0, -1));
    assert_eq!(Vec2i::from_direction(Direction::West), Vec2i::new(-1, 0));
  }

  #[test]
  fn it_should_compute_vec2i_from_directions() {
    assert_eq!(Vec2i::from_directions(Direction::North | Direction::East), Vec2i::new(1, 1));
    assert_eq!(Vec2i::from_directions(Direction::South | Direction::West), Vec2i::new(-1, -1));
  }
}