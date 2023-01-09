use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

use surreal::graphics::PrimitiveTopology;
use surreal::maths::AABB;

#[cfg(feature = "backend-headless")]
mod headless;
#[cfg(feature = "backend-opengl")]
mod opengl;
#[cfg(feature = "backend-vulkan")]
mod vulkan;

surreal::impl_rid_type!(ShaderId);
surreal::impl_rid_type!(MaterialId);
surreal::impl_rid_type!(MeshId);
surreal::impl_rid_type!(LightId);

/// Surface data used for mesh creation.
#[derive(Default, Clone)]
pub struct SurfaceData {
  pub topology: PrimitiveTopology,
  pub vertices: Vec<u8>,
  pub indices: Vec<u8>,
  pub aabb: AABB,
  pub material: MaterialId,
}

/// Different kinds of lights supported.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LightType {
  Directional,
  Point,
  Spot,
}

/// Parameters that can be set on lights.
#[derive(Copy, Clone, Debug)]
pub enum LightParameter {
  Color(surreal::graphics::Color),
  Color32(surreal::graphics::Color32),
  Intensity(f32),
  Size(f32),
  Range(f32),
  BakingMode(LightBakeMode),
}

/// Baking modes for lights.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LightBakeMode {
  Disabled,
  Static,
  Dynamic,
}

/// Metadata for a shader program.
#[derive(Default, Clone)]
pub struct ShaderMetadata {
  pub name: String,
}

/// Metadata for a material.
#[derive(Default, Clone)]
pub struct MaterialMetadata {
  pub name: String,
  pub description: String,
  pub queue: MaterialQueue,
}

/// Different possible queues for a material.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MaterialQueue {
  #[default]
  Opaque,
  Transparent,
  Compute,
}

pub enum UniformValue {
  F32(f32),
  F64(f64),
  Vec2,
  Vec3,
}

/// The singleton graphics server implementation for the project.
///
/// All instructions to the graphics server should be sent through this facade.
/// Internally we delegate to the active [`GraphicsServerBackend`], which can
/// vary depending on the target platform.
pub struct GraphicsServer {
  backend: Arc<dyn GraphicsServerBackend>,
}

impl GraphicsServer {
  /// Creates a [`GraphicsServer`] for a Headless, no-op backend.
  #[cfg(feature = "backend-headless")]
  pub fn from_headless() -> Self {
    Self::from_backend(headless::HeadlessBackend::default())
  }

  /// Creates a [`GraphicsServer`] for OpenGL.
  #[cfg(feature = "backend-opengl")]
  pub fn from_opengl(window: winit::window::WindowBuilder) -> surreal::Result<Self> {
    Ok(Self::from_backend(opengl::OpenGLBackend::new(window)?))
  }

  /// Creates a [`GraphicsServer`] for Vulkan.
  #[cfg(feature = "backend-vulkan")]
  pub fn from_vulkan(window: Arc<winit::window::Window>) -> surreal::Result<Self> {
    Ok(Self::from_backend(vulkan::VulkanBackend::new(window)?))
  }

  /// Create a [`GraphicsServer`] from the given [`GraphicsServerBackend`].
  pub fn from_backend(backend: impl GraphicsServerBackend + 'static) -> Self {
    GraphicsServer {
      backend: Arc::new(backend),
    }
  }
}

impl Deref for GraphicsServer {
  type Target = dyn GraphicsServerBackend;

  fn deref(&self) -> &Self::Target {
    self.backend.as_ref()
  }
}

/// An abstraction on top of the underlying graphics API.
///
/// This is a high-level abstraction that makes use of 'opaque' [`GraphicsId`]
/// to hide away implementation details. The server is intended to be a low-level
/// implementation abstraction.
///
/// This achieves a number of goals for us. In particular:
///
/// * It allows us to depend on abstractions instead of concretions; important since
///   graphics API landscape continues to change, especially in Rust.
/// * It allows us to build an API that spans lifetime requirements. Whilst some API
///   methods will be
pub trait GraphicsServerBackend {
  // general operations
  fn begin_frame(&self);
  fn end_frame(&self);

  // shader operations
  fn shader_create(&self) -> surreal::Result<ShaderId>;
  fn shader_set_code(&self, shader_id: ShaderId, code: &str) -> surreal::Result<()>;
  fn shader_get_code(&self, shader_id: ShaderId) -> surreal::Result<String>;
  fn shader_set_metadata(&self, shader_id: ShaderId, metadata: ShaderMetadata) -> surreal::Result<()>;
  fn shader_get_metadata(&self, shader_id: ShaderId) -> surreal::Result<ShaderMetadata>;
  fn shader_delete(&self, shader_id: ShaderId) -> surreal::Result<()>;

  // material operations
  fn material_create(&self) -> surreal::Result<MaterialId>;
  fn material_set_shader(&self, material_id: MaterialId, shader_id: MaterialId) -> surreal::Result<()>;
  fn material_get_shader(&self, material_id: MaterialId) -> surreal::Result<MaterialId>;
  fn material_set_metadata(&self, material_id: MaterialId, metadata: MaterialMetadata) -> surreal::Result<()>;
  fn material_get_metadata(&self, material_id: MaterialId) -> surreal::Result<MaterialMetadata>;
  fn material_set_uniform(&self, material_id: MaterialId, uniform_name: &str, value: &UniformValue) -> surreal::Result<()>;
  fn material_get_uniform(&self, material_id: MaterialId, uniform_name: &str) -> surreal::Result<Option<UniformValue>>;
  fn material_delete(&self, material_id: MaterialId) -> surreal::Result<()>;

  // mesh operations
  fn mesh_create(&self) -> surreal::Result<MeshId>;
  fn mesh_get_surface_count(&self, mesh_id: MeshId) -> surreal::Result<usize>;
  fn mesh_add_surface(&self, mesh_id: MeshId, surface_data: SurfaceData) -> surreal::Result<()>;
  fn mesh_get_surface(&self, mesh_id: MeshId, surface_index: usize) -> surreal::Result<SurfaceData>;
  fn mesh_get_surface_material(&self, mesh_id: MeshId, surface_index: usize) -> surreal::Result<MeshId>;
  fn mesh_set_surface_material(&self, mesh_id: MeshId, surface_index: usize, material_id: MeshId) -> surreal::Result<()>;
  fn mesh_clear(&self, mesh_id: MeshId) -> surreal::Result<()>;
  fn mesh_delete(&self, mesh_id: MeshId) -> surreal::Result<()>;

  // light operations
  fn light_create(&self, light_type: LightType) -> surreal::Result<LightId>;
  fn light_get_type(&self, light_id: LightId) -> surreal::Result<LightType>;
  fn light_set_parameter(&self, light_id: LightId, parameter: LightParameter) -> surreal::Result<()>;
  fn light_delete(&self, light_id: LightId) -> surreal::Result<()>;
}
