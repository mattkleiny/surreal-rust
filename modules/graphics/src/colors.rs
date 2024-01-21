//! Pixel types and abstractions for color manipulation.
//!
//! We support the two most common color types, a 32-bit integral RGBA color,
//! and a 32-bit floating point per-channel representation for more precise
//! rendering.

use std::ops::{Div, Mul};

use common::{ApproxEq, FromRandom, Lerp, Random, Scalar};
use serde::{Deserialize, Serialize};

/// Represents a type of pixel.
pub trait Pixel: Copy + Default {
  /// The scalar type that is used to store each channel in this pixel.
  type Subpixel: Scalar;

  /// The number of channels in this pixel type.
  const CHANNEL_COUNT: usize;

  /// Converts this pixel type from a raw slice of 0-255 byte values.
  fn from_bytes(slice: &[u8; 4]) -> Self;
}

/// A simple floating point color value with 4 channels (RGBA).
#[repr(C)]
#[derive(Serialize, Deserialize, Copy, Clone, Default, Debug)]
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
  #[inline(always)]
  pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
    Self::rgba(r, g, b, 1.)
  }

  /// Creates a new color with the given components.
  #[inline(always)]
  pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self { r, g, b, a }
  }
}

impl Pixel for Color {
  type Subpixel = f32;

  const CHANNEL_COUNT: usize = 4;

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
  #[inline(always)]
  fn from_random(random: &mut Random) -> Self {
    Color::rgb(random.next(), random.next(), random.next())
  }
}

/// A simple 32-bit color value with 4 channels (RGBA).
#[repr(C)]
#[derive(Serialize, Deserialize, Copy, Clone, Default, Debug, Eq)]
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
  #[inline(always)]
  pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self::rgba(r, g, b, 255)
  }

  /// Creates a new color with the given components.
  #[inline(always)]
  pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }
}

impl Pixel for Color32 {
  type Subpixel = u8;

  const CHANNEL_COUNT: usize = 4;

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

/// Implements standard operations for a color type.
macro_rules! impl_std_ops {
  ($type:ty, $scalar:ty) => {
    impl std::ops::Add for $type {
      type Output = Self;

      #[inline]
      fn add(self, rhs: Self) -> Self::Output {
        Self {
          r: self.r + rhs.r,
          g: self.g + rhs.g,
          b: self.b + rhs.b,
          a: self.a + rhs.a,
        }
      }
    }

    impl std::ops::AddAssign for $type {
      #[inline]
      fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
      }
    }

    impl std::ops::Sub for $type {
      type Output = Self;

      #[inline]
      fn sub(self, rhs: Self) -> Self::Output {
        Self {
          r: self.r - rhs.r,
          g: self.g - rhs.g,
          b: self.b - rhs.b,
          a: self.a - rhs.a,
        }
      }
    }

    impl std::ops::SubAssign for $type {
      #[inline]
      fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
      }
    }

    impl std::ops::Mul for $type {
      type Output = Self;

      #[inline]
      fn mul(self, rhs: Self) -> Self::Output {
        Self {
          r: self.r * rhs.r,
          g: self.g * rhs.g,
          b: self.b * rhs.b,
          a: self.a * rhs.a,
        }
      }
    }

    impl std::ops::MulAssign for $type {
      #[inline]
      fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
      }
    }

    impl std::ops::Div for $type {
      type Output = Self;

      #[inline]
      fn div(self, rhs: Self) -> Self::Output {
        Self {
          r: self.r / rhs.r,
          g: self.g / rhs.g,
          b: self.b / rhs.b,
          a: self.a / rhs.a,
        }
      }
    }

    impl std::ops::DivAssign for $type {
      #[inline]
      fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
      }
    }

    impl Mul<$scalar> for $type {
      type Output = Self;

      fn mul(self, rhs: $scalar) -> Self::Output {
        Self {
          r: self.r * rhs,
          g: self.g * rhs,
          b: self.b * rhs,
          a: self.a * rhs,
        }
      }
    }

    impl Div<$scalar> for $type {
      type Output = Self;

      fn div(self, rhs: $scalar) -> Self::Output {
        Self {
          r: self.r / rhs,
          g: self.g / rhs,
          b: self.b / rhs,
          a: self.a / rhs,
        }
      }
    }
  };
}

impl_std_ops!(Color, f32);
impl_std_ops!(Color32, u8);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn color_should_be_equatable() {
    let color1 = Color::WHITE;
    let color2 = Color::WHITE;
    let color3 = Color::BLACK;

    assert_eq!(color1, color2);
    assert_ne!(color1, color3);
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
