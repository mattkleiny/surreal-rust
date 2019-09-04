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