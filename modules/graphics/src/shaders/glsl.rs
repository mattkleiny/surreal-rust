//! GLSL language support for the shader system

use super::*;

/// The OpenGL [`ShaderLanguage`] implementation.
pub struct GlslShaderLanguage;

impl ShaderLanguage for GlslShaderLanguage {
  /// Parses the given raw GLSL source and performs some basic pre-processing.
  ///
  /// Allows for the following basic transformations:
  ///
  /// * Multiple shader types per file (separated with #shader_type directives).
  /// * Shared code amongst each shader definition by placing it prior to the
  ///   #shader_type directives.
  /// * Allows #include directives to fetch other files.
  fn parse_kernels(source_code: &str) -> surreal::Result<Vec<ShaderKernel>> {
    use surreal::io::*;

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
          let dependent_file = VirtualPath::from(&path);
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
