/// Abstract defines behaviour common to all matrices.
pub trait Matrix {
  fn transpose(&self) -> Self;
}

/// A 2x2 matrix.
#[derive(Default, Copy, Clone, Debug)]
pub struct Mat2 {
  elements: [f32; 2 * 2],
}

impl Mat2 {
  pub fn identity() -> Self {
    unimplemented!()
  }

  pub fn as_ref(&self) -> &[f32; 2 * 2] {
    unimplemented!()
  }
}

impl Matrix for Mat2 {
  fn transpose(&self) -> Self {
    unimplemented!()
  }
}

/// A 3x3 matrix.
#[derive(Default, Copy, Clone, Debug)]
pub struct Mat3 {
  elements: [f32; 3 * 3],
}

impl Mat3 {
  pub fn identity() -> Self {
    unimplemented!()
  }

  pub fn as_ref(&self) -> &[f32; 3 * 3] {
    unimplemented!()
  }
}

impl Matrix for Mat3 {
  fn transpose(&self) -> Self {
    unimplemented!()
  }
}

/// A 4x4 matrix.
#[derive(Default, Copy, Clone, Debug)]
pub struct Mat4 {
  elements: [f32; 4 * 4],
}

impl Mat4 {
  pub fn identity() -> Self {
    unimplemented!()
  }

  pub fn as_ref(&self) -> &[f32; 4 * 4] {
    unimplemented!()
  }
}

impl Matrix for Mat4 {
  fn transpose(&self) -> Self {
    unimplemented!()
  }
}