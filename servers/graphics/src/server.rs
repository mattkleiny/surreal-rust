use surreal::graphics::Color;
use utilities::*;

mod headless;
mod wgpu;

mod utilities;

surreal::impl_rid!(ShaderId);
surreal::impl_rid!(MaterialId);
surreal::impl_rid!(MeshId);
surreal::impl_rid!(LightId);

/// The singleton graphics server implementation for the project.
///
/// All instructions to the graphics server should be sent through this facade.
/// Internally we delegate to the active [`GraphicsServerBackend`], which can
/// vary depending on the target platform.
pub struct GraphicsServer {
  backend: std::sync::Arc<dyn GraphicsServerBackend>,
}

impl GraphicsServer {
  /// Creates a [`GraphicsServer`] for a Headless, no-op backend.
  pub fn from_headless() -> Self {
    Self::from_backend(headless::HeadlessBackend::default())
  }

  /// Creates a [`GraphicsServer`] for WGPU.
  pub async fn from_wgpu(window: &winit::window::Window) -> surreal::Result<Self> {
    Ok(Self::from_backend(wgpu::WgpuBackend::new(window).await?))
  }

  /// Create a [`GraphicsServer`] from the given [`GraphicsServerBackend`].
  pub fn from_backend(backend: impl GraphicsServerBackend + 'static) -> Self {
    GraphicsServer {
      backend: std::sync::Arc::new(backend),
    }
  }
}

impl std::ops::Deref for GraphicsServer {
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
  fn begin_frame(&self, color: Color) -> surreal::Result<()>;
  fn end_frame(&self) -> surreal::Result<()>;
  fn resize_viewport(&self, new_size: winit::dpi::PhysicalSize<u32>) -> surreal::Result<()>;

  // shader operations
  fn shader_create(&self, name: Option<&str>) -> surreal::Result<ShaderId>;
  fn shader_set_code(&self, shader_id: ShaderId, code: &str) -> surreal::Result<()>;
  fn shader_delete(&self, shader_id: ShaderId) -> surreal::Result<()>;

  // material operations
  // fn material_create(&self) -> surreal::Result<MaterialId>;
  // fn material_set_shader(&self, material_id: MaterialId, shader_id: MaterialId) -> surreal::Result<()>;
  // fn material_get_shader(&self, material_id: MaterialId) -> surreal::Result<MaterialId>;
  // fn material_set_metadata(&self, material_id: MaterialId, metadata: MaterialMetadata) -> surreal::Result<()>;
  // fn material_get_metadata(&self, material_id: MaterialId) -> surreal::Result<MaterialMetadata>;
  // fn material_set_uniform(&self, material_id: MaterialId, uniform_name: &str, value: &UniformValue) -> surreal::Result<()>;
  // fn material_get_uniform(&self, material_id: MaterialId, uniform_name: &str) -> surreal::Result<Option<UniformValue>>;
  // fn material_delete(&self, material_id: MaterialId) -> surreal::Result<()>;
  //
  // // mesh operations
  // fn mesh_create(&self) -> surreal::Result<MeshId>;
  // fn mesh_get_surface_count(&self, mesh_id: MeshId) -> surreal::Result<usize>;
  // fn mesh_add_surface(&self, mesh_id: MeshId, surface_data: SurfaceData) -> surreal::Result<()>;
  // fn mesh_get_surface(&self, mesh_id: MeshId, surface_index: usize) -> surreal::Result<SurfaceData>;
  // fn mesh_get_surface_material(&self, mesh_id: MeshId, surface_index: usize) -> surreal::Result<MeshId>;
  // fn mesh_set_surface_material(&self, mesh_id: MeshId, surface_index: usize, material_id: MeshId) -> surreal::Result<()>;
  // fn mesh_clear(&self, mesh_id: MeshId) -> surreal::Result<()>;
  // fn mesh_delete(&self, mesh_id: MeshId) -> surreal::Result<()>;
  //
  // // light operations
  // fn light_create(&self, light_type: LightType) -> surreal::Result<LightId>;
  // fn light_get_type(&self, light_id: LightId) -> surreal::Result<LightType>;
  // fn light_set_parameter(&self, light_id: LightId, parameter: LightParameter) -> surreal::Result<()>;
  // fn light_delete(&self, light_id: LightId) -> surreal::Result<()>;
}
