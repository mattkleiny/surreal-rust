use std::collections::HashMap;

use super::*;

pub const PROJECTION_VIEW: MaterialKey<crate::maths::Matrix4x4<f32>> = MaterialKey::new("u_projectionView");

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
  SrcAlpha,
  SrcColor,
  DstAlpha,
  DstColor,
  OneMinusSrcAlpha,
  OneMinusSrcColor,
  OneMinusDstAlpha,
  OneMinusDstColor,
}

/// A single uniform setting in a material.
#[derive(Debug)]
struct Uniform<G> where G: GraphicsImpl {
  pub location: usize,
  pub value: ShaderUniform<G>,
}

/// A strongly-typed key for a property that can be used in a material.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MaterialKey<T> {
  pub name: &'static str,
  _type: std::marker::PhantomData<T>,
}

impl<T> MaterialKey<T> {
  /// Constructs a new key.
  pub const fn new(name: &'static str) -> Self {
    Self {
      name,
      _type: std::marker::PhantomData,
    }
  }
}

/// A kind of property that can be applied to a material.
pub trait MaterialProperty<G> where G: GraphicsImpl {
  fn apply_to_material(self, material: &mut Material<G>, name: &str);
}

/// Transform shader uniforms into bindings on a material.
impl<G, U> MaterialProperty<G> for U where U : Into<ShaderUniform<G>>, G: GraphicsImpl {
  fn apply_to_material(self, material: &mut Material<G>, name: &str) {
    material.set_uniform(name, self.into());
  }
}

/// Transform textures into bindings on a material.
impl<G> MaterialProperty<G> for Texture<G> where G: GraphicsImpl {
  fn apply_to_material(self, material: &mut Material<G>, name: &str) {
    material.set_texture(name, self.handle, 0, None);
  }
}

/// A material describes how to render a mesh and describes the underlying GPU pipeline state needed.
pub struct Material<'a, G> where G: GraphicsImpl {
  server: GraphicsServer<G>,
  shader: &'a ShaderProgram<G>,
  uniforms: HashMap<String, Uniform<G>>,
  blend_state: BlendState,
}

impl<'a, G> Material<'a, G> where G: GraphicsImpl {
  /// Constructs a new material for the given shader program.
  pub fn new(server: &GraphicsServer<G>, shader: &'a ShaderProgram<G>) -> Self {
    Self {
      server: server.clone(),
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

  /// Sets the given property on the material.
  pub fn set_property<T>(&mut self, key: MaterialKey<T>, value: T) where T: MaterialProperty<G> {
    value.apply_to_material(self, key.name);
  }

  /// Sets the given material uniform.
  pub fn set_uniform(&mut self, name: &str, value: impl Into<ShaderUniform<G>>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      self.uniforms.insert(
        name.to_string(),
        Uniform { location, value: value.into() },
      );
    }
  }

  /// Sets the given material texture, with optional sampler configuration.
  pub fn set_texture(&mut self, name: &str, texture: G::Handle, slot: usize, sampler: Option<TextureSampler>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      self.uniforms.insert(
        name.to_string(),
        Uniform { location, value: ShaderUniform::Texture(texture, slot, sampler) },
      );
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
  pub fn bind(&self) {
    self.server.set_blend_state(self.blend_state);

    for (_, uniform) in &self.uniforms {
      self.shader.set_uniform(uniform.location, &uniform.value);
    }

    self.server.set_active_shader(self.shader.handle);
  }
}
