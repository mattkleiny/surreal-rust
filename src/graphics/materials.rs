use std::collections::HashMap;

use super::*;

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

/// A single uniform setting in a `Material`.
#[derive(Clone)]
struct MaterialUniform {
  pub location: usize,
  pub value: ShaderUniform,
}

/// A material describes how to render a mesh and describes the underlying GPU pipeline state needed.
#[derive(Clone)]
pub struct Material {
  server: GraphicsServer,
  shader: ShaderProgram,
  uniforms: HashMap<String, MaterialUniform>,
  blend_state: BlendState,
}

impl Material {
  /// Constructs a new material for the given shader program.
  pub fn new(server: &GraphicsServer, shader: &ShaderProgram) -> Self {
    Self {
      server: server.clone(),
      shader: shader.clone(),
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
  pub fn set_uniform(&mut self, name: &str, value: impl Into<ShaderUniform>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      let uniform = MaterialUniform {
        location,
        value: value.into(),
      };

      self.uniforms.insert(name.to_string(), uniform);
    }
  }

  /// Sets the given material texture with texture slot and optional sampler options.
  pub fn set_texture(&mut self, name: &str, texture: &Texture, slot: usize, sampler: Option<TextureSampler>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      let uniform = MaterialUniform {
        location,
        value: ShaderUniform::Texture(texture.clone(), slot, sampler),
      };

      self.uniforms.insert(name.to_string(), uniform);
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

    self.server.set_active_shader(self.shader.handle());
  }
}
