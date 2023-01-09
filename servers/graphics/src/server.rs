use std::ops::Deref;

use surreal::graphics::PrimitiveTopology;
use surreal::maths::AABB;

#[cfg(feature = "backend-headless")]
mod headless;
#[cfg(feature = "backend-opengl")]
mod opengl;
#[cfg(feature = "backend-vulkan")]
mod vulkan;

// A unique ID for graphics resources in the [`GraphicsServerBackend`].
surreal::impl_rid_type!(GraphicsId);

/// Storage for a resource, keyed by it's [`GraphicsId`].
#[derive(Default)]
struct GraphicsStorage<T> {
  entries: std::collections::HashMap<GraphicsId, T>,
}

impl<T> GraphicsStorage<T> {
  /// Retrieves an item from storage.
  pub fn get(&self, id: GraphicsId) -> Option<&T> {
    self.entries.get(&id)
  }

  /// Inserts an item into storage.
  pub fn insert(&mut self, id: GraphicsId, entry: T) {
    self.entries.insert(id, entry);
  }

  /// Removes an item from storage.
  pub fn remove(&mut self, id: GraphicsId) -> Option<T> {
    self.entries.remove(&id)
  }
}

/// Surface data used for mesh creation.
#[derive(Default, Clone)]
pub struct SurfaceData {
  pub topology: PrimitiveTopology,
  pub vertices: Vec<u8>,
  pub indices: Vec<u8>,
  pub aabb: AABB,
  pub material: GraphicsId,
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

/// The singleton graphics server implementation for the project.
///
/// All instructions to the graphics server should be sent through this facade.
/// Internally we delegate to the active [`GraphicsServerBackend`], which can
/// vary depending on the target platform.
pub struct GraphicsServer {
  backend: Box<dyn GraphicsServerBackend>,
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
  pub fn from_vulkan(window: winit::window::WindowBuilder, event_loop: &winit::event_loop::EventLoop<()>) -> surreal::Result<Self> {
    Ok(Self::from_backend(vulkan::VulkanBackend::new(window, event_loop)?))
  }

  /// Create a [`GraphicsServer`] from the given [`GraphicsServerBackend`].
  pub fn from_backend(backend: impl GraphicsServerBackend + 'static) -> Self {
    GraphicsServer {
      backend: Box::new(backend),
    }
  }
}

impl Deref for GraphicsServer {
  type Target = dyn GraphicsServerBackend;

  fn deref(&self) -> &Self::Target {
    self.backend.as_ref()
  }
}

/// An abstraction on top of the underlying graphics system.
///
/// This is a high-level abstraction that makes use of 'opaque' [`GraphicsId`]
/// to hide away implementation details. The server is intended to be a low-level
/// implementation abstraction.
///
/// Different render pipelines might offer different features and capabilities on
/// top of those exported here.
pub trait GraphicsServerBackend {
  // shader operations
  fn shader_create(&self) -> surreal::Result<GraphicsId>;
  fn shader_set_code(&self, shader_id: GraphicsId, code: &str) -> surreal::Result<()>;
  fn shader_get_code(&self, shader_id: GraphicsId) -> surreal::Result<String>;
  fn shader_set_metadata(&self, shader_id: GraphicsId, metadata: ShaderMetadata) -> surreal::Result<()>;
  fn shader_get_metadata(&self, shader_id: GraphicsId) -> surreal::Result<ShaderMetadata>;
  fn shader_delete(&self, shader_id: GraphicsId) -> surreal::Result<()>;

  // material operations
  fn material_create(&self) -> surreal::Result<GraphicsId>;
  fn material_set_shader(&self, material_id: GraphicsId, shader_id: GraphicsId) -> surreal::Result<()>;
  fn material_get_shader(&self, material_id: GraphicsId) -> surreal::Result<GraphicsId>;
  fn material_set_metadata(&self, material_id: GraphicsId, metadata: MaterialMetadata) -> surreal::Result<()>;
  fn material_get_metadata(&self, material_id: GraphicsId) -> surreal::Result<MaterialMetadata>;
  fn material_delete(&self, material_id: GraphicsId) -> surreal::Result<()>;

  // mesh operations
  fn mesh_create(&self) -> surreal::Result<GraphicsId>;
  fn mesh_get_surface_count(&self, mesh_id: GraphicsId) -> surreal::Result<usize>;
  fn mesh_add_surface(&self, mesh_id: GraphicsId, surface_data: SurfaceData) -> surreal::Result<()>;
  fn mesh_get_surface(&self, mesh_id: GraphicsId, surface_index: usize) -> surreal::Result<SurfaceData>;
  fn mesh_get_surface_material(&self, mesh_id: GraphicsId, surface_index: usize) -> surreal::Result<GraphicsId>;
  fn mesh_set_surface_material(&self, mesh_id: GraphicsId, surface_index: usize, material_id: GraphicsId) -> surreal::Result<()>;
  fn mesh_clear(&self, mesh_id: GraphicsId) -> surreal::Result<()>;
  fn mesh_delete(&self, mesh_id: GraphicsId) -> surreal::Result<()>;

  // light operations
  fn light_create(&self, light_type: LightType) -> surreal::Result<GraphicsId>;
  fn light_get_type(&self, light_id: GraphicsId) -> surreal::Result<LightType>;
  fn light_set_parameter(&self, light_id: GraphicsId, parameter: LightParameter) -> surreal::Result<()>;
  fn light_delete(&self, light_id: GraphicsId) -> surreal::Result<()>;
}
