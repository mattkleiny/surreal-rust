use std::ops::Neg;

use crate::maths::{Lerp, Random, RNG};

/// A vector in 2-space.
#[derive(Default, Copy, Clone, Debug)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

impl Vec2 {
  pub const ZERO: Vec2 = Vec2::new(0., 0.);
  pub const ONE: Vec2 = Vec2::new(1., 1.);

  pub const fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }
}

impl Neg for Vec2 {
  type Output = Self;

  #[inline]
  fn neg(self) -> Self::Output {
    Self::new(-self.x, -self.y)
  }
}

impl RNG for Vec2 {
  fn random(random: &mut Random) -> Self {
    Self::new(
      random.next_f32(),
      random.next_f32(),
    )
  }
}
impl Lerp for Vec2 {
  fn lerp(from: Self, to: Self, amount: f32) -> Self {
    Self::new(
      f32::lerp(from.x, to.x, amount),
      f32::lerp(from.y, to.y, amount),
    )
  }
}

/// A vector in 3-space.
#[derive(Default, Copy, Clone, Debug)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3 {
  pub const ZERO: Vec3 = Vec3::new(0., 0., 0.);
  pub const ONE: Vec3 = Vec3::new(1., 1., 1.);

  pub const fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }
}

impl Neg for Vec3 {
  type Output = Self;

  #[inline]
  fn neg(self) -> Self::Output {
    Self::new(-self.x, -self.y, -self.z)
  }
}

impl RNG for Vec3 {
  fn random(random: &mut Random) -> Self {
    Self::new(
      random.next_f32(),
      random.next_f32(),
      random.next_f32(),
    )
  }
}

impl Lerp for Vec3 {
  fn lerp(from: Self, to: Self, amount: f32) -> Self {
    Self::new(
      f32::lerp(from.x, to.x, amount),
      f32::lerp(from.y, to.y, amount),
      f32::lerp(from.z, to.z, amount),
    )
  }
}

/// A vector in 4-space.
#[derive(Default, Copy, Clone, Debug)]
pub struct Vec4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl Vec4 {
  pub const ZERO: Vec4 = Vec4::new(0., 0., 0., 0.);
  pub const ONE: Vec4 = Vec4::new(1., 1., 1., 1.);

  pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self { x, y, z, w }
  }
}

impl Neg for Vec4 {
  type Output = Self;

  #[inline]
  fn neg(self) -> Self::Output {
    Self::new(-self.x, -self.y, -self.z, -self.w)
  }
}

impl RNG for Vec4 {
  fn random(random: &mut Random) -> Self {
    Self::new(
      random.next_f32(),
      random.next_f32(),
      random.next_f32(),
      random.next_f32(),
    )
  }
}

impl Lerp for Vec4 {
  fn lerp(from: Self, to: Self, amount: f32) -> Self {
    Self::new(
      f32::lerp(from.x, to.x, amount),
      f32::lerp(from.y, to.y, amount),
      f32::lerp(from.z, to.z, amount),
      f32::lerp(from.w, to.w, amount),
    )
  }
}