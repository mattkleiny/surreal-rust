use std::ops::{Add, Sub};

use crate::maths::{Lerp, Random, RandomGenerator};

/// A simple 32 bit color value with 4 channels (RGBA).
#[derive(Copy, Clone, Default, Eq, PartialEq, PartialOrd, Debug)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Color {
  pub const WHITE: Color = Color::rgb(255, 255, 255);
  pub const BLACK: Color = Color::rgb(0, 0, 0);
  pub const RED: Color = Color::rgb(255, 0, 0);
  pub const GREEN: Color = Color::rgb(0, 255, 0);
  pub const BLUE: Color = Color::rgb(0, 0, 255);

  #[inline]
  pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b, a: 255 }
  }

  #[inline]
  pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }
}

impl Into<(u8, u8, u8, u8)> for Color {
  fn into(self) -> (u8, u8, u8, u8) {
    (self.r, self.g, self.b, self.a)
  }
}

impl Into<[f32; 4]> for Color {
  fn into(self) -> [f32; 4] {
    [
      self.r as f32 / 255.0,
      self.g as f32 / 255.0,
      self.b as f32 / 255.0,
      self.a as f32 / 255.0
    ]
  }
}

impl From<[u8; 4]> for Color {
  fn from(source: [u8; 4]) -> Self {
    Self::rgba(source[0], source[1], source[2], source[3])
  }
}

impl Into<u32> for Color {
  fn into(self) -> u32 {
    ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32) as u32
  }
}

impl From<u32> for Color {
  fn from(packed: u32) -> Self {
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
  #[inline]
  fn lerp(a: Color, b: Color, t: f32) -> Self {
    Color::rgba(
      u8::lerp(a.r, b.r, t),
      u8::lerp(a.g, b.g, t),
      u8::lerp(a.b, b.b, t),
      u8::lerp(a.a, b.a, t),
    )
  }
}

impl Random for Color {
  fn random(generator: &mut RandomGenerator) -> Self {
    Color::rgba(
      generator.next::<u8>(),
      generator.next::<u8>(),
      generator.next::<u8>(),
      generator.next::<u8>(),
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::Seed;

  use super::*;

  #[test]
  fn color_should_generate_random_values() {
    let seed = Seed::random();
    let mut rng = seed.to_random();

    let color1: Color = rng.next();
    let color2: Color = rng.next();
    let color3: Color = rng.next();
    let color4: Color = rng.next();

    assert_ne!(color1, color2);
    assert_ne!(color2, color3);
    assert_ne!(color3, color4);
  }

  #[test]
  fn color_should_interpolate_between_values() {
    let color = Color::lerp(Color::BLACK, Color::WHITE, 0.5);

    assert_eq!(color.r, 127);
    assert_eq!(color.g, 127);
    assert_eq!(color.b, 127);
    assert_eq!(color.a, 255);
  }
}