use super::{vec2, Vec2};
use crate::maths::{FromRandom, Lerp, Random};

/// Creates a new [`Hex`] from the given axial coordinates.
#[inline]
pub const fn hex(x: i32, y: i32) -> Hex {
  Hex::new(x, y)
}

/// Defines a position in a hexagonal grid.
#[repr(packed)]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Hex {
  /// X axial coordinate; (sometimes called q or i).
  pub x: i32,
  /// Y axial coordinate; (sometimes called r or j).
  pub y: i32,
}

impl Hex {
  pub const ORIGIN: Hex = hex(0, 0);
  pub const ZERO: Hex = hex(0, 0);
  pub const ONE: Hex = hex(1, 1);
  pub const NEG_ONE: Hex = hex(-1, -1);

  pub const X: Self = hex(1, 0);
  pub const Y: Self = hex(0, 1);
  pub const Z: Self = hex(0, -1);

  pub const NEG_X: Self = hex(-1, 0);
  pub const NEG_Y: Self = hex(0, -1);
  pub const NEG_Z: Self = hex(0, 1);

  /// Creates a new [`Hex`] from the given axial coordinates.
  #[inline]
  pub const fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  /// Creates a new [`Hex`] from the given axial coordinates splat in both X, Y.
  #[inline]
  pub const fn splat(value: i32) -> Self {
    Self { x: value, y: value }
  }

  /// `x` coordinate (sometimes called `q` or `i`)
  #[inline]
  #[doc(alias = "q")]
  pub const fn x(self) -> i32 {
    self.x
  }

  /// `y` coordinate (sometimes called `r` or `j`)
  #[inline]
  #[doc(alias = "r")]
  pub const fn y(self) -> i32 {
    self.y
  }

  /// `z` coordinate (sometimes called `s` or `k`).
  ///
  /// This cubic space coordinate is computed as `-x - y`
  #[inline]
  #[doc(alias = "s")]
  pub const fn z(self) -> i32 {
    -self.x - self.y
  }

  /// Creates a new [`Hex`] from the given array.
  #[inline]
  pub const fn from_array(array: [i32; 2]) -> Self {
    Self {
      x: array[0],
      y: array[1],
    }
  }

  /// Creates an array from the given [`Hex`].
  #[inline]
  pub const fn to_array(self) -> [i32; 2] {
    [self.x, self.y]
  }

  /// Converts this [`Hex`] into a [`Vec2`].
  #[inline]
  pub const fn as_vec2(self) -> Vec2 {
    vec2(self.x as f32, self.y as f32)
  }

  /// Returns the minimum value of the [`Hex`].
  pub fn min(self, other: Self) -> Self {
    Self {
      x: self.x.min(other.x),
      y: self.y.min(other.y),
    }
  }

  /// Returns the maximum value of the [`Hex`].
  pub fn max(self, other: Self) -> Self {
    Self {
      x: self.x.max(other.x),
      y: self.y.max(other.y),
    }
  }

  /// Clamps this [`Hex`] between the given minimum and maximum values.
  pub fn clamp(self, min: Self, max: Self) -> Self {
    self.max(min).min(max)
  }
}

impl Lerp for Hex {
  /// Interpolates between two [`Hex`] values by the given amount.
  fn lerp(a: Self, b: Self, t: f32) -> Self {
    let x = i32::lerp(a.x, b.x, t);
    let y = i32::lerp(a.y, b.y, t);

    Self::new(x, y)
  }
}

impl FromRandom for Hex {
  /// Creates a new [`Hex`] from the given [`Random`] instance.
  fn from_random(random: &mut Random) -> Self {
    Self::new(random.next(), random.next())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hex_coordinates() {
    let hex = Hex::new(3, 4);

    assert_eq!(hex.x(), 3);
    assert_eq!(hex.y(), 4);
    assert_eq!(hex.z(), -7);
  }

  #[test]
  fn test_hex_from_array() {
    let array = [2, 5];

    let hex = Hex::from_array(array);

    assert_eq!(hex.x(), 2);
    assert_eq!(hex.y(), 5);
    assert_eq!(hex.z(), -7);
  }

  #[test]
  fn test_hex_to_array() {
    let hex = Hex::new(3, 4);

    let array = hex.to_array();

    assert_eq!(array, [3, 4]);
  }

  #[test]
  fn test_hex_as_vec2() {
    let hex = Hex::new(3, 4);

    let vec2 = hex.as_vec2();

    assert_eq!(vec2.x, 3.0);
    assert_eq!(vec2.y, 4.0);
  }

  #[test]
  fn test_hex_min() {
    let hex1 = Hex::new(3, 4);
    let hex2 = Hex::new(2, 5);

    let min_hex = hex1.min(hex2);

    assert_eq!(min_hex.x(), 2);
    assert_eq!(min_hex.y(), 4);
    assert_eq!(min_hex.z(), -6);
  }

  #[test]
  fn test_hex_max() {
    let hex1 = Hex::new(3, 4);
    let hex2 = Hex::new(2, 5);

    let max_hex = hex1.max(hex2);

    assert_eq!(max_hex.x(), 3);
    assert_eq!(max_hex.y(), 5);
  }

  #[test]
  fn test_hex_clamp() {
    let hex = Hex::new(3, 4);
    let min_hex = Hex::new(2, 3);
    let max_hex = Hex::new(4, 5);

    let clamped_hex = hex.clamp(min_hex, max_hex);

    assert_eq!(clamped_hex.x(), 3);
    assert_eq!(clamped_hex.y(), 4);
    assert_eq!(clamped_hex.z(), -7);
  }

  #[test]
  fn test_hex_lerp() {
    let hex1 = Hex::new(0, 0);
    let hex2 = Hex::new(10, 10);
    let t = 0.5;

    let result = Hex::lerp(hex1, hex2, t);

    assert_eq!(result.x(), 5);
    assert_eq!(result.y(), 5);
    assert_eq!(result.z(), -10);
  }
}
