//! Shader loading and management.
//!
//! Shader programs form the programmable part of the GPU pipeline, outside of
//! state changes, and are managed through this module.
//!
//! For higher-level shader control see the material module instead.

use bitflags::bitflags;
use common::*;

use super::*;

mod lang;
mod templates;

pub use lang::*;
pub use templates::*;

/// The nominal max number of texture units that might be bound in the GPU
/// for a single shader program.
///
/// This is a hint for sizing arrays and other data structures.
const MAX_TEXTURE_UNITS: usize = 32;

bitflags! {
  /// Metadata flags indicating what state the shader program requires.
  #[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
  pub struct ShaderFlags: u32 {
    const ALPHA_TESTING = 0b0000001;
    const DEPTH_TESTING = 0b00000010;
    const DEPTH_WRITING = 0b00000100;
  }
}

/// Different types of shaders supported by the engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
  Compute,
}

/// Defines a single kernel function in a shader program.
#[derive(Debug)]
pub struct ShaderKernel {
  pub kind: ShaderKind,
  pub code: String,
}

/// Represents a single compiled shader program.
#[derive(Clone)]
pub struct ShaderProgram {
  state: internal::GraphicsCell<ShaderProgramState>,
}

/// The internal state for a [`ShaderProgram`] .
struct ShaderProgramState {
  id: ShaderId,
  location_cache: FastHashMap<String, Option<usize>>,
}

impl ShaderProgram {
  /// Creates a new blank [`ShaderProgram`] on the GPU.
  pub fn new() -> Result<Self, ShaderError> {
    Ok(Self {
      state: internal::GraphicsCell::new(ShaderProgramState {
        id: graphics().shader_create()?,
        location_cache: FastHashMap::default(),
      }),
    })
  }

  /// Loads a [`ShaderProgram`] from the given [`VirtualPath`] code.
  pub fn from_path<S: ShaderLanguage>(path: impl ToVirtualPath) -> Result<Self, ShaderError> {
    let path = path.to_virtual_path();
    let mut stream = path.open_input_stream().map_err(|_| ShaderError::FailedToLoad)?;

    Self::from_stream::<S>(&mut stream)
  }

  /// Loads a [`ShaderProgram`] from the given [`VirtualPath`] code.
  pub fn from_stream<S: ShaderLanguage>(stream: &mut dyn InputStream) -> Result<Self, ShaderError> {
    let code = stream.to_string().map_err(|_| ShaderError::FailedToLoad)?;

    Self::from_code::<S>(&code)
  }

  /// Loads a [`ShaderProgram`] from the given raw shader code.
  pub fn from_code<S: ShaderLanguage>(code: &str) -> Result<Self, ShaderError> {
    let program = Self::new()?;

    program.load_code::<S>(code)?;

    Ok(program)
  }

  /// Loads a [`ShaderProgram`] from the given [`ShaderKernel`]s.
  pub fn from_kernels(kernels: &[ShaderKernel]) -> Result<Self, ShaderError> {
    let program = Self::new()?;

    program.load_kernels(kernels)?;

    Ok(program)
  }

  /// Returns the [`ShaderId`] of the underlying program.
  pub fn id(&self) -> ShaderId {
    self.state.read().id
  }

  /// Retrieves the binding location of the given shader uniform in the
  /// underlying program.
  pub fn get_uniform_location(&self, name: &str) -> Option<usize> {
    let state = self.state.read();

    if let Some(location) = state.location_cache.get(name) {
      return location.to_owned();
    }

    drop(state);

    let mut state = self.state.write();
    let location = graphics().shader_uniform_location(state.id, name);

    state.location_cache.insert(name.to_string(), location);

    location
  }

  /// Sets the given uniform value in the underlying program.
  pub fn set_uniform(&self, name: &str, value: &ShaderUniform) {
    if let Some(location) = self.get_uniform_location(name) {
      let state = self.state.read();

      graphics()
        .shader_set_uniform(state.id, location, value)
        .expect("Failed to set uniform");
    }
  }

  /// Reloads the [`ShaderProgram`] from a file at the given virtual path.
  pub fn load_from_path<S: ShaderLanguage>(&self, path: impl ToVirtualPath) -> Result<(), ShaderError> {
    let path = path.to_virtual_path();
    let mut stream = path.open_input_stream().map_err(|_| ShaderError::FailedToLoad)?;

    self.load_from_stream::<S>(&mut stream)?;

    Ok(())
  }

  /// Reloads the [`ShaderProgram`] from a stream.
  pub fn load_from_stream<S: ShaderLanguage>(&self, stream: &mut dyn InputStream) -> Result<(), ShaderError> {
    let mut source_code = String::new();

    stream
      .read_to_string(&mut source_code)
      .map_err(|_| ShaderError::FailedToLoad)?;

    self.load_code::<S>(&source_code)?;

    Ok(())
  }

  /// Reloads the [`ShaderProgram`] from the given shader code.
  pub fn load_code<S: ShaderLanguage>(&self, text: &str) -> Result<(), ShaderError> {
    self.load_kernels(&S::parse_kernels(text)?)?;

    Ok(())
  }

  /// Reloads the [`ShaderProgram`] from the given shader code.
  pub fn load_kernels(&self, kernels: &[ShaderKernel]) -> Result<(), ShaderError> {
    graphics().shader_link(self.id(), kernels)?;

    Ok(())
  }
}

impl Drop for ShaderProgramState {
  fn drop(&mut self) {
    graphics()
      .shader_delete(self.id)
      .expect("Failed to delete shader program");
  }
}

