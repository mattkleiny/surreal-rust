use std::ops::Neg;

/// Abstract defines behaviour common to all vectors.
pub trait Vector: Sized {
  fn magitude(&self) -> f32;
  fn normalized(&self) -> Self;
}

/// A vector in 2-space.
#[derive(Default, Copy, Clone, Debug)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

impl Vec2 {
  pub const ZERO: Vec2 = Vec2::new(0., 0.);

  pub const fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }
}

impl Vector for Vec2 {
  fn magitude(&self) -> f32 {
    unimplemented!()
  }

  fn normalized(&self) -> Self {
    unimplemented!()
  }
}

impl Neg for Vec2 {
  type Output = Self;

  #[inline]
  fn neg(self) -> Self::Output {
    Self::new(-self.x, -self.y)
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

  pub const fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }
}

impl Vector for Vec3 {
  fn magitude(&self) -> f32 {
    unimplemented!()
  }

  fn normalized(&self) -> Self {
    unimplemented!()
  }
}

impl Neg for Vec3 {
  type Output = Self;

  #[inline]
  fn neg(self) -> Self::Output {
    Self::new(-self.x, -self.y, -self.z)
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

  pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self { x, y, z, w }
  }
}

impl Vector for Vec4 {
  fn magitude(&self) -> f32 {
    unimplemented!()
  }

  fn normalized(&self) -> Self {
    unimplemented!()
  }
}

impl Neg for Vec4 {
  type Output = Self;

  #[inline]
  fn neg(self) -> Self::Output {
    Self::new(-self.x, -self.y, -self.z, -self.w)
  }
}