//! Shader loading and management.
//!
//! Shader programs form the programmable part of the GPU pipeline, outside of
//! state changes, and are managed through this module.
//!
//! For higher-level shader control see the material module instead.

use std::{cell::RefCell, rc::Rc};

use super::*;
use crate::{
  assets::{Asset, AssetContext, AssetLoader},
  collections::FastHashMap,
  io::VirtualPath,
  maths::{Degrees, Mat2, Mat3, Mat4, Radians, Vec2, Vec3, Vec4},
};

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
  Mat2(Mat2),
  Mat3(Mat3),
  Mat4(Mat4),
  Color(Color),
  Color32(Color32),
  Texture(Texture, u8, Option<TextureSampler>),
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

/// Represents a language for [`ShaderKernel`] compilation.
///
/// Abstracting over shader languages allows us to build out new language
/// paradigms in the future.
pub trait ShaderLanguage {
  /// Parses the given raw source code into one or more [`ShaderKernel`]s.
  fn parse_kernels(source_code: &str) -> crate::Result<Vec<ShaderKernel>>;
}

/// The OpenGL [`ShaderLanguage`] implementation.
pub struct GLSL;

impl ShaderLanguage for GLSL {
  /// Parses the given raw GLSL source and performs some basic pre-processing.
  ///
  /// Allows for the following basic transformations:
  ///
  /// * Multiple shader types per file (separated with #shader_type directives).
  /// * Shared code amongst each shader definition by placing it prior to the
  ///   #shader_type directives.
  /// * Allows #include directives to fetch other files.
  fn parse_kernels(source_code: &str) -> crate::Result<Vec<ShaderKernel>> {
    use crate::io::*;

    let mut result = Vec::with_capacity(2); // usually 2 shaders per file
    let mut shared_code = String::new();

    for line in source_code.lines() {
      if line.trim().starts_with("#shader_type") {
        // determine shader type
        let kind = match line.split_whitespace().nth(1) {
          Some("vertex") => ShaderKind::Vertex,
          Some("fragment") => ShaderKind::Fragment,
          Some("compute") => ShaderKind::Compute,
          _ => continue,
        };

        result.push(ShaderKernel {
          kind,
          code: shared_code.clone(),
        });
      } else if line.trim().starts_with("#include") {
        if let Some(path) = line.split_whitespace().nth(1) {
          // trim the fat from the include path
          let path = path.replace(['"', '"', ';'], "");

          // fetch and splat the dependent shader
          let dependent_file = VirtualPath::parse(&path);
          let dependent_code = dependent_file.read_all_text()?;

          if let Some(shader) = result.last_mut() {
            shader.code.push_str(&dependent_code);
          } else {
            shared_code.push_str(&dependent_code);
          }
        }
      } else if let Some(shader) = result.last_mut() {
        // build up the active shader unit
        shader.code.push_str(line);
        shader.code.push('\n');
      } else {
        // build up the shared code unit
        shared_code.push_str(line);
        shared_code.push('\n');
      };
    }

    Ok(result)
  }
}

/// Represents a single compiled shader program.
#[derive(Clone)]
pub struct ShaderProgram {
  state: Rc<RefCell<ShaderProgramState>>,
}

/// The internal state for a [`ShaderProgram`] .
struct ShaderProgramState {
  id: ShaderId,
  graphics: GraphicsServer,
  location_cache: FastHashMap<String, Option<usize>>,
}

impl ShaderProgram {
  /// Creates a new blank [`ShaderProgram`] on the GPU.
  pub fn new(graphics: &GraphicsServer) -> crate::Result<Self> {
    Ok(Self {
      state: Rc::new(RefCell::new(ShaderProgramState {
        id: graphics.shader_create()?,
        graphics: graphics.clone(),
        location_cache: FastHashMap::default(),
      })),
    })
  }

  /// Loads a [`ShaderProgram`] from the given raw shader code.
  pub fn from_code<S: ShaderLanguage>(
    graphics: &GraphicsServer,
    code: &str,
  ) -> crate::Result<Self> {
    let program = Self::new(graphics)?;

    program.load_code::<S>(code)?;

    Ok(program)
  }