/// Representation of a single value that can be used in a shader.
#[derive(Clone)]
pub enum ShaderUniform {
  Bool(bool),
  I32(i32),
  U32(u32),
  F32(f32),
  Vec2(Vec2),
  Vec3(Vec3),
  Vec4(Vec4),
  DVec2(DVec2),
  DVec3(DVec3),
  DVec4(DVec4),
  Mat2(Mat2),
  Mat3(Mat3),
  Mat4(Mat4),
  DMat2(DMat2),
  DMat3(DMat3),
  DMat4(DMat4),
  Quat(Quat),
  DQuat(DQuat),
  Color(Color),
  Color32(Color32),
  Texture(TextureId, u8, Option<TextureSampler>),
  TextureArray(Vec<TextureId>),
}

/// Implements uniform value transformation for common types.
macro_rules! impl_uniform {
  ($type:ty as $value:ident) => {
    impl From<$type> for ShaderUniform {
      fn from(value: $type) -> Self {
        ShaderUniform::$value(value.into())
      }
    }

    impl From<&$type> for ShaderUniform {
      fn from(value: &$type) -> Self {
        ShaderUniform::$value(value.to_owned().into())
      }
    }
  };
}

impl_uniform!(bool as Bool);
impl_uniform!(u32 as U32);
impl_uniform!(f32 as F32);
impl_uniform!(Vec2 as Vec2);
impl_uniform!(Vec3 as Vec3);
impl_uniform!(Vec4 as Vec4);
impl_uniform!(DVec2 as DVec2);
impl_uniform!(DVec3 as DVec3);
impl_uniform!(DVec4 as DVec4);
impl_uniform!(Mat2 as Mat2);
impl_uniform!(Mat3 as Mat3);
impl_uniform!(Mat4 as Mat4);
impl_uniform!(DMat2 as DMat2);
impl_uniform!(DMat3 as DMat3);
impl_uniform!(DMat4 as DMat4);
impl_uniform!(Quat as Quat);
impl_uniform!(DQuat as DQuat);
impl_uniform!(Color as Color);
impl_uniform!(Color32 as Color32);

/// Identifies a kind of [`ShaderUniform`] for strongly-typed assignment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ShaderUniformKey<U> {
  pub name: &'static str,
  _phantom: std::marker::PhantomData<U>,
}

impl<U> ShaderUniformKey<U> {
  /// Creates a new uniform key with the given name.
  #[inline(always)]
  pub const fn new(name: &'static str) -> Self {
    Self {
      name,
      _phantom: std::marker::PhantomData,
    }
  }
}

impl<U> From<&'static str> for ShaderUniformKey<U> {
  fn from(name: &'static str) -> Self {
    ShaderUniformKey::new(name)
  }
}

/// A set of [`ShaderUniform`]s that can be passed around the application.
#[derive(Default, Clone)]
pub struct ShaderUniformSet {
  uniforms: FastHashMap<String, ShaderUniform>,
  textures: TextureBindingSet,
}

impl ShaderUniformSet {
  /// Sets the given key as a uniform.
  pub fn set_uniform<K, U>(&mut self, key: K, value: U)
  where
    K: Into<ShaderUniformKey<U>>,
    U: Into<ShaderUniform>,
  {
    let key = key.into().name.to_string();
    let value = value.into();

    self.uniforms.insert(key, value);
  }

  /// Sets the given key as a uniform with a single texture.
  pub fn set_texture<'a, K>(&mut self, key: K, texture: &'a Texture, sampler: Option<TextureSampler>)
  where
    K: Into<ShaderUniformKey<&'a Texture>>,
  {
    let key = key.into().name.to_string();
    let slot = self.allocate_texture_slot(texture);
    let uniform = ShaderUniform::Texture(texture.id(), slot, sampler);

    self.uniforms.insert(key, uniform);
  }

  /// Applies all the uniforms to the given shader program.
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
    self.textures.try_allocate(texture).unwrap_or_else(|| {
      panic!(
        "Failed to allocate texture slot. There's a limit of {MAX_TEXTURE_UNITS} concurrent textures per material."
      )
    })
  }

  /// Returns an iterator over all uniforms in the set.
  pub fn iter(&self) -> impl Iterator<Item = (&String, &ShaderUniform)> {
    self.uniforms.iter()
  }
}

impl<'a> IntoIterator for &'a ShaderUniformSet {
  type Item = (&'a String, &'a ShaderUniform);
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.uniforms.iter()
  }
}

/// Keeps texture assignments uniquely associated with slot indices.
///
/// This is useful for tracking unique texture assignments across multiple
/// materials, invocations, vertices, etc.
#[derive(Default, Clone)]
pub struct TextureBindingSet {
  slots: [Option<TextureId>; MAX_TEXTURE_UNITS],
}

impl TextureBindingSet {
  /// Allocates a texture slot for the given texture.
  ///
  /// If the texture is already bound, it will return the existing slot.
  /// Otherwise, the first empty slot will be used.
  ///
  /// If we've allocated all texture slots, `None` will be returned.
  pub fn try_allocate(&mut self, texture: &Texture) -> Option<u8> {
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

  /// Returns an iterator over all texture IDs in the set.
  pub fn iter(&self) -> impl Iterator<Item = &TextureId> {
    self.slots.iter().filter_map(|slot| slot.as_ref())
  }

  /// Returns a vector of all texture IDs in the set.
  pub fn to_vec(&self) -> Vec<TextureId> {
    self.iter().copied().collect()
  }

  /// Returns an iterator over all texture IDs in the set.
  pub fn into_vec(self) -> Vec<TextureId> {
    self.slots.into_iter().flatten().collect()
  }
}

impl<'a> IntoIterator for &'a TextureBindingSet {
  type Item = &'a TextureId;
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
