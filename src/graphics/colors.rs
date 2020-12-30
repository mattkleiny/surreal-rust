use std::ops::{Add, Sub};

use crate::maths::{Lerp, Random, Rng};

/// A simple 32 bit color value with 4 channels (RGBA).
#[derive(Copy, Clone, Default, Eq, PartialEq, PartialOrd, Debug)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Color {
  pub const CLEAR: Color = Self::rgba(0,0,0,0);
  pub const WHITE: Color = Self::rgb(255, 255, 255);
  pub const BLACK: Color = Self::rgb(0, 0, 0);
  pub const RED: Color = Self::rgb(255, 0, 0);
  pub const GREEN: Color = Self::rgb(0, 255, 0);
  pub const BLUE: Color = Self::rgb(0, 0, 255);
  pub const PINK: Color = Self::rgb(255, 0, 255);

  #[inline]
  pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self::rgba(r, g, b, 255)
  }

  #[inline]
  pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }

  pub fn to_packed_rgba(&self) -> u32 {
    ((self.r as u32) << 24)
        | ((self.g as u32) << 16)
        | ((self.b as u32) << 8)
        | (self.a as u32) as u32
  }

  pub fn from_packed_rgba(packed: u32) -> Self {
    Self::rgba(
      (packed >> 24 & 0xFF) as u8,
      (packed >> 16 & 0xFF) as u8,
      (packed >> 8 & 0xFF) as u8,
      (packed >> 0 & 0xFF) as u8,
    )
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
  fn lerp(a: &Color, b: &Color, t: f32) -> Self {
    Color::rgba(
      u8::lerp(&a.r, &b.r, t),
      u8::lerp(&a.g, &b.g, t),
      u8::lerp(&a.b, &b.b, t),
      u8::lerp(&a.a, &b.a, t),
    )
  }
}

impl Random for Color {
  fn generate(gen: &mut Rng) -> Self {
    Color::rgba(gen.next(), gen.next(), gen.next(), gen.next())
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::Seed;

  use super::*;

  #[test]
  fn color_should_interpolate_between_values() {
    let color = Color::lerp(&Color::BLACK, &Color::WHITE, 0.5);

    assert_eq!(color.r, 127);
    assert_eq!(color.g, 127);
    assert_eq!(color.b, 127);
    assert_eq!(color.a, 255);
  }

  #[test]
  fn color_should_generate_random_values() {
    let seed = Seed::random();
    let mut rng = seed.to_rng();

    let color1: Color = rng.next();
    let color2: Color = rng.next();
    let color3: Color = rng.next();
    let color4: Color = rng.next();

    assert_ne!(color1, color2);
    assert_ne!(color2, color3);
    assert_ne!(color3, color4);
  }
}
