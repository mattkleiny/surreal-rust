use std::collections::HashMap;

use crate::graphics::{GraphicsHandle, MagnifyFilter, MinifyFilter, ShaderProgram, WrapFunction};
use crate::maths::{Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};

/// A material of uniform values and associated `ShaderProgram`.
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
  pub fn set_uniform(&mut self, name: &str, value: impl Into<UniformValue>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      let key = name.to_string();
      let value = Uniform {
        location,
        value: value.into(),
      };

      self.uniforms.insert(key, value);
    }
  }

  /// Sets the given material texture.
  pub fn set_texture(&mut self, name: &str, texture: GraphicsHandle, slot: usize, sampler: Option<Sampler>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      let key = name.to_string();
      let value = Uniform {
        location,
        value: UniformValue::Texture(texture, slot, sampler),
      };

      self.uniforms.insert(key, value);
    }
  }

  /// Removes a uniform from the material.
  pub fn clear_uniform(&mut self, name: &str) {
    self.uniforms.remove(name);
  }

  /// Binds the material as the active shader and uploads it's uniforms.
  pub unsafe fn bind(&self) {
    for (_, uniform) in &self.uniforms {
      match &uniform.value {
        UniformValue::Integer(value) => self.shader.set_uniform_u32(uniform.location, *value),
        UniformValue::Floating(value) => self.shader.set_uniform_f32(uniform.location, *value),
        UniformValue::Point2(value) => self.shader.set_uniform_vec2i32(uniform.location, *value),
        UniformValue::Point3(value) => self.shader.set_uniform_vec3i32(uniform.location, *value),
        UniformValue::Point4(value) => self.shader.set_uniform_vec4i32(uniform.location, *value),
        UniformValue::Vector2(value) => self.shader.set_uniform_vec2f32(uniform.location, *value),
        UniformValue::Vector3(value) => self.shader.set_uniform_vec3f32(uniform.location, *value),
        UniformValue::Vector4(value) => self.shader.set_uniform_vec4f32(uniform.location, *value),
        UniformValue::Texture(texture, slot, sampler) => {
          // TODO: set sampler state, as well
          self.shader.set_texture(uniform.location, *texture, *slot)
        }
        _ => {}
      };
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
  Texture(GraphicsHandle, usize, Option<Sampler>),
}

/// Behaviour for a sampler in a material.
#[derive(Debug)]
pub struct Sampler {
  pub wrap_function: (WrapFunction, WrapFunction, WrapFunction),
  pub minify_filter: MinifyFilter,
  pub magnify_filter: MagnifyFilter,
}

macro_rules! implement_uniform {
  ($type:ty, $value:tt) => {
    impl Into<UniformValue> for $type {
      fn into(self) -> UniformValue {
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
