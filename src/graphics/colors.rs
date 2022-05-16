use std::ops::{Add, Sub};

use crate::maths::{FromRandom, Lerp, Random};

/// A simple 32 bit color value with 4 channels (RGBA).
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

  #[inline]
  pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
    Self::rgba(r, g, b, 1.)
  }

  #[inline]
  pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self { r, g, b, a }
  }
}

impl Add for Color {
  type Output = Color;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Color::rgba(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a + rhs.a)
  }
}

impl Sub for Color {
  type Output = Color;

  #[inline]
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
  fn color_should_interpolate_between_values() {
    let color = Color::lerp(Color::BLACK, Color::WHITE, 0.5);

    assert_eq!(color.r, 127);
    assert_eq!(color.g, 127);
    assert_eq!(color.b, 127);
    assert_eq!(color.a, 255);
  }
}
