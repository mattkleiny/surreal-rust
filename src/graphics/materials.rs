use std::collections::HashMap;

use crate::graphics::{GraphicsHandle, ShaderProgram};
use crate::maths::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};

/// A material of uniform values and associated `Shader`.
#[derive(Debug)]
pub struct Material<'a> {
  shader: &'a ShaderProgram,
  uniforms: HashMap<String, Uniform>,
}

impl<'a> Material<'a> {
  /// Constructs a new material for the given shader program.
  pub fn new(shader: &'a ShaderProgram) -> Self {
    Self {
      shader,
      uniforms: HashMap::new(),
    }
  }

  /// Sets the given material uniform.
  pub fn set_uniform<T>(&mut self, name: &str, value: T) where T: IntoUniform {
    if let Some(location) = self.shader.get_uniform_location(name) {
      let key = name.to_string();
      let value = Uniform {
        location,
        value: value.to_uniform(),
      };

      self.uniforms.insert(key, value);
    }
  }

  /// Binds the material as the active shader and uploads it's uniforms.
  pub unsafe fn upload(&self) {
    for (_, uniform) in &self.uniforms {
      uniform.value.upload(uniform.location, &self.shader);
    }
  }
}

/// A single uniform value in a material.
#[derive(Debug)]
struct Uniform {
  pub location: usize,
  pub value: UniformValue,
}

/// Representation of single value in a `Uniform` in a `Material`.
#[derive(Debug)]
pub enum UniformValue {
  Integer(u32),
  Floating(f32),
  Point2(Vector2<i32>),
  Point3(Vector3<i32>),
  Point4(Vector4<i32>),
  Vector2(Vector2<f32>),
  Vector3(Vector3<f32>),
  Vector4(Vector4<f32>),
  Matrix2(Matrix2<f32>),
  Matrix3(Matrix3<f32>),
  Matrix4(Matrix4<f32>),
  Texture(GraphicsHandle, usize),
}

impl UniformValue {
  unsafe fn upload(&self, location: usize, program: &ShaderProgram) {
    match self {
      UniformValue::Integer(value) => program.set_uniform_u32(location, *value),
      UniformValue::Floating(value) => program.set_uniform_f32(location, *value),
      UniformValue::Point2(value) => program.set_uniform_vec2i32(location, *value),
      UniformValue::Point3(value) => program.set_uniform_vec3i32(location, *value),
      UniformValue::Point4(value) => program.set_uniform_vec4i32(location, *value),
      UniformValue::Vector2(value) => program.set_uniform_vec2f32(location, *value),
      UniformValue::Vector3(value) => program.set_uniform_vec3f32(location, *value),
      UniformValue::Vector4(value) => program.set_uniform_vec4f32(location, *value),
      _ => {}
    }
  }
}

/// Allows conversion of value into a `UniformValue`.
pub trait IntoUniform {
  fn to_uniform(self) -> UniformValue;
}

macro_rules! implement_uniform {
  ($type:ty, $value:tt) => {
    impl IntoUniform for $type {
      fn to_uniform(self) -> UniformValue {
        UniformValue::$value(self)
      }
    }
  };
}

implement_uniform!(u32, Integer);
implement_uniform!(f32, Floating);
implement_uniform!(Vector2<i32>, Point2);
implement_uniform!(Vector3<i32>, Point3);
implement_uniform!(Vector4<i32>, Point4);
implement_uniform!(Vector2<f32>, Vector2);
implement_uniform!(Vector3<f32>, Vector3);
implement_uniform!(Vector4<f32>, Vector4);
implement_uniform!(Matrix2<f32>, Matrix2);
implement_uniform!(Matrix3<f32>, Matrix3);
implement_uniform!(Matrix4<f32>, Matrix4);

#[cfg(test)]
mod tests {
  use crate::maths::{vec2, vec3};

  use super::*;

  #[test]
  fn it_should_set_uniform_values() {
    let shader = ShaderProgram::new();
    let mut material = Material::new(&shader);

    material.set_uniform("Test 1", vec2(0., 1.));
    material.set_uniform("Test 2", vec3(0., 1., 0.));
    material.set_uniform("Test 3", vec2(0., 1.));
    material.set_uniform("Test 4", vec3(0., 1., 0.));

    println!("{:#?}", material);

    unsafe { material.upload(); }
  }
}