use std::borrow::Cow;
use std::ops::Range;

use surreal::graphics::{Color, TextureFormat};

mod headless;
mod wgpu;

/// Possible kinds of [`GraphicsBackend`]s.
pub enum GraphicsBackendKind {
  Headless,
  WGPU,
}

/// The singleton graphics server implementation for the project.
///
/// All instructions to the graphics server should be sent through this facade.
/// Internally we delegate to the active [`GraphicsBackend`], which can
/// vary depending on the target platform.
#[derive(Clone)]
pub struct GraphicsServer {
  backend: std::sync::Arc<dyn GraphicsBackend>,
}

impl GraphicsServer {
  /// Create a [`GraphicsServer`] from the given [`GraphicsBackend`].
  pub fn from_backend(backend: impl GraphicsBackend + 'static) -> Self {
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
  pub async fn from_kind(window: &winit::window::Window, kind: GraphicsBackendKind) -> surreal::Result<Self> {
    match kind {
      GraphicsBackendKind::Headless => Ok(Self::from_headless()),
      GraphicsBackendKind::WGPU => Self::from_wgpu(window).await,
    }
  }
}

unsafe impl Send for GraphicsServer {}
unsafe impl Sync for GraphicsServer {}

impl std::ops::Deref for GraphicsServer {
  type Target = dyn GraphicsBackend;

  fn deref(&self) -> &Self::Target {
    self.backend.as_ref()
  }
}

/// A buffer of [`Command`]s for execution in the [`GraphicsBackend`].
#[derive(Default)]
pub struct CommandBuffer<'a> {
  _label: Option<&'a str>,
  commands: Vec<Command<'a>>,
}

impl<'a> CommandBuffer<'a> {
  /// Enqueues a [`Command`] to the buffer.
  pub fn enqueue(&mut self, command: Command<'a>) {
    self.commands.push(command);
  }

  /// Dequeues a [`Command`] to the buffer.
  pub fn dequeue(&mut self) -> Option<Command<'a>> {
    self.commands.pop()
  }
}

/// Builder pattern for [`CommandBuffer`]s.
#[must_use]
#[derive(Default)]
pub struct CommandBufferBuilder<'a> {
  label: Option<&'a str>,
  capacity: Option<usize>,
}

impl<'a> CommandBufferBuilder<'a> {
  /// Creates a new [`CommandBufferBuilder`].
  pub fn with_label(mut self, label: &'a str) -> Self {
    self.label = Some(label);
    self
  }

  /// Sets the capacity of the [`CommandBuffer`] to be built.
  pub fn with_capacity(mut self, capacity: usize) -> Self {
    self.capacity = Some(capacity);
    self
  }

  /// Builds the resultant [`CommandBuffer`].
  pub fn build(self) -> CommandBuffer<'a> {
    CommandBuffer {
      _label: self.label,
      commands: Vec::with_capacity(self.capacity.unwrap_or(0)),
    }
  }
}

