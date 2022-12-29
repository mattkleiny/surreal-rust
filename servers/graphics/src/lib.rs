use core::graphics::PrimitiveTopology;
use core::maths::AABB;

#[cfg(feature = "hdrp")]
pub mod hdrp;
#[cfg(feature = "lwrp")]
pub mod lwrp;

pub type GraphicsId = core::utilities::RID;

#[derive(Clone)]
pub struct SurfaceData {
  pub topology: PrimitiveTopology,
  pub vertices: Vec<u8>,
  pub indices: Vec<u8>,
  pub aabb: AABB<f32>,
  pub material: GraphicsId,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LightType {
  Directional,
  Point,
  Spot,
}

#[derive(Copy, Clone, Debug)]
pub enum LightParameter {
  Color(core::graphics::Color),
  Intensity(f32),
  Size(f32),
  Range(f32),
  BakingMode(LightBakeMode),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LightBakeMode {
  Disabled,
  Static,
  Dynamic,
}

/// An abstraction on top of the underlying graphics system.
///
/// This is a high-level abstraction that makes use of 'opaque' [`GraphicsId`]
/// to hide away implementation details. The server is intended to be a low-level
/// implementation abstraction.
pub trait GraphicsServer {
  // shader operations
  fn shader_create(&self) -> core::Result<GraphicsId>;
  fn shader_set_code(&self, shader_id: GraphicsId, code: &str) -> core::Result<()>;
  fn shader_get_code(&self, shader_id: GraphicsId) -> core::Result<String>;
  fn shader_delete(&self, shader_id: GraphicsId) -> core::Result<()>;

  // material operations
  fn material_create(&self) -> core::Result<GraphicsId>;
  fn material_set_shader(&self, material_id: GraphicsId, shader_id: GraphicsId) -> core::Result<()>;
  fn material_delete(&self, material_id: GraphicsId) -> core::Result<()>;

  // mesh operations
  fn mesh_create(&self) -> core::Result<GraphicsId>;
  fn mesh_get_surface_count(&self, mesh_id: GraphicsId) -> core::Result<usize>;
  fn mesh_add_surface(&self, mesh_id: GraphicsId, surface_data: SurfaceData) -> core::Result<()>;
  fn mesh_get_surface(&self, mesh_id: GraphicsId, surface_index: usize) -> core::Result<SurfaceData>;
  fn mesh_get_surface_material(&self, mesh_id: GraphicsId, surface_index: usize) -> core::Result<GraphicsId>;
  fn mesh_set_surface_material(&self, mesh_id: GraphicsId, surface_index: usize, material_id: GraphicsId) -> core::Result<()>;
  fn mesh_clear(&self, mesh_id: GraphicsId) -> core::Result<()>;
  fn mesh_delete(&self, mesh_id: GraphicsId) -> core::Result<()>;

  // light operations
  fn light_create(&self, light_type: LightType) -> core::Result<GraphicsId>;
  fn light_get_type(&self, light_id: GraphicsId) -> core::Result<LightType>;
  fn light_set_parameter(&self, light_id: GraphicsId, parameter: LightParameter) -> core::Result<()>;
  fn light_delete(&self, light_id: GraphicsId) -> core::Result<()>;
}
