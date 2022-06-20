//! Material management and configuration.
//!
//! Materials define all data required to perform some rendering step, from
//! pipeline state changes through to shader programs and uniforms.

use std::collections::HashMap;

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
  graphics: GraphicsServer,
  shader: ShaderProgram,
  uniforms: HashMap<String, MaterialUniform>,
  textures: TextureBindingSet,
  blend_state: BlendState,
  culling_mode: CullingMode,
  scissor_mode: ScissorMode,
  fullscreen_quad: Option<Mesh<Vertex2>>,
}

impl Material {
  /// Constructs a new material for the given shader program.
  pub fn new(graphics: &GraphicsServer, shader: &ShaderProgram) -> Self {
    Self {
      graphics: graphics.clone(),
      shader: shader.clone(),
      uniforms: HashMap::new(),
      textures: TextureBindingSet::default(),
      blend_state: BlendState::Disabled,
      culling_mode: CullingMode::Disabled,
      scissor_mode: ScissorMode::Disabled,
      fullscreen_quad: None,
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
  /// Sets the given name as uniform.
  pub fn set_uniform(&mut self, name: &str, value: impl Into<ShaderUniform>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      let uniform = MaterialUniform {
        location,
        value: value.into(),
      };

      self.uniforms.insert(name.to_string(), uniform);
    }
  }

  /// Sets the given name as a uniform with a single texture.
  pub fn set_texture(&mut self, name: &str, texture: &Texture, sampler: Option<TextureSampler>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      let slot = self.allocate_texture_slot(texture);

      let uniform = MaterialUniform {
        location,
        value: ShaderUniform::Texture(texture.clone(), slot, sampler),
      };

      self.uniforms.insert(name.to_string(), uniform);
    }
  }

  /// Sets the given name as a uniform with an array of textures.
  pub fn set_texture_array(&mut self, name: &str, textures: &[&Texture], sampler: Option<TextureSampler>) {
    if let Some(location) = self.shader.get_uniform_location(name) {
      let mut bindings = smallvec::SmallVec::<[(Texture, u8); MAX_TEXTURE_UNITS]>::new();

      for texture in textures {
        let slot = self.allocate_texture_slot(texture);
        let texture = (*texture).clone();

        bindings.push((texture, slot));
      }

      let uniform = MaterialUniform {
        location,
        value: ShaderUniform::TextureArray(bindings, sampler),
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
    self.textures.clear();
  }

  /// Binds this material to the graphics server.
  pub fn bind(&self) {
    self.graphics.set_blend_state(self.blend_state);
    self.graphics.set_culling_mode(self.culling_mode);
    self.graphics.set_scissor_mode(self.scissor_mode);

    for uniform in self.uniforms.values() {
      self.shader.set_uniform_at(uniform.location, &uniform.value);
    }

    self.graphics.set_active_shader(self.shader.handle());
  }

  /// Unbinds this material from the graphics server.
  pub fn unbind(&self) {
    self.graphics.set_blend_state(BlendState::Disabled);
    self.graphics.set_culling_mode(CullingMode::Disabled);
    self.graphics.set_scissor_mode(ScissorMode::Disabled);
  }

  /// Draws a fullscreen quad with this material.
  /// TODO: maybe this would make sense in a render pipeline or manager?
  pub fn draw_fullscreen_quad(&mut self) {
    match &self.fullscreen_quad {
      Some(mesh) => mesh.draw(self, PrimitiveTopology::Triangles),
      None => {
        // create the quad lazily
        self.fullscreen_quad = Some(Mesh::create_quad(&self.graphics, 1.));
        self.draw_fullscreen_quad();
      }
    }
  }

  /// Finds the first free texture slot in the material.
  ///
  /// This will also re-organise any old textures back into a linear ordering.
  fn allocate_texture_slot(&mut self, texture: &Texture) -> u8 {
    self
      .textures
      .allocate(texture)
      .expect("Failed to allocate texture slot. There's a limit of 16 concurrent textures per material.")
  }
}

/// An [`AssetLoader`] for materials
pub struct MaterialLoader {
  pub graphics: GraphicsServer,
}

impl Asset for Material {
  type Loader = MaterialLoader;
}

impl AssetLoader<Material> for MaterialLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<Material> {
    let shader = context.load_asset(context.path)?;
    let material = Material::new(&self.graphics, &shader);

    Ok(material)
  }
}

/// Keeps texture assignments uniquely associated with slot indices for use in
/// texture binding in a material.
#[derive(Default, Clone)]
struct TextureBindingSet {
  slots: [Option<GraphicsHandle>; MAX_TEXTURE_UNITS],
}

impl TextureBindingSet {
  /// ALlocates a texture slot for the given texture.
  ///
  /// If the texture is already bound, it will return the existing slot.
  /// Otherwise the first empty slot will be used.
  ///
  /// If we've allocated all texture slots, `None` will be returned.
  pub fn allocate(&mut self, texture: &Texture) -> Option<u8> {
    for (index, slot) in self.slots.iter_mut().enumerate() {
      match slot {
        Some(existing) if *existing == texture.handle() => {
          return Some(index as u8);
        }
        None => {
          *slot = Some(texture.handle());
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
