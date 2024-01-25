use super::*;

/// The OpenGL [`ShaderLanguage`] implementation.
pub struct GLSL;

impl ShaderProgram {
  /// Loads a [`ShaderProgram`] from the given raw GLSL shader code.
  pub fn from_glsl(graphics: &GraphicsEngine, code: &str) -> Result<Self, ShaderError> {
    Self::from_code::<GLSL>(graphics, code)
  }

  /// Loads a [`ShaderProgram`] from the given raw GLSL shader code file.
  pub fn from_glsl_path<'a>(graphics: &GraphicsEngine, path: impl ToVirtualPath) -> Result<Self, ShaderError> {
    Self::from_path::<GLSL>(graphics, path)
  }

  /// Loads a [`ShaderProgram`] from the given raw GLSL stream.
  pub fn from_glsl_stream(graphics: &GraphicsEngine, stream: &mut dyn InputStream) -> Result<Self, ShaderError> {
    Self::from_stream::<GLSL>(graphics, stream)
  }
}

impl ShaderLanguage for GLSL {
  /// Parses the given raw GLSL source and performs some basic pre-processing.
  ///
  /// Allows for the following basic transformations:
  ///
  /// * Multiple shader types per file (separated with #shader_type directives).
  /// * Shared code amongst each shader definition by placing it prior to the
  ///   #shader_type directives.
  /// * Allows #include directives to fetch other files.
  fn parse_kernels(source_code: &str) -> Result<Vec<ShaderKernel>, ShaderError> {
    use common::*;

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
          let dependent_file = path.to_virtual_path();
          let dependent_code = dependent_file
            .read_all_text()
            .map_err(|_| ShaderError::InvalidInclude)?;

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

      varying vec2 v_uv;
      varying vec4 v_color;

      void main() {
        v_uv    = a_uv;
        v_color = a_color * u_color;

        gl_Position = vec4(a_position, 0.0, 1.0) * u_projectionView;
      }

      #shader_type fragment

      uniform sampler2d u_texture;

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
