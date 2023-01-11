use std::ops::Range;

mod headless;
mod utilities;
mod wgpu;

/// Possible kinds of [`GraphicsServerBackend`]s.
pub enum GraphicsBackendKind {
  Headless,
  WGPU,
}

/// The singleton graphics server implementation for the project.
///
/// All instructions to the graphics server should be sent through this facade.
/// Internally we delegate to the active [`GraphicsServerBackend`], which can
/// vary depending on the target platform.
#[derive(Clone)]
pub struct GraphicsServer {
  backend: std::sync::Arc<dyn GraphicsServerBackend>,
}

impl GraphicsServer {
  /// Create a [`GraphicsServer`] from the given [`GraphicsServerBackend`].
  pub fn from_backend(backend: impl GraphicsServerBackend + 'static) -> Self {
    GraphicsServer {
      backend: std::sync::Arc::new(backend),
    }
  }

  /// Creates a [`GraphicsServer`] for a Headless, no-op backend.
  pub fn from_headless() -> Self {
    Self::from_backend(headless::HeadlessBackend::default())
  }

  /// Creates a [`GraphicsServer`] for WGPU.
  pub async fn from_wgpu(window: &winit::window::Window) -> surreal::Result<Self> {
    Ok(Self::from_backend(wgpu::WgpuBackend::new(window).await?))
  }

  /// Creates a [`GraphicsServer`] for the given [`GraphicsBackendKind`].
  pub async fn from_kind(kind: GraphicsBackendKind, window: &winit::window::Window) -> surreal::Result<Self> {
    match kind {
      GraphicsBackendKind::Headless => Ok(Self::from_headless()),
      GraphicsBackendKind::WGPU => Self::from_wgpu(window).await,
    }
  }
}

impl std::ops::Deref for GraphicsServer {
  type Target = dyn GraphicsServerBackend;

  fn deref(&self) -> &Self::Target {
    self.backend.as_ref()
  }
}

/// A buffer of [`Command`]s for execution in the [`GraphicsServerBackend`].
#[derive(Default, Clone)]
pub struct CommandBuffer {
  commands: Vec<Command>,
}

impl CommandBuffer {
  /// Enqueues a [`Command`] to the buffer.
  pub fn enqueue(&mut self, command: Command) {
    self.commands.push(command);
  }

  /// Dequeues a [`Command`] to the buffer.
  pub fn dequeue(&mut self) -> Option<Command> {
    self.commands.pop()
  }
}

/// A single command in a [`CommandBuffer`].
#[derive(Clone)]
pub enum Command {
  /// Performs an indirect draw with the given material.
  DrawIndirect {
    material_id: MaterialId,
    vertices: Range<u32>,
    instances: Range<u32>,
  },
}

/// An abstraction on top of the underlying graphics API.
///
/// This is a high-level abstraction that makes use of 'opaque' [`RID`] to hide away implementation
/// details. The server is intended to be a mid-level implementation abstraction.
pub trait GraphicsServerBackend {
  /// Executes the given [`CommandBuffer`] against the backend.
  ///
  /// This is the main entry point for the graphics server.
  /// All commands will be drained from the [`CommandBuffer`] and executed in sequence.
  fn execute_commands(&self, commands: &mut CommandBuffer) -> surreal::Result<()>;

  /// Notifies the backend that the main viewport has resized to a new physical size.
  fn resize_viewport(&self, new_size: winit::dpi::PhysicalSize<u32>) -> surreal::Result<()>;

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

surreal::impl_rid!(ShaderId);
surreal::impl_rid!(MaterialId);
surreal::impl_rid!(MeshId);
surreal::impl_rid!(LightId);
