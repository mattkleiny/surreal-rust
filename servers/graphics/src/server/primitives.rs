//! Command buffer for the graphics server.

use surreal::graphics::TextureFormat;

surreal::impl_rid!(ShaderId);
surreal::impl_rid!(MaterialId);
surreal::impl_rid!(MeshId);
surreal::impl_rid!(TextureId);
surreal::impl_rid!(RenderTargetId);

/// A possible value for a uniform in a shader program.
pub enum UniformValue {
  Float(f32),
  Vec2([f32; 2]),
  Vec3([f32; 3]),
  Vec4([f32; 4]),
  Mat2([f32; 2 * 2]),
  Mat3([f32; 3 * 3]),
  Mat4([f32; 4 * 4]),
  Texture(TextureId),
}

/// A descriptor for how to build a shader in the [`GraphicsBackend`].
pub struct ShaderDescriptor {
  pub label: Option<&'static str>,
  pub shader_code: &'static str,
}

/// A descriptor for how to build a material in the [`GraphicsBackend`].
pub struct MaterialDescriptor {
  pub label: Option<&'static str>,
  pub shader_id: ShaderId,
}

/// A descriptor for how to build a texture in the [`GraphicsBackend`].
pub struct TextureDescriptor {
  pub label: Option<&'static str>,
  pub size: (u32, u32, u32),
  pub format: TextureFormat,
}
