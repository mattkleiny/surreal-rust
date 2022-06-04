//! Material management and configuration.
//!
//! Materials define all data required to perform some rendering step, from
//! pipeline state changes through to shader programs and uniforms.

use std::collections::HashMap;
use std::marker::PhantomData;

use crate::assets::{Asset, AssetContext, AssetLoader};

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
  SrcAlpha,
  SrcColor,
  DstAlpha,
  DstColor,
  OneMinusSrcAlpha,
  OneMinusSrcColor,
  OneMinusDstAlpha,
  OneMinusDstColor,
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
  culling_mode: CullingMode,
  scissor_mode: ScissorMode,
}

impl Material {
  /// Constructs a new material for the given shader program.
  pub fn new(server: &GraphicsServer, shader: &ShaderProgram) -> Self {
    Self {
      server: server.clone(),
      shader: shader.clone(),
      uniforms: HashMap::new(),
      blend_state: BlendState::Disabled,
      culling_mode: CullingMode::Disabled,
      scissor_mode: ScissorMode::Disabled,
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
  pub fn set_texture(
    &mut self,
    name: &str,
    texture: &Texture,
    slot: usize,
    sampler: Option<TextureSampler>,
  ) {
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
    // TODO: minimize state changes in the underlying platform?
    // TODO: material batching or sorting in a render pipeline?
    self.server.set_blend_state(self.blend_state);
    self.server.set_culling_mode(self.culling_mode);
    self.server.set_scissor_mode(self.scissor_mode);

    for uniform in self.uniforms.values() {
      self.shader.set_uniform_at(uniform.location, &uniform.value);
    }

    self.server.set_active_shader(self.shader.handle());
  }
}

/// An [`AssetLoader`] for materials
pub struct MaterialLoader {
  pub server: GraphicsServer,
}

impl Asset for Material {
  type Loader = MaterialLoader;
}

impl AssetLoader<Material> for MaterialLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<Material> {
    let shader = context.load_asset(context.path)?;
    let material = Material::new(&self.server, &shader);

    Ok(material)
  }
}
