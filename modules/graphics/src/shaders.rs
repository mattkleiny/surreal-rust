//! Shader loading and management.
//!
//! Shader programs form the programmable part of the GPU pipeline, outside of
//! state changes, and are managed through this module.
//!
//! For higher-level shader control see the material module instead.

use core::{
  collections::FastHashMap,
  io::VirtualPath,
  maths::{
    DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, Degrees, Mat2, Mat3, Mat4, Quat, Radians, Vec2, Vec3, Vec4,
  },
};
use std::{cell::RefCell, rc::Rc};

use super::*;

mod glsl;
mod shady;

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

/// Represents a language for [`ShaderKernel`] compilation.
///
/// Abstracting over shader languages allows us to build out new language
/// paradigms in the future.
pub trait ShaderLanguage {
  /// Parses the given raw source code into one or more [`ShaderKernel`]s.
  fn parse_kernels(source_code: &str) -> core::Result<Vec<ShaderKernel>>;
}

/// Represents a single compiled shader program.
#[derive(Clone)]
pub struct ShaderProgram {
  state: Rc<RefCell<ShaderProgramState>>,
}

/// The internal state for a [`ShaderProgram`] .
struct ShaderProgramState {
  id: ShaderId,
  graphics: GraphicsEngine,
  location_cache: FastHashMap<String, Option<usize>>,
}

impl ShaderProgram {
  /// Creates a new blank [`ShaderProgram`] on the GPU.
  pub fn new(graphics: &GraphicsEngine) -> core::Result<Self> {
    Ok(Self {
      state: Rc::new(RefCell::new(ShaderProgramState {
        id: graphics.shader_create()?,
        graphics: graphics.clone(),
        location_cache: FastHashMap::default(),
      })),
    })
  }

  /// Loads a [`ShaderProgram`] from the given raw GLSL shader code.
  pub fn from_glsl(graphics: &GraphicsEngine, code: &str) -> core::Result<Self> {
    Self::from_code::<glsl::GlslShaderLanguage>(graphics, code)
  }

  /// Loads a [`ShaderProgram`] from the given raw GLSL shader code file.
  pub fn from_glsl_path<'a>(graphics: &GraphicsEngine, path: impl Into<VirtualPath<'a>>) -> core::Result<Self> {
    Self::from_path::<glsl::GlslShaderLanguage>(graphics, path)
  }

  /// Loads a [`ShaderProgram`] from the given Shady shader code.
  pub fn from_shady(graphics: &GraphicsEngine, code: &str) -> core::Result<Self> {
    Self::from_code::<shady::ShadyShaderLanguage>(graphics, code)
  }

  /// Loads a [`ShaderProgram`] from the given Shady shader code file.
  pub fn from_shady_path<'a>(graphics: &GraphicsEngine, path: impl Into<VirtualPath<'a>>) -> core::Result<Self> {
    Self::from_path::<shady::ShadyShaderLanguage>(graphics, path)
  }

  /// Loads a [`ShaderProgram`] from the given raw shader code.
  pub fn from_code<S: ShaderLanguage>(graphics: &GraphicsEngine, code: &str) -> core::Result<Self> {
    let program = Self::new(graphics)?;

    program.load_code::<S>(code)?;

    Ok(program)
  }

  /// Loads a [`ShaderProgram`] from the given [`VirtualPath`] code.
  pub fn from_path<'a, S: ShaderLanguage>(
    graphics: &GraphicsEngine,
    path: impl Into<VirtualPath<'a>>,
  ) -> core::Result<Self> {
    let path = path.into();
    let code = path.read_all_text()?;

    Self::from_code::<S>(graphics, &code)
  }

  /// Returns the [`ShaderId`] of the underlying program.
  pub fn id(&self) -> ShaderId {
    self.state.borrow().id
  }

  /// Retrieves the binding location of the given shader uniform in the
  /// underlying program.
  pub fn get_uniform_location(&self, name: &str) -> Option<usize> {
    let state = self.state.borrow();

    if let Some(location) = state.location_cache.get(name) {
      return location.to_owned();
    }

    drop(state);

    let mut state = self.state.borrow_mut();
    let graphics = &state.graphics;
    let location = graphics.shader_uniform_location(state.id, name);

    state.location_cache.insert(name.to_string(), location);

    location
  }

  /// Sets the given uniform value in the underlying program.
  pub fn set_uniform(&self, name: &str, value: &ShaderUniform) {
    if let Some(location) = self.get_uniform_location(name) {
      let state = self.state.borrow();
      let graphics = &state.graphics;

      graphics
        .shader_set_uniform(state.id, location, value)
        .expect("Failed to set uniform");
    }
  }

  /// Reloads the [`ShaderProgram`] from the given shader code.
  pub fn load_code<S: ShaderLanguage>(&self, text: &str) -> core::Result<()> {
    let state = self.state.borrow();
    let graphics = &state.graphics;
    let shaders = S::parse_kernels(text)?;

    graphics.shader_link(state.id, &shaders)?;

    Ok(())
  }

  /// Reloads the [`ShaderProgram`] from a file at the given virtual path.
  pub fn load_from_path<'a, S: ShaderLanguage>(&self, path: impl Into<VirtualPath<'a>>) -> core::Result<()> {
    let path = path.into();
    let source_code = path.read_all_text()?;

    self.load_code::<S>(&source_code)?;

    Ok(())
  }
}

impl Drop for ShaderProgramState {
  fn drop(&mut self) {
    self
      .graphics
      .shader_delete(self.id)
      .expect("Failed to delete shader program");
  }
}

/// An [`AssetLoader`] for shader programs
pub struct ShaderProgramLoader {
  pub graphics: GraphicsEngine,
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
        ShaderUniform::$value(value.clone().into())
      }
    }
  };
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
  Texture(Texture, u8, Option<TextureSampler>),
  Array(Vec<ShaderUniform>),
}

/// Allow for the conversion of a slice of values into a shader uniform array,
/// provided all of the values can be individually converted into a uniform.
impl<U> From<&[U]> for ShaderUniform
where
  for<'a> &'a U: Into<ShaderUniform>,
{
  fn from(value: &[U]) -> Self {
    Self::Array(value.iter().map(|v| v.into()).collect::<Vec<ShaderUniform>>())
  }
}

/// Identifies a kind of [`ShaderUniform`] for strongly-typed assignment.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UniformKey<U> {
  pub name: &'static str,
  _phantom: std::marker::PhantomData<U>,
}

impl<U> UniformKey<U> {
  /// Creates a new uniform key with the given name.
  #[inline(always)]
  pub const fn new(name: &'static str) -> Self {
    Self {
      name,
      _phantom: std::marker::PhantomData,
    }
  }
}

impl<U> From<&'static str> for UniformKey<U> {
  fn from(name: &'static str) -> Self {
    UniformKey::new(name)
  }
}

impl_uniform!(bool as Bool);
impl_uniform!(u32 as U32);
impl_uniform!(f32 as F32);
impl_uniform!(Degrees as F32);
impl_uniform!(Radians as F32);
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
