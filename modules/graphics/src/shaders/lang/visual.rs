use super::*;

/// The Visual [`ShaderLanguage`] implementation.
pub struct Visual;

impl ShaderProgram {
  /// Loads a [`ShaderProgram`] from the given Visual Shader blueprint.
  pub fn from_visual(graphics: &GraphicsEngine, code: &str) -> Result<Self, ShaderError> {
    Self::from_code::<Visual>(graphics, code)
  }

  /// Loads a [`ShaderProgram`] from the given raw Visual Shader file.
  pub fn from_visual_path<'a>(graphics: &GraphicsEngine, path: impl ToVirtualPath) -> Result<Self, ShaderError> {
    Self::from_path::<Visual>(graphics, path)
  }

  /// Loads a [`ShaderProgram`] from the given raw Visual Shader stream.
  pub fn from_visual_stream(graphics: &GraphicsEngine, stream: &mut dyn InputStream) -> Result<Self, ShaderError> {
    Self::from_stream::<Visual>(graphics, stream)
  }
}

impl ShaderLanguage for Visual {
  fn parse_kernels(_source_code: &str, _environment: &ShaderEnvironment) -> Result<Vec<ShaderKernel>, ShaderError> {
    todo!()
  }
}

#[allow(dead_code)]
mod definition {
  //! Internal representation of a Visual Shader.
  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  struct VisualShaderDefinition {
    name: String,
    description: Option<String>,
    inputs: Vec<VisualShaderInput>,
    outputs: Vec<VisualShaderOutput>,
    uniforms: Vec<VisualShaderUniform>,
    functions: Vec<VisualShaderFunction>,
    code: String,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  struct VisualShaderInput {
    name: String,
    kind: VisualShaderKind,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  struct VisualShaderOutput {
    name: String,
    kind: VisualShaderKind,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  struct VisualShaderUniform {
    name: String,
    kind: VisualShaderKind,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  struct VisualShaderFunction {
    name: String,
    inputs: Vec<VisualShaderInput>,
    outputs: Vec<VisualShaderOutput>,
    code: String,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  enum VisualShaderKind {
    Scalar(ScalarKind),
    Vector(VectorKind),
    Matrix(MatrixKind),
    Sampler(SamplerKind),
    Struct(StructKind),
    Array(ArrayKind),
    Void,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  enum ScalarKind {
    Float,
    Int,
    Bool,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  enum VectorKind {
    Float2,
    Float3,
    Float4,
    Int2,
    Int3,
    Int4,
    Bool2,
    Bool3,
    Bool4,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  enum MatrixKind {
    Float2x2,
    Float3x3,
    Float4x4,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  enum SamplerKind {
    Texture2D,
    Texture3D,
    TextureCube,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  struct StructKind {
    name: String,
    fields: Vec<VisualShaderField>,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  struct ArrayKind {
    kind: Box<VisualShaderKind>,
    size: usize,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
  struct VisualShaderField {
    name: String,
    kind: VisualShaderKind,
  }
}