  /// Loads a [`ShaderProgram`] from the given [`VirtualPath`] code.
  pub fn from_path<S: ShaderLanguage>(
    graphics: &GraphicsServer,
    path: impl Into<VirtualPath>,
  ) -> crate::Result<Self> {
    let path = path.into();
    let code = path.read_all_text()?;

    Self::from_code::<S>(graphics, &code)
  }

  /// Loads a [`ShaderProgram`] from the given raw GLSL shader code.
  pub fn from_glsl(graphics: &GraphicsServer, code: &str) -> crate::Result<Self> {
    Self::from_code::<GLSL>(graphics, code)
  }

  /// Loads a [`ShaderProgram`] from the given raw GLSL shader code file.
  pub fn from_glsl_path(
    graphics: &GraphicsServer,
    path: impl Into<VirtualPath>,
  ) -> crate::Result<Self> {
    Self::from_path::<GLSL>(graphics, path)
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

  /// Reloads the [`ShaderProgram`] from the given 'glsl' program code.
  pub fn load_code<S: ShaderLanguage>(&self, text: &str) -> crate::Result<()> {
    let state = self.state.borrow();
    let graphics = &state.graphics;
    let shaders = S::parse_kernels(text)?;

    graphics.shader_link(state.id, &shaders)?;

    Ok(())
  }

  /// Reloads the [`ShaderProgram`] from a file at the given virtual path.
  pub fn load_from_path<S: ShaderLanguage>(
    &self,
    path: impl Into<VirtualPath>,
  ) -> crate::Result<()> {
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
  pub graphics: GraphicsServer,
}

impl Asset for ShaderProgram {
  type Loader = ShaderProgramLoader;
}

impl AssetLoader<ShaderProgram> for ShaderProgramLoader {
  fn load(&self, context: &AssetContext) -> crate::Result<ShaderProgram> {
    let program = ShaderProgram::new(&self.graphics)?;
    let source_code = context.path.read_all_text()?;

    program.load_code::<GLSL>(&source_code)?;

    Ok(program)
  }
}

/// Implements uniform value transformation for common shader uniforms.
macro_rules! impl_uniform {
  ($type:ty, $value:ident) => {
    impl From<$type> for ShaderUniform {
      fn from(value: $type) -> Self {
        ShaderUniform::$value(value.clone().into())
      }
    }
  };
}

impl_uniform!(bool, Bool);
impl_uniform!(u32, U32);
impl_uniform!(f32, F32);
impl_uniform!(Degrees, F32);
impl_uniform!(Radians, F32);
impl_uniform!(Vec2, Vec2);
impl_uniform!(Vec3, Vec3);
impl_uniform!(Vec4, Vec4);
impl_uniform!(&Mat2, Mat2);
impl_uniform!(&Mat3, Mat3);
impl_uniform!(&Mat4, Mat4);
impl_uniform!(Color, Color);
impl_uniform!(Color32, Color32);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_glsl_source_should_build_valid_code() {
    let result = GLSL::parse_kernels(
      r"
      #version 330 core

      // shared code
      uniform mat4 u_projectionView;
      uniform vec2 u_resolution;
      uniform vec4 u_color;

      #shader_type vertex

      layout(location = 0) in vec2 a_position;
      layout(location = 1) in vec2 a_tex_coord;
      layout(location = 2) in vec4 a_color;

      out vec2 v_uv;
      out vec4 v_color;

      void main() {
        v_uv    = a_uv;
        v_color = a_color * u_color;

        gl_Position = vec4(a_position, 0.0, 1.0) * u_projectionView;
      }

      #shader_type fragment

      uniform sampler2d u_texture;

      in vec2 v_uv;
      in vec4 v_color;

      void main() {
        gl_FragColor = texture(u_texture, v_uv) * v_color;
      }
    ",
    )
    .expect("Failed to parse simple shader kernels");

    assert_eq!(result.len(), 2);

    assert_eq!(result[0].kind, ShaderKind::Vertex);
    assert!(result[0].code.trim().starts_with("#version 330 core"));
    assert!(result[0].code.contains("gl_Position"));

    assert_eq!(result[1].kind, ShaderKind::Fragment);
    assert!(result[1].code.trim().starts_with("#version 330 core"));
    assert!(result[1].code.contains("gl_FragColor"));

    println!("{result:#?}");
  }
}
