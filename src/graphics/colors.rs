//! Pixel types and abstractions for color manipulation.
//!
//! We support the two most common color types, a 32-bit integral RGBA color,
//! and a 32-bit floating point per-chanenl representation for more precise rendering.

use std::ops::{Add, Div, Mul, Sub};

use crate::maths::{ApproxEq, FromRandom, Lerp, Numeric, Random};

/// Represents a type of pixel.
pub trait Pixel: Copy + Default {
  /// The scalar type that is used to store each channel in this pixel.
  type Subpixel: Numeric;

  /// The number of channels in this pixel type.
  const CHANNEL_COUNT: usize;

  /// A representation of an empty color in this pixel type.
  const EMPTY: Self;

  /// Converts this pixel type from a raw slice of 0-255 byte values.
  fn from_bytes(slice: &[u8; 4]) -> Self;
}

/// A simple floating point color value with 4 channels (RGBA).
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Color {
  pub const CLEAR: Self = Self::rgba(0., 0., 0., 0.);
  pub const WHITE: Self = Self::rgb(1., 1., 1.);
  pub const BLACK: Self = Self::rgb(0., 0., 0.);
  pub const RED: Self = Self::rgb(1., 0., 0.);
  pub const GREEN: Self = Self::rgb(0., 1., 0.);
  pub const BLUE: Self = Self::rgb(0., 0., 1.);
  pub const MAGENTA: Self = Self::rgb(1., 0., 1.);

  /// Creates a new color with the given components.
  pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
    Self::rgba(r, g, b, 1.)
  }

  /// Creates a new color with the given components.
  pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self { r, g, b, a }
  }
}

impl Pixel for Color {
  type Subpixel = f32;

  const CHANNEL_COUNT: usize = 4;
  const EMPTY: Self = Self::CLEAR;

  fn from_bytes(slice: &[u8; 4]) -> Self {
    Self {
      r: slice[0] as f32 / 255.,
      g: slice[1] as f32 / 255.,
      b: slice[2] as f32 / 255.,
      a: slice[3] as f32 / 255.,
    }
  }
}

impl From<Color32> for Color {
  fn from(color: Color32) -> Self {
    Self::rgba(
      color.r as f32 / 255.0,
      color.g as f32 / 255.0,
      color.r as f32 / 255.0,
      color.a as f32 / 255.0,
    )
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    self.r.approx_eq(other.r) && self.g.approx_eq(other.g) && self.b.approx_eq(other.b) && self.a.approx_eq(other.a)
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

impl Mul for Color {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      r: self.r * rhs.r,
      g: self.g * rhs.g,
      b: self.b * rhs.b,
      a: self.a * rhs.a,
    }
  }
}

impl Mul<f32> for Color {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Self {
      r: self.r * rhs,
      g: self.g * rhs,
      b: self.b * rhs,
      a: self.a * rhs,
    }
  }
}

impl Div for Color {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    Self {
      r: self.r / rhs.r,
      g: self.g / rhs.g,
      b: self.b / rhs.b,
      a: self.a / rhs.a,
    }
  }
}

impl Div<f32> for Color {
  type Output = Self;

  fn div(self, rhs: f32) -> Self::Output {
    Self {
      r: self.r / rhs,
      g: self.g / rhs,
      b: self.b / rhs,
      a: self.a / rhs,
    }
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
    Color::rgb(random.next(), random.next(), random.next())
  }
}

/// A simple 32-bit color value with 4 channels (RGBA).
#[repr(C)]
#[derive(Copy, Clone, Default, Debug, Eq)]
pub struct Color32 {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Color32 {
  pub const CLEAR: Self = Self::rgba(0, 0, 0, 0);
  pub const WHITE: Self = Self::rgb(255, 255, 255);
  pub const BLACK: Self = Self::rgb(0, 0, 0);
  pub const RED: Self = Self::rgb(255, 0, 0);
  pub const GREEN: Self = Self::rgb(0, 255, 0);
  pub const BLUE: Self = Self::rgb(0, 0, 255);
  pub const YELLOW: Self = Self::rgb(255, 255, 0);
  pub const MAGENTA: Self = Self::rgb(255, 0, 255);
  pub const CYAN: Self = Self::rgb(0, 255, 255);

  /// Creates a new color with the given components.
  pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self::rgba(r, g, b, 255)
  }

