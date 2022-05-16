use crate::graphics::{GraphicsContext, GraphicsHandle, Sampler};
use crate::maths::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};

/// Different types fo shaders supported by the engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

/// Represents a single compiled shader program.
pub struct ShaderProgram {
  handle: GraphicsHandle,
  context: GraphicsContext,
}

impl ShaderProgram {
  /// Creates a new blank shader program on the GPU.
  pub fn new(context: &GraphicsContext) -> Self {
    Self {
      handle: unsafe { context.create_shader() },
      context: context.clone(),
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

  pub unsafe fn set_uniform_mat2(&self, location: usize, value: &Matrix2<f32>) {
    todo!()
  }

  pub unsafe fn set_uniform_mat3(&self, location: usize, value: &Matrix3<f32>) {
    todo!()
  }

  pub unsafe fn set_uniform_mat4(&self, location: usize, value: &Matrix4<f32>) {
    todo!()
  }

  pub unsafe fn set_texture(&self, location: usize, texture: GraphicsHandle, slot: usize) {
    todo!()
  }

  pub unsafe fn set_texture_sampler(&self, texture: GraphicsHandle, sampler: &Sampler) {
    todo!()
  }
}

impl Drop for ShaderProgram {
  /// Deletes the shader program from the GPU.
  fn drop(&mut self) {
    unsafe {
      self.context.delete_shader(self.handle);
    }
  }
}
