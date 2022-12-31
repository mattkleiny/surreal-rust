//! A cross-platform graphics engine for Surreal.
//!
//! The engine is split into different 'pipelines' to allow specific targeting
//! of different project goals.

use std::ops::Deref;

use surreal::graphics::PrimitiveTopology;
use surreal::maths::AABB;

#[cfg(feature = "hdrp")]
pub mod hdrp;
#[cfg(feature = "lwrp")]
pub mod lwrp;

#[cfg(feature = "opengl")]
mod opengl;

/// A unique [`surreal::utilities::RID`] for graphics resources.
pub type GraphicsId = surreal::utilities::RID;

/// The singleton graphics server implementation for the project.
///
/// All instructions to the graphics server should be sent through this facade.
/// Internally we delegate to the active [`GraphicsServerBackend`], which can
/// vary depending on the target platform.
struct GraphicsServer {
  backend: Box<dyn GraphicsServerBackend>,
}

// TODO: improve this?
surreal::impl_singleton!(GraphicsServer);

impl Default for GraphicsServer {
  fn default() -> Self {
    Self {
      backend: Box::new(opengl::OpenGLBackend::default()),
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
  fn shader_delete(&self, shader_id: GraphicsId) -> surreal::Result<()>;

  // material operations
  fn material_create(&self) -> surreal::Result<GraphicsId>;
  fn material_set_shader(&self, material_id: GraphicsId, shader_id: GraphicsId) -> surreal::Result<()>;
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

/// Surface data used for mesh creation.
#[derive(Clone)]
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
