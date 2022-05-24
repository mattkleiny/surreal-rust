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
#[derive(Debug)]
struct MaterialUniform<G> where G: GraphicsImpl {
  pub location: usize,
  pub value: ShaderUniform<G>,
}

/// A material describes how to render a mesh and describes the underlying GPU pipeline state needed.
pub struct Material<'a, G> where G: GraphicsImpl {
  server: GraphicsServer<G>,
  shader: &'a ShaderProgram<G>,
  uniforms: HashMap<String, MaterialUniform<G>>,
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

  /// Sets the given material uniform.
  pub fn set_uniform(&mut self, name: &str, value: impl Into<ShaderUniform<G>>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      self.uniforms.insert(
        name.to_string(),
        MaterialUniform { location, value: value.into() },
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
