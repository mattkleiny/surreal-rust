use std::collections::HashMap;

use crate::graphics::{GraphicsImpl, GraphicsServer, ShaderProgram, ShaderUniform, TextureSampler};

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

/// A single entry in a material.
#[derive(Debug)]
struct Entry<G> where G: GraphicsImpl {
  pub location: usize,
  pub value: ShaderUniform<G>,
}

impl<G> Entry<G> where G: GraphicsImpl {
  /// Creates a new uniform.
  pub fn new(location: usize, value: ShaderUniform<G>) -> Self {
    Self { location, value }
  }
}

/// A material describes how to render a mesh and describes the underlying GPU pipeline state needed.
pub struct Material<'a, G> where G: GraphicsImpl {
  server: GraphicsServer<G>,
  shader: &'a ShaderProgram<G>,
  entries: HashMap<String, Entry<G>>,
  blend_state: BlendState,
}

impl<'a, G> Material<'a, G> where G: GraphicsImpl {
  /// Constructs a new material for the given shader program.
  pub fn new(server: &GraphicsServer<G>, shader: &'a ShaderProgram<G>) -> Self {
    Self {
      server: server.clone(),
      shader,
      entries: HashMap::new(),
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
  pub fn set_uniform(&mut self, name: &str, value: impl Into<ShaderUniform<G>>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      self.entries.insert(
        name.to_string(),
        Entry::new(location, value.into()),
      );
    }
  }

  /// Sets the given material texture, with optional sampler configuration.
  pub fn set_texture(&mut self, name: &str, texture: G::Handle, slot: usize, sampler: Option<TextureSampler>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      self.entries.insert(
        name.to_string(),
        Entry::new(location, ShaderUniform::Texture(texture, slot, sampler)),
      );
    }
  }

  /// Removes a uniform from the material.
  pub fn remove_uniform(&mut self, name: &str) {
    self.entries.remove(name);
  }

  /// Removes all uniforms from the material.
  pub fn clear_uniforms(&mut self) {
    self.entries.clear();
  }

  /// Binds the material as the active shader and uploads it's uniforms.
  pub fn bind(&self) {
    self.server.set_blend_state(self.blend_state);

    for (_, uniform) in &self.entries {
      self.shader.set_uniform(uniform.location, &uniform.value);
    }

    self.server.set_active_shader(self.shader.handle);
  }
}
