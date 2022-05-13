use crate::graphics::GraphicsHandle;
use crate::maths::{Vector2, Vector3, Vector4};

/// Different types fo shaders supported by the engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

/// Represents a single compiled shader program.
#[derive(Debug)]
pub struct ShaderProgram {
  handle: GraphicsHandle,
}

impl ShaderProgram {
  pub fn new() -> Self {
    Self {
      handle: GraphicsHandle(0)
    }
  }

  pub fn get_uniform_location(&self, name: &str) -> Option<usize> {
    Some(0)
  }

  pub unsafe fn set_uniform_u32(&self, location: usize, value: u32) {
    todo!()
  }

  pub unsafe fn set_uniform_f32(&self, location: usize, value: f32) {
    todo!()
  }

  pub unsafe fn set_uniform_vec2i32(&self, location: usize, value: Vector2<i32>) {
    todo!()
  }

  pub unsafe fn set_uniform_vec3i32(&self, location: usize, value: Vector3<i32>) {
    todo!()
  }

  pub unsafe fn set_uniform_vec4i32(&self, location: usize, value: Vector4<i32>) {
    todo!()
  }

  pub unsafe fn set_uniform_vec2f32(&self, location: usize, value: Vector2<f32>) {
    todo!()
  }

  pub unsafe fn set_uniform_vec3f32(&self, location: usize, value: Vector3<f32>) {
    todo!()
  }

  pub unsafe fn set_uniform_vec4f32(&self, location: usize, value: Vector4<f32>) {
    todo!()
  }
}