/// A single command in a [`CommandBuffer`].
pub enum Command<'a> {
  /// Reads the contents of the given [`TextureId`] into a [`Vec`] of [`u8`].
  ReadTexturePixels { texture_id: TextureId, pixels: &'a mut Vec<u8> },
  /// Writes the given [`Vec`] of [`u8`] pixel data into the given [`TextureId`].
  WriteTexturePixels { texture_id: TextureId, pixels: &'a [u8] },
  /// Sets the view matrix on the underlying pipeline.
  SetViewMatrix { view_matrix: [f32; 4 * 4] },
  /// Sets the projection matrix on the underlying pipeline.
  SetProjectionMatrix { projection_matrix: [f32; 4 * 4] },
  /// Sets the given global [`UniformValue`] for all materials.
  SetGlobalUniform {
    uniform_name: &'a str,
    uniform_value: UniformValue<'a>,
  },
  /// Sets the given viewport size on the underlying pipeline.
  SetViewport { viewport_size: winit::dpi::PhysicalSize<u32> },
  /// Sets the given render target as the active one for rendering.
  SetRenderTarget {
    render_target_id: Option<RenderTargetId>,
    clear_color: Option<Color>,
    depth_value: Option<f32>,
  },
  /// Begins sampling command information in the profiling system with the given name.
  BeginSample { sample_name: Cow<'a, str> },
  /// Stops sampling command information in the profiling system with the given name.
  EndSample { sample_name: Cow<'a, str> },
  /// Draws a mesh with the given material and optional material properties.
  DrawMesh {
    mesh_id: MeshId,
    material_id: MaterialId,
    material_props: &'a [UniformValue<'a>],
    sub_mesh_index: usize,
  },
  /// Performs an indirect draw call with the given material and vertex/index counts.
  DrawIndirect {
    material_id: MaterialId,
    vertices: Range<u32>,
    instances: Range<u32>,
  },
}

/// A possible value for a uniform in a [`Command`].
pub enum UniformValue<'a> {
  Float(f32),
  Vec2([f32; 2]),
  Vec3([f32; 3]),
  Vec4([f32; 4]),
  Mat2(&'a [f32; 2 * 2]),
  Mat3(&'a [f32; 3 * 3]),
  Mat4(&'a [f32; 4 * 4]),
  Texture(TextureId),
}

/// A descriptor for how to build a material in the [`GraphicsBackend`].
pub struct MaterialDescriptor {
  pub label: Option<&'static str>,
  pub shader_code: &'static str,
}

/// A descriptor for how to build a texture in the [`GraphicsBackend`].
pub struct TextureDescriptor {
  pub label: Option<&'static str>,
  pub size: (u32, u32, u32),
  pub format: TextureFormat,
}

/// An abstraction on top of the underlying graphics API.
///
/// This is a mid-level abstraction that makes use of 'opaque' resource IDs to hide away
/// implementation details and lifetimes. The backend forms the foundation of higher-level
/// abstractions that make it simpler to build graphics programs.
pub trait GraphicsBackend {
  /// Executes the given [`CommandBuffer`] against the backend.
  ///
  /// All commands will be drained from the [`CommandBuffer`] and executed in sequence.
  fn execute_commands(&self, commands: &mut CommandBuffer) -> surreal::Result<()>;

  /// Notifies the backend that the main viewport has resized to a new physical size.
  fn resize_viewport(&self, new_size: winit::dpi::PhysicalSize<u32>) -> surreal::Result<()>;

  // material operations
  fn material_create(&self, descriptor: &MaterialDescriptor) -> surreal::Result<MaterialId>;
  fn material_set_uniform(&self, material_id: MaterialId, uniform_name: &str, value: &UniformValue) -> surreal::Result<()>;
  fn material_get_uniform(&self, material_id: MaterialId, uniform_name: &str) -> surreal::Result<Option<UniformValue>>;
  fn material_delete(&self, material_id: MaterialId) -> surreal::Result<()>;

  // // mesh operations
  // fn mesh_create(&self) -> surreal::Result<MeshId>;
  // fn mesh_get_surface_count(&self, mesh_id: MeshId) -> surreal::Result<usize>;
  // fn mesh_add_surface(&self, mesh_id: MeshId, surface_data: SurfaceData) -> surreal::Result<()>;
  // fn mesh_get_surface(&self, mesh_id: MeshId, surface_index: usize) -> surreal::Result<SurfaceData>;
  // fn mesh_get_surface_material(&self, mesh_id: MeshId, surface_index: usize) -> surreal::Result<MeshId>;
  // fn mesh_set_surface_material(&self, mesh_id: MeshId, surface_index: usize, material_id: MeshId) -> surreal::Result<()>;
  // fn mesh_clear(&self, mesh_id: MeshId) -> surreal::Result<()>;
  // fn mesh_delete(&self, mesh_id: MeshId) -> surreal::Result<()>;

  // texture operations
  fn texture_create(&self, descriptor: &TextureDescriptor) -> surreal::Result<TextureId>;
  fn texture_read(&self, texture_id: TextureId) -> surreal::Result<Box<[u8]>>;
  fn texture_write(&self, texture_id: TextureId, pixels: &[u8]) -> surreal::Result<()>;
  fn texture_delete(&self, texture_id: TextureId) -> surreal::Result<()>;

  // render target operations
  fn render_target_create(&self, label: Option<&str>, size: (u32, u32), format: TextureFormat) -> surreal::Result<RenderTargetId>;
  fn render_target_delete(&self, render_target_id: RenderTargetId) -> surreal::Result<()>;

  // TODO: lighting
  // TODO: sdf
  // TODO: skybox
}

surreal::impl_rid!(MaterialId);
surreal::impl_rid!(TextureId);
surreal::impl_rid!(MeshId);
surreal::impl_rid!(RenderTargetId);
