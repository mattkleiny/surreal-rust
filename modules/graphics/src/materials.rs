//! Material management and configuration.
//!
//! Materials define all data required to perform some rendering step, from
//! pipeline state changes through to shader programs and uniforms.

use core::collections::FastHashMap;

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

/// A set of [`ShaderUniform`]s that can be passed around the application.
#[derive(Default, Clone)]
pub struct MaterialUniformSet {
  uniforms: FastHashMap<String, ShaderUniform>,
  textures: TextureBindingSet,
}

impl MaterialUniformSet {
  /// Sets the given [`UniformKey`] as a uniform in the set.
  pub fn set_uniform<K, U>(&mut self, key: K, value: U)
  where
    K: Into<UniformKey<U>>,
    U: Into<ShaderUniform>,
  {
    let key = key.into().name.to_string();
    let value = value.into();

    self.uniforms.insert(key, value);
  }

  /// Sets the given [`UniformKey`] as a uniform with a single texture in the
  /// set.
  pub fn set_texture<'a, K>(&'a mut self, key: K, texture: &Texture, sampler: Option<TextureSampler>)
  where
    K: Into<UniformKey<&'a Texture>>,
  {
    let slot = self.allocate_texture_slot(texture);
    let uniform = ShaderUniform::Texture(texture.clone(), slot, sampler);

    self.uniforms.insert(key.into().name.to_string(), uniform);
  }

  /// Applies all of the uniforms to the given shader program.
  pub fn apply_to_shader(&self, shader: &ShaderProgram) {
    for (name, uniform) in &self.uniforms {
      shader.set_uniform(name, uniform);
    }
  }

  /// Clears all uniforms from the set.
  pub fn clear(&mut self) {
    self.uniforms.clear();
    self.textures.clear();
  }

  /// Finds the first free texture slot in the material.
  ///
  /// This will also re-organise any old textures back into a linear ordering.
  fn allocate_texture_slot(&mut self, texture: &Texture) -> u8 {
    self.textures.allocate(texture).unwrap_or_else(|| {
      panic!(
        "Failed to allocate texture slot. There's a limit of {MAX_TEXTURE_UNITS} concurrent textures per material.",
      )
    })
  }
}

/// A material describes how to render a mesh and describes the underlying GPU
/// pipeline state needed.
#[derive(Clone)]
pub struct Material {
  graphics: GraphicsEngine,
  shader: ShaderProgram,
  uniforms: MaterialUniformSet,
  blend_state: BlendState,
  culling_mode: CullingMode,
  scissor_mode: ScissorMode,
}

impl Material {
  /// Constructs a new material for the given [`ShaderProgram`].
  pub fn new(graphics: &GraphicsEngine, shader: &ShaderProgram) -> Self {
    Self {
      graphics: graphics.clone(),
      shader: shader.clone(),
      uniforms: MaterialUniformSet::default(),
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

  /// Sets the given [`UniformKey`] with the given value.
  pub fn set_uniform<K, U>(&mut self, key: K, value: U)
  where
    K: Into<UniformKey<U>>,
    U: Into<ShaderUniform>,
  {
    self.uniforms.set_uniform(key, value);
  }

  /// Sets the given [`UniformKey`] with a single texture.
  pub fn set_texture<'a, K>(&'a mut self, key: K, texture: &Texture, sampler: Option<TextureSampler>)
  where
    K: Into<UniformKey<&'a Texture>>,
  {
    self.uniforms.set_texture(key, texture, sampler);
  }

  /// Removes all uniforms from the material.
  pub fn clear_uniforms(&mut self) {
    self.uniforms.clear();
  }

  /// Binds this material to the graphics server.
  pub fn bind(&self) {
    self.graphics.set_blend_state(self.blend_state);
    self.graphics.set_culling_mode(self.culling_mode);
    self.graphics.set_scissor_mode(self.scissor_mode);

    self.uniforms.apply_to_shader(&self.shader);

    self
      .graphics
      .shader_activate(self.shader.id())
      .expect("Failed to activate shader");
  }

  /// Unbinds this material from the graphics server.
  pub fn unbind(&self) {
    self.graphics.set_blend_state(BlendState::Disabled);
    self.graphics.set_culling_mode(CullingMode::Disabled);
    self.graphics.set_scissor_mode(ScissorMode::Disabled);
  }
}

/// Keeps texture assignments uniquely associated with slot indices for use in
/// texture binding in a material.
#[derive(Default, Clone)]
struct TextureBindingSet {
  slots: [Option<TextureId>; MAX_TEXTURE_UNITS],
}

impl TextureBindingSet {
  /// Allocates a texture slot for the given texture.
  ///
  /// If the texture is already bound, it will return the existing slot.
  /// Otherwise the first empty slot will be used.
  ///
  /// If we've allocated all texture slots, `None` will be returned.
  pub fn allocate(&mut self, texture: &Texture) -> Option<u8> {
    for (index, slot) in self.slots.iter_mut().enumerate() {
      match slot {
        Some(existing) if *existing == texture.id() => {
          return Some(index as u8);
        }
        None => {
          *slot = Some(texture.id());
          return Some(index as u8);
        }
        _ => continue,
      }
    }

    None
  }

  /// Clears all used texture slots from the bindings.
  pub fn clear(&mut self) {
    self.slots.fill(None);
  }
}