  /// Creates a new color with the given components.
  pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }
}

impl Pixel for Color32 {
  type Subpixel = u8;

  const CHANNEL_COUNT: usize = 4;
  const EMPTY: Self = Self::CLEAR;

  fn from_bytes(slice: &[u8; 4]) -> Self {
    Self {
      r: slice[0],
      g: slice[1],
      b: slice[2],
      a: slice[3],
    }
  }
}

impl From<Color> for Color32 {
  fn from(color: Color) -> Self {
    Self::rgba(
      (color.r * 255.0) as u8,
      (color.g * 255.0) as u8,
      (color.b * 255.0) as u8,
      (color.a * 255.0) as u8,
    )
  }
}

impl PartialEq for Color32 {
  fn eq(&self, other: &Self) -> bool {
    self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
  }
}

impl Add for Color32 {
  type Output = Color32;

  fn add(self, rhs: Self) -> Self::Output {
    Color32::rgba(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b, self.a + rhs.a)
  }
}

impl Sub for Color32 {
  type Output = Color32;

  fn sub(self, rhs: Self) -> Self::Output {
    Color32::rgba(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b, self.a - rhs.a)
  }
}

impl Mul for Color32 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      r: self.r * rhs.r,
      g: self.g * rhs.g,
      b: self.b * rhs.b,
      a: self.a * rhs.a,
    }
  }
}

impl Mul<u8> for Color32 {
  type Output = Self;

  fn mul(self, rhs: u8) -> Self::Output {
    Self {
      r: self.r * rhs,
      g: self.g * rhs,
      b: self.b * rhs,
      a: self.a * rhs,
    }
  }
}

impl Div for Color32 {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    Self {
      r: self.r / rhs.r,
      g: self.g / rhs.g,
      b: self.b / rhs.b,
      a: self.a / rhs.a,
    }
  }
}

impl Div<u8> for Color32 {
  type Output = Self;

  fn div(self, rhs: u8) -> Self::Output {
    Self {
      r: self.r / rhs,
      g: self.g / rhs,
      b: self.b / rhs,
      a: self.a / rhs,
    }
  }
}

impl Lerp for Color32 {
  fn lerp(a: Color32, b: Color32, t: f32) -> Self {
    Color32::rgba(
      u8::lerp(a.r, b.r, t),
      u8::lerp(a.g, b.g, t),
      u8::lerp(a.b, b.b, t),
      u8::lerp(a.a, b.a, t),
    )
  }
}

impl FromRandom for Color32 {
  fn from_random(random: &mut Random) -> Self {
    Color32::rgb(random.next(), random.next(), random.next())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
  fn color_should_lerp_between_values() {
    let color = Color::lerp(Color::BLACK, Color::WHITE, 0.5);

    assert_eq!(color.r, 0.5);
    assert_eq!(color.g, 0.5);
    assert_eq!(color.b, 0.5);
    assert_eq!(color.a, 1.);
  }

  #[test]
  fn color_should_create_red_green_blue_tuples() {
    let color = Color::rgb(-0.5, 0.4, 1.7);

    assert_eq!(color.r, -0.5);
    assert_eq!(color.g, 0.4);
    assert_eq!(color.b, 1.7);
  }

  #[test]
  fn colors_should_add() {
    let a = Color::rgba(0.9, 0.6, 0.75, 1.);
    let b = Color::rgba(0.7, 0.1, 0.25, 1.);

    assert_eq!(a + b, Color::rgba(1.6, 0.7, 1.0, 2.));
  }

  #[test]
  fn colors_should_subtract() {
    let a = Color::rgba(0.9, 0.6, 0.75, 2.);
    let b = Color::rgba(0.7, 0.1, 0.25, 1.);

    assert_eq!(a - b, Color::rgba(0.2, 0.5, 0.5, 1.));
  }

  #[test]
  fn colors_should_multiply() {
    let a = Color::rgb(1., 0.2, 0.4);
    let b = Color::rgb(0.9, 1., 0.1);

    assert_eq!(a * b, Color::rgb(0.9, 0.2, 0.04));
  }

  #[test]
  fn colors_should_multiply_by_scalar() {
    let a = Color::rgba(0.2, 0.3, 0.4, 1.);

    assert_eq!(a * 2., Color::rgba(0.4, 0.6, 0.8, 2.));
  }
}
