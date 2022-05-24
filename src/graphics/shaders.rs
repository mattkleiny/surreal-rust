use crate::graphics::{GraphicsServer, TextureSampler, GraphicsImpl};
use crate::io::{AsVirtualPath, FileResult};
use crate::maths::{Matrix2x2, Matrix3x3, Matrix4x4, Vector2, Vector3, Vector4};

use super::*;

/// Different types fo shaders supported by the engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

/// Defines a single shader kernel in a shader program.
pub struct Shader {
  pub kind: ShaderKind,
  pub code: String,
}

/// Representation of single value that can be used in a `Material`.
#[derive(Debug)]
pub enum ShaderUniform<G> where G: GraphicsImpl + ?Sized {
  Integer(u32),
  Floating(f32),
  Point2(Vector2<i32>),
  Point3(Vector3<i32>),
  Point4(Vector4<i32>),
  Vector2(Vector2<f32>),
  Vector3(Vector3<f32>),
  Vector4(Vector4<f32>),
  Matrix2x2(Matrix2x2<f32>),
  Matrix3x3(Matrix3x3<f32>),
  Matrix4x4(Matrix4x4<f32>),
  Texture(G::Handle, usize, Option<TextureSampler>),
}

/// Represents a single compiled shader program.
pub struct ShaderProgram<G> where G: GraphicsImpl {
  server: GraphicsServer<G>,
  pub handle: G::Handle,
}

impl<G> ShaderProgram<G> where G: GraphicsImpl {
  /// Creates a new blank shader program on the GPU.
  pub fn new(server: &GraphicsServer<G>) -> Self {
    Self {
      server: server.clone(),
      handle: server.create_shader(),
    }
  }

  /// Loads a shader program from the given raw code.
  pub fn from_string(server: &GraphicsServer<G>, code: &str) -> GraphicsResult<Self> {
    let program = Self::new(server);

    program.load_from_string(code)?;

    Ok(program)
  }

  /// Retrieves the binding location of the given shader uniform in the underlying program.
  pub fn get_uniform_location(&self, name: &str) -> Option<usize> {
    self.server.get_shader_uniform_location(self.handle, name)
  }

  /// Sets the given uniform value in the underlying program.
  pub fn set_uniform(&self, location: usize, value: &ShaderUniform<G>) {
    self.server.set_shader_uniform(self.handle, location, value);
  }

  /// Reloads the shader program from the given text.
  pub fn load_from_string(&self, text: &str) -> FileResult<()> {
    let shaders = parse_glsl_source(&text);

    self.server.link_shaders(self.handle, shaders)?;

    Ok(())
  }
  
  /// Reloads the shader program from a file at the given virtual path.
  pub fn load_from_path(&self, path: impl AsVirtualPath) -> FileResult<()> {
    let source_code = path.as_virtual_path().read_all_text()?;
    let shaders = parse_glsl_source(&source_code);

    self.server.link_shaders(self.handle, shaders)?;

    Ok(())
  }
}

impl<G> Drop for ShaderProgram<G> where G: GraphicsImpl {
  /// Deletes the shader program from the GPU.
  fn drop(&mut self) {
    self.server.delete_shader(self.handle);
  }
}

/// Implements uniform value transformation for common shader uniforms.
macro_rules! implement_uniform {
  ($type:ty, $value:ident) => {
    impl<G> Into<ShaderUniform<G>> for $type where G: GraphicsImpl {
      fn into(self) -> ShaderUniform<G> {
        ShaderUniform::$value(self)
      }
    }
  };
}

implement_uniform!(u32, Integer);
implement_uniform!(f32, Floating);
implement_uniform!(Vector2<i32>, Point2);
implement_uniform!(Vector3<i32>, Point3);
implement_uniform!(Vector4<i32>, Point4);
implement_uniform!(Vector2<f32>, Vector2);
implement_uniform!(Vector3<f32>, Vector3);
implement_uniform!(Vector4<f32>, Vector4);
implement_uniform!(Matrix2x2<f32>, Matrix2x2);
implement_uniform!(Matrix3x3<f32>, Matrix3x3);
implement_uniform!(Matrix4x4<f32>, Matrix4x4);

/// Parses the given raw GLSL source and performs some basic pre-processing.
///
/// Allows for the following basic transformations:
///
/// * Multiple shader types per file (separated with #shader_type directives).
/// * Shared code amongst each shader definition by placing it prior to the #shader_type directives.
fn parse_glsl_source(source: &str) -> Vec<Shader> {
  let mut result = Vec::with_capacity(2); // usually 2 shaders per file
  let mut shared_code = String::new();

  for line in source.lines() {
    if line.trim().starts_with("#shader_type") {
      // determine shader type
      let kind = match line.split_whitespace().nth(1) {
        Some("vertex") => ShaderKind::Vertex,
        Some("fragment") => ShaderKind::Fragment,
        _ => continue,
      };

      result.push(Shader { kind, code: shared_code.clone() });
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

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_should_process_valid_program() {
    let result = parse_glsl_source(r"
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
    ");

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].kind, ShaderKind::Vertex);
    assert!(result[0].code.trim().starts_with("#version 330 core"));
    assert_eq!(result[1].kind, ShaderKind::Fragment);
    assert!(result[1].code.trim().starts_with("#version 330 core"));
  }
}