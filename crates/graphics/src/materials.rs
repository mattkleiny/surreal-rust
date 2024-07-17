//! Material management and configuration.
//!
//! Materials define all data required to perform some rendering step, from
//! pipeline state changes through to shader programs and uniforms.

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
  One,
  SourceAlpha,
  SourceColor,
  DestinationAlpha,
  DestinationColor,
  OneMinusSourceAlpha,
  OneMinusSourceColor,
  OneMinusDestinationAlpha,
  OneMinusDestinationColor,
}

/// Culling modes for materials.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CullingMode {
  Disabled,
  Front,
  Back,
  Both,
}

/// Scissor modes for materials.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ScissorMode {
  Disabled,
  Enabled {
    left: i32,
    bottom: i32,
    width: i32,
    height: i32,
  },
}

/// A material describes how to render a mesh and describes the underlying GPU
/// pipeline state needed.
#[derive(Clone)]
pub struct Material {
  shader: ShaderProgram,
  uniforms: ShaderUniformSet,
  blend_state: BlendState,
  culling_mode: CullingMode,
  scissor_mode: ScissorMode,
}

impl Material {
  /// Constructs a new material for the given [`ShaderProgram`].
  pub fn from_program(shader: &ShaderProgram) -> Self {
    Self {
      shader: shader.clone(),
      uniforms: ShaderUniformSet::default(),
      blend_state: BlendState::Disabled,
      culling_mode: CullingMode::Disabled,
      scissor_mode: ScissorMode::Disabled,
    }
  }

  /// Gets the underlying [`ShaderProgram`] of the material.
  pub fn shader(&self) -> &ShaderProgram {
    &self.shader
  }

  /// Gets the flags of the material.
  pub fn flags(&self) -> MaterialFlags {
    let mut flags = MaterialFlags::empty();

    if self.blend_state() != BlendState::Disabled {
      flags.insert(MaterialFlags::ALPHA_TESTING);
    }

    flags
  }

  /// Gets the underlying [`ShaderUniformSet`] of the material.
  pub fn uniforms(&self) -> &ShaderUniformSet {
    &self.uniforms
  }

  /// Gets the blend state of the material.
  pub fn blend_state(&self) -> BlendState {
    self.blend_state
  }

  /// Sets the blend state of the material.
  pub fn set_blend_state(&mut self, state: BlendState) {
    self.blend_state = state;
  }

  /// Gets the culling mode of the material.
  pub fn culling_mode(&self) -> CullingMode {
    self.culling_mode
  }

  /// Sets the culling mode of the material.
  pub fn set_culling_mode(&mut self, mode: CullingMode) {
    self.culling_mode = mode;
  }

  /// Gets the scissor mode of the material.
  pub fn scissor_mode(&self) -> ScissorMode {
    self.scissor_mode
  }

  /// Sets the scissor mode of the material.
  pub fn set_scissor_mode(&mut self, mode: ScissorMode) {
    self.scissor_mode = mode;
  }

  /// Sets the given [`UniformKey`] with the given value.
  pub fn set_uniform<K, U>(&mut self, key: K, value: U)
  where
    K: Into<ShaderUniformKey<U>>,
    U: Into<ShaderUniform>,
  {
    self.uniforms.set_uniform(key, value);
  }

  /// Sets the given [`UniformKey`] with a single texture.
  pub fn set_texture<'a, K>(&'a mut self, key: K, texture: &Texture, sampler: Option<TextureSampler>)
  where
    K: Into<ShaderUniformKey<&'a Texture>>,
  {
    self.uniforms.set_texture(key, texture, sampler);
  }

  /// Removes all uniforms from the material.
  pub fn clear_uniforms(&mut self) {
    self.uniforms.clear();
  }

  /// Binds this material to the graphics server.
  pub fn bind(&self) {
    let graphics = graphics();

    graphics.set_blend_state(self.blend_state);
    graphics.set_culling_mode(self.culling_mode);
    graphics.set_scissor_mode(self.scissor_mode);

    self.uniforms.apply_to_shader(&self.shader);

    graphics
      .shader_activate(self.shader.id())
      .expect("Failed to activate shader");
  }

  /// Unbinds this material from the graphics server.
  pub fn unbind(&self) {
    let graphics = graphics();

    graphics.set_blend_state(BlendState::Disabled);
    graphics.set_culling_mode(CullingMode::Disabled);
    graphics.set_scissor_mode(ScissorMode::Disabled);
  }
}
