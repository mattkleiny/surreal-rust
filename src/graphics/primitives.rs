//! Common graphics primitives.

use crate::maths::{Random, RNG};

/// A simple 32 bit color value with 4 channels (RGBA).
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Color {
  pub const WHITE: Color = Color::RGB(255, 255, 255);
  pub const BLACK: Color = Color::RGB(0, 0, 0);
  pub const RED: Color = Color::RGB(255, 0, 0);
  pub const GREEN: Color = Color::RGB(0, 255, 0);
  pub const BLUE: Color = Color::RGB(0, 0, 255);

  #[allow(non_snake_case)]
  pub const fn RGB(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b, a: 255 }
  }

  #[allow(non_snake_case)]
  pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }
}

impl Random for Color {
  fn random(rng: &mut RNG) -> Self {
    Color::RGBA(rng.next_u8(), rng.next_u8(), rng.next_u8(), rng.next_u8())
  }
}