//! Pixel types and abstractions for color manipulation.

use std::ops::{Add, Sub};

use crate::maths::{ApproxEq, FromRandom, Lerp, Numeric, Random};

/// Represents a type of pixel.
pub trait Pixel: Copy + Clone {
  /// The scalar type that is used to store each channel in this pixel.
  type Subpixel: Numeric;

  /// The number of channels in this pixel type.
  const CHANNEL_COUNT: usize;

  /// Constructs this pixel type from raw subpixel values.
  fn from_slice(slice: &[Self::Subpixel]) -> Self;

  /// Gets an array of the pixel's values.
  fn channels(&self) -> [Self::Subpixel; Self::CHANNEL_COUNT];
}

/// A simple floating point color value with 4 channels (RGBA).
#[derive(Copy, Clone, Default, Debug)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Color {
  pub const CLEAR: Color = Self::rgba(0., 0., 0., 0.);
  pub const WHITE: Color = Self::rgb(1., 1., 1.);
  pub const BLACK: Color = Self::rgb(0., 0., 0.);
  pub const RED: Color = Self::rgb(1., 0., 0.);
  pub const GREEN: Color = Self::rgb(0., 1., 0.);
  pub const BLUE: Color = Self::rgb(0., 0., 1.);
  pub const MAGENTA: Color = Self::rgb(1., 0., 1.);

  pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
    Self::rgba(r, g, b, 1.)
  }

  pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self { r, g, b, a }
  }
}

impl Pixel for Color {
  type Subpixel = f32;

  const CHANNEL_COUNT: usize = 4;

  fn from_slice(slice: &[Self::Subpixel]) -> Self {
    Self { r: slice[0], g: slice[1], b: slice[2], a: slice[3] }
  }

  fn channels(&self) -> [Self::Subpixel; Self::CHANNEL_COUNT] {
    [self.r, self.g, self.b, self.a]
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    self.r.approx_eq(other.r) &&
      self.g.approx_eq(other.g) &&
      self.b.approx_eq(other.b) &&
      self.a.approx_eq(other.a)
  }
}

impl Add for Color {
  type Output = Color;

  fn add(self, rhs: Self) -> Self::Output {
    Color::rgba(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a + rhs.a)
  }
}

impl Sub for Color {
  type Output = Color;

  fn sub(self, rhs: Self) -> Self::Output {
    Color::rgba(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b, self.a - rhs.a)
  }
}

impl Lerp for Color {
  fn lerp(a: Color, b: Color, t: f32) -> Self {
    Color::rgba(
      f32::lerp(a.r, b.r, t),
      f32::lerp(a.g, b.g, t),
      f32::lerp(a.b, b.b, t),
      f32::lerp(a.a, b.a, t),
    )
  }
}

impl FromRandom for Color {
  fn from_random(random: &mut Random) -> Self {
    Color::rgb(
      random.next(),
      random.next(),
      random.next(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn color_should_be_convertible_from_slice() {
    let color = Color::from_slice(&[1., 1., 0., 0.]);

    assert_eq!(color.r, 1.);
    assert_eq!(color.g, 1.);
    assert_eq!(color.b, 0.);
    assert_eq!(color.a, 0.);
  }

  #[test]
  fn color_should_be_convertible_to_channels() {
    let channels = Color::RED.channels();

    assert_eq!(channels[0], 1.);
    assert_eq!(channels[1], 0.);
    assert_eq!(channels[2], 0.);
    assert_eq!(channels[3], 1.);
  }

  #[test]
  fn color_should_be_equatable() {
    let color1 = Color::WHITE;
    let color2 = Color::WHITE;

    assert_eq!(color1, color2);
  }

  #[test]
  fn color_should_be_randomly_creatable() {
    let color1 = Color::random();
    let color2 = Color::random();

    assert_ne!(color1, color2);
  }

  #[test]
  fn color_should_interpolate_between_values() {
    let color = Color::lerp(Color::BLACK, Color::WHITE, 0.5);

    assert_eq!(color.r, 0.5);
    assert_eq!(color.g, 0.5);
    assert_eq!(color.b, 0.5);
    assert_eq!(color.a, 1.);
  }
}
