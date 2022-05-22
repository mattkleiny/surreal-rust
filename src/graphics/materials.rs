use std::collections::HashMap;

use crate::graphics::{GraphicsContext, GraphicsHandle, ShaderProgram, TextureFilter, TextureWrap};
use crate::maths::{Matrix2x2, Matrix3x3, Matrix4x4, Vector2, Vector3, Vector4};

/// Blending states for materials.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlendState {
  Disabled,
  Enabled {
    source: BlendFactor,
    destination: BlendFactor,
  },
}

/// Blending factors for materials.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlendFactor {
  OneMinusSrcAlpha,
  OneMinusSrcColor,
  OneMinusDstAlpha,
  OneMinusDstColor,
}

/// A single uniform value in a material.
#[derive(Debug)]
struct Uniform {
  pub location: usize,
  pub value: UniformValue,
}

impl Uniform {
  /// Creates a new uniform.
  pub fn new(location: usize, value: UniformValue) -> Self {
    Self { location, value }
  }
}

/// Representation of single value that can be used in a `Material`.
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
  Matrix2x2(Matrix2x2<f32>),
  Matrix3x3(Matrix3x3<f32>),
  Matrix4x4(Matrix4x4<f32>),
  Texture(GraphicsHandle, usize, Option<Sampler>),
}

/// A sampler describes how a texture should be read from a shader program.
///
/// Sampler allow re-configuring wrap and filter modes on a per-material basis.
#[derive(Debug)]
pub struct Sampler {
  pub wrap_function: (TextureWrap, TextureWrap, TextureWrap),
  pub minify_filter: TextureFilter,
  pub magnify_filter: TextureFilter,
}

/// A material describes how to render a mesh and describes the underlying GPU pipeline state needed.
pub struct Material<'a> {
  context: GraphicsContext,
  shader: &'a ShaderProgram,
  uniforms: HashMap<String, Uniform>,
  blend_state: BlendState,
}

impl<'a> Material<'a> {
  /// Constructs a new material for the given shader program.
  pub fn new(context: &GraphicsContext, shader: &'a ShaderProgram) -> Self {
    Self {
      context: context.clone(),
      shader,
      uniforms: HashMap::new(),
      blend_state: BlendState::Disabled,
    }
  }

  /// Gets the blend state of the material.
  pub fn blend_state(&self) -> BlendState {
    self.blend_state
  }

  /// Sets the blend state of the material.
  pub fn set_blend_state(&mut self, state: BlendState) {
    self.blend_state = state;
  }

  /// Sets the given material uniform.
  pub fn set_uniform(&mut self, name: &str, value: impl Into<UniformValue>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      self.uniforms.insert(name.to_string(), Uniform::new(location, value.into()));
    }
  }

  /// Sets the given material texture, with optional sampler configuration.
  pub fn set_texture(&mut self, name: &str, texture: GraphicsHandle, slot: usize, sampler: Option<Sampler>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      self.uniforms.insert(name.to_string(), Uniform::new(location, UniformValue::Texture(texture, slot, sampler)));
    }
  }

  /// Removes a uniform from the material.
  pub fn remove_uniform(&mut self, name: &str) {
    self.uniforms.remove(name);
  }

  /// Removes all uniforms from the material.
  pub fn clear_uniforms(&mut self) {
    self.uniforms.clear();
  }

  /// Binds the material as the active shader and uploads it's uniforms.
  pub unsafe fn bind(&self) {
    self.context.set_blend_state(self.blend_state);

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
        UniformValue::Matrix2x2(value) => self.shader.set_uniform_mat2(uniform.location, value),
        UniformValue::Matrix3x3(value) => self.shader.set_uniform_mat3(uniform.location, value),
        UniformValue::Matrix4x4(value) => self.shader.set_uniform_mat4(uniform.location, value),
        UniformValue::Texture(texture, slot, sampler) => {
          self.shader.set_texture(uniform.location, *texture, *slot);

          if let Some(sampler) = sampler {
            self.shader.set_texture_sampler(*texture, sampler);
          }
        }
      };
    }
  }
}

/// Implements uniform value transformation for a common type.
macro implement_uniform($type:ty, $value:ident) {
impl Into<UniformValue> for $type {
  fn into(self) -> UniformValue {
    UniformValue::$value(self)
  }
}
}

implement_uniform!(u32, Integer);
implement_uniform!(f32, Floating);
implement_uniform!(Vector2<i32>, Point2);
implement_uniform!(Vector3<i32>, Point3);
implement_uniform!(Vector4<i32>, Point4);
implement_uniform!(Vector2<f32>, Vector2);
implement_uniform!(Vector3<f32>, Vector3);
implement_uniform!(Vector4<f32>, Vector4);
implement_uniform!(Matrix2x2<f32>, Matrix2x2);
implement_uniform!(Matrix3x3<f32>, Matrix3x3);
implement_uniform!(Matrix4x4<f32>, Matrix4x4);