//! Shader loading and management.
//!
//! Shader programs form the programmable part of the GPU pipeline, outside of state changes,
//! and are managed through this module.
//!
//! For higher-level shader control see the material module instead.

use std::rc::Rc;

use crate::assets::{Asset, AssetContext, AssetLoader};
use crate::io::AsPath;
use crate::maths::{Matrix2x2, Matrix3x3, Matrix4x4, Vector2, Vector3, Vector4};

use super::*;

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
  Integer(u32),
  Floating(f32),
  Point2(Vector2<i32>),
  Point3(Vector3<i32>),
  Point4(Vector4<i32>),
  Vector2(Vector2<f32>),
  Vector3(Vector3<f32>),
  Vector4(Vector4<f32>),
  Matrix2x2(Matrix2x2),
  Matrix3x3(Matrix3x3),
  Matrix4x4(Matrix4x4),
  Texture(Texture, u8, Option<TextureSampler>),

  /// A special case of a texture uniform,
  /// used for binding a texture to a compute shader.
  TextureBinding(Texture, usize, TextureBindingMode, TextureFormat),
}

/// Represents a single compiled shader program.
#[derive(Clone)]
pub struct ShaderProgram {
  state: Rc<ShaderProgramState>,
}

/// The internal state for a [`ShaderProgram`] .
struct ShaderProgramState {
  graphics: GraphicsServer,
  handle: GraphicsHandle,
}

impl ShaderProgram {
  /// Creates a new blank [`ShaderProgram`] on the GPU.
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self {
      state: Rc::new(ShaderProgramState {
        graphics: graphics.clone(),
        handle: graphics.create_shader(),
      }),
    }
  }

  /// Loads a [`ShaderProgram`] from the given raw 'glsl' code.
  pub fn from_glsl(graphics: &GraphicsServer, code: &str) -> crate::Result<Self> {
    let program = Self::new(graphics);

    program.load_glsl(code)?;

    Ok(program)
  }

  /// Retrieves the binding location of the given shader uniform in the underlying program.
  pub fn get_uniform_location(&self, name: &str) -> Option<usize> {
    let graphics = &self.state.graphics;

    graphics.get_shader_uniform_location(self.state.handle, name)
  }

  /// Sets the given uniform value in the underlying program.
  pub fn set_uniform(&self, name: &str, value: &ShaderUniform) {
    if let Some(location) = self.get_uniform_location(name) {
      let graphics = &self.state.graphics;

      graphics.set_shader_uniform(self.state.handle, location, value);
    }
  }

  /// Sets the given uniform value in the underlying program.
  pub fn set_uniform_at(&self, location: usize, value: &ShaderUniform) {
    let graphics = &self.state.graphics;

    graphics.set_shader_uniform(self.state.handle, location, value);
  }

  /// Dispatches compute work to the GPU for this shader program.
  pub fn dispatch_compute(&self, x: u32, y: u32, z: u32) {
    let graphics = &self.state.graphics;

    graphics.dispatch_compute(self.state.handle, x, y, z);
  }

  /// Reloads the [`ShaderProgram`] from the given 'glsl' program code.
  pub fn load_glsl(&self, text: &str) -> crate::Result<()> {
    let graphics = &self.state.graphics;
    let shaders = parse_glsl_source(text)?;

    graphics.link_shaders(self.state.handle, shaders)?;

    Ok(())
  }

  /// Reloads the [`ShaderProgram`] from a file at the given virtual path.
  pub fn load_from_path(&self, path: impl AsPath) -> crate::Result<()> {
    let path = path.as_path();
    let source_code = path.read_all_text()?;

    self.load_glsl(&source_code)?;

    Ok(())
  }
}

impl GraphicsResource for ShaderProgram {
  /// Retrieves the handle for the given [`ShaderProgram`].
  fn handle(&self) -> GraphicsHandle {
    self.state.handle
  }
}

impl Drop for ShaderProgramState {
  /// Deletes the [`ShaderProgram`] from the GPU.
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
implement_uniform!(u32, Integer);
implement_uniform!(f32, Floating);
implement_uniform!(Vector2<i32>, Point2);
implement_uniform!(Vector3<i32>, Point3);
implement_uniform!(Vector4<i32>, Point4);
implement_uniform!(Vector2<f32>, Vector2);
implement_uniform!(Vector3<f32>, Vector3);
implement_uniform!(Vector4<f32>, Vector4);
implement_uniform!(&Matrix2x2, Matrix2x2);
implement_uniform!(&Matrix3x3, Matrix3x3);
implement_uniform!(&Matrix4x4, Matrix4x4);

/// Different read/write modes for shader texture bindings.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TextureBindingMode {
  ReadOnly,
  WriteOnly,
  ReadWrite,
}

/// A compute image allows bound access to a texture image from compute shaders.
#[derive(Clone)]
pub struct TextureBinding {
  pub texture: Texture,
  pub mode: TextureBindingMode,
  pub format: TextureFormat,
}

impl From<&Texture> for ShaderUniform {
  fn from(texture: &Texture) -> Self {
    ShaderUniform::Texture(texture.clone(), 0, None)
  }
}

impl From<TextureBinding> for ShaderUniform {
  fn from(image: TextureBinding) -> Self {
    ShaderUniform::TextureBinding(image.texture, 0, image.mode, image.format)
  }
}

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
