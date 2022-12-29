//! Shader loading and management.
//!
//! Shader programs form the programmable part of the GPU pipeline, outside of state changes,
//! and are managed through this module.
//!
//! For higher-level shader control see the material module instead.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use smallvec::SmallVec;

use crate::assets::{Asset, AssetContext, AssetLoader};
use crate::maths::{Mat2, Mat3, Mat4, Vec2, Vec3, Vec4};

use super::*;

use crate::io::VirtualPath;
pub use compiler::*;

mod compiler;

/// Different types of shaders supported by the engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
  Compute,
}

/// Defines a single shader kernel in a shader program.
pub struct Shader {
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
  Texture(Texture, u8, Option<TextureSampler>),
  TextureArray(SmallVec<[(Texture, u8); MAX_TEXTURE_UNITS]>, Option<TextureSampler>),
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

/// Represents a single compiled shader program.
#[derive(Clone)]
pub struct ShaderProgram {
  state: Rc<RefCell<ShaderProgramState>>,
}

/// The internal state for a [`ShaderProgram`] .
struct ShaderProgramState {
  graphics: GraphicsServer,
  handle: GraphicsHandle,
  location_cache: HashMap<String, Option<usize>>,
}

impl ShaderProgram {
  /// Creates a new blank [`ShaderProgram`] on the GPU.
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self {
      state: Rc::new(RefCell::new(ShaderProgramState {
        graphics: graphics.clone(),
        handle: graphics.create_shader(),
        location_cache: HashMap::new(),
      })),
    }
  }

  /// Compiles a [`ShaderProgram`] from the given raw string.
  pub fn compile<S: ShaderLanguage>(graphics: &GraphicsServer, source: &str) -> crate::Result<Self> {
    let shaders = S::compile_shader(source)?;

    Self::from_shaders(graphics, &shaders)
  }

  /// Loads a [`ShaderProgram`] from the given raw 'glsl' code.
  pub fn from_glsl(graphics: &GraphicsServer, code: &str) -> crate::Result<Self> {
    let program = Self::new(graphics);

    program.load_glsl(code)?;

    Ok(program)
  }

  /// Loads a [`ShaderProgram`] from the given discrete [`Shader`] pieces.
  pub fn from_shaders(graphics: &GraphicsServer, shaders: &[Shader]) -> crate::Result<Self> {
    let program = Self::new(graphics);

    program.load_shaders(shaders)?;

    Ok(program)
  }

  /// Retrieves the binding location of the given shader uniform in the underlying program.
  #[profiling::function]
  pub fn get_uniform_location(&self, name: &str) -> Option<usize> {
    let state = self.state.borrow();

    if let Some(location) = state.location_cache.get(name) {
      return location.to_owned();
    }

    drop(state);

    let mut state = self.state.borrow_mut();
    let graphics = &state.graphics;
    let location = graphics.get_shader_uniform_location(state.handle, name);

    state.location_cache.insert(name.to_string(), location);

    location
  }

  /// Sets the given uniform value in the underlying program.
  #[profiling::function]
  pub fn set_uniform(&self, name: &str, value: &ShaderUniform) {
    if let Some(location) = self.get_uniform_location(name) {
      let state = self.state.borrow();
      let graphics = &state.graphics;

      graphics.set_shader_uniform(state.handle, location, value);
    }
  }

  /// Dispatches compute work to the GPU for this shader program.
  #[profiling::function]
  pub fn dispatch_compute(&self, x: u32, y: u32, z: u32) {
    let state = self.state.borrow();
    let graphics = &state.graphics;

    graphics.dispatch_compute(state.handle, x, y, z);
  }

  /// Reloads the [`ShaderProgram`] from the given 'glsl' program code.
  pub fn load_glsl(&self, text: &str) -> crate::Result<()> {
    let state = self.state.borrow();
    let graphics = &state.graphics;
    let shaders = parse_glsl_source(text)?;

    graphics.link_shaders(state.handle, &shaders)?;

    Ok(())
  }

  /// Reloads the [`ShaderProgram`] from the given [`Shader`] pieces.
  pub fn load_shaders(&self, shaders: &[Shader]) -> crate::Result<()> {
    let state = self.state.borrow();
    let graphics = &state.graphics;

    graphics.link_shaders(state.handle, shaders)?;

    Ok(())
  }

  /// Reloads the [`ShaderProgram`] from a file at the given virtual path.
  pub fn load_from_path(&self, path: impl Into<VirtualPath>) -> crate::Result<()> {
    let path = path.into();
    let source_code = path.read_all_text()?;

    self.load_glsl(&source_code)?;

    Ok(())
  }
}

impl GraphicsResource for ShaderProgram {
  fn handle(&self) -> GraphicsHandle {
    self.state.borrow().handle
  }
}

impl Drop for ShaderProgramState {
  fn drop(&mut self) {
    self.graphics.delete_shader(self.handle);
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
    let program = ShaderProgram::new(&self.graphics);
    let source_code = context.path.read_all_text()?;

    program.load_glsl(&source_code)?;

    Ok(program)
  }
}

/// Implements uniform value transformation for common shader uniforms.
macro_rules! implement_uniform {
  ($type:ty, $value:ident) => {
    impl From<$type> for ShaderUniform {
      fn from(value: $type) -> Self {
        ShaderUniform::$value(value.clone())
      }
    }
  };
}

implement_uniform!(bool, Bool);
implement_uniform!(u32, U32);
implement_uniform!(f32, F32);
implement_uniform!(Vec2, Vec2);
implement_uniform!(Vec3, Vec3);
implement_uniform!(Vec4, Vec4);
implement_uniform!(&Mat2, Mat2);
implement_uniform!(&Mat3, Mat3);
implement_uniform!(&Mat4, Mat4);

/// Parses the given raw GLSL source and performs some basic pre-processing.
///
/// Allows for the following basic transformations:
///
/// * Multiple shader types per file (separated with #shader_type directives).
/// * Shared code amongst each shader definition by placing it prior to the #shader_type directives.
/// * Allows #include directives to fetch other files.
fn parse_glsl_source(source: &str) -> crate::Result<Vec<Shader>> {
  use crate::io::*;

  let mut result = Vec::with_capacity(2); // usually 2 shaders per file
  let mut shared_code = String::new();

  for line in source.lines() {
    if line.trim().starts_with("#shader_type") {
      // determine shader type
      let kind = match line.split_whitespace().nth(1) {
        Some("vertex") => ShaderKind::Vertex,
        Some("fragment") => ShaderKind::Fragment,
        Some("compute") => ShaderKind::Compute,
        _ => continue,
      };

      result.push(Shader {
        kind,
        code: shared_code.clone(),
      });
    } else if line.trim().starts_with("#include") {
      if let Some(path) = line.split_whitespace().nth(1) {
        // trim the fat from the include path
        let path = path.replace('"', "").replace('"', "").replace(';', "");

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_glsl_source_should_build_valid_code() {
    let result = parse_glsl_source(
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
    .expect("Failed to parse simple program");

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].kind, ShaderKind::Vertex);
    assert!(result[0].code.trim().starts_with("#version 330 core"));
    assert!(result[0].code.contains("gl_Position"));
    assert_eq!(result[1].kind, ShaderKind::Fragment);
    assert!(result[1].code.trim().starts_with("#version 330 core"));
    assert!(result[1].code.contains("gl_Frag"));
  }
}