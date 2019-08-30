//! Common graphics primitives.

use crate::maths::{Random, RandomGenerator};

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

impl RandomGenerator for Color {
  fn random(rng: &mut Random) -> Self {
    Color::RGBA(rng.next_u8(), rng.next_u8(), rng.next_u8(), rng.next_u8())
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::Seed;

  use super::*;

  #[test]
  fn color_should_generate_random_values() {
    let seed = Seed::random();
    let mut rng = seed.to_rng();

    let color1 = Color::random(&mut rng);
    let color2 = Color::random(&mut rng);
    let color3 = Color::random(&mut rng);
    let color4 = Color::random(&mut rng);

    assert_ne!(color1, color2);
    assert_ne!(color2, color3);
    assert_ne!(color3, color4);
  }
}