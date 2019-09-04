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

