use std::{borrow::Cow, ops::Range};

pub use primitives::*;
use surreal::graphics::{Color, TextureFormat};

mod headless;
mod primitives;
mod wgpu;

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
  /// Creates a [`GraphicsServer`] for a Headless, no-op backend.
  pub fn from_headless() -> Self {
    Self::from_backend(headless::HeadlessGraphicsBackend::default())
  }

  /// Creates a [`GraphicsServer`] for WGPU.
  pub async fn from_wgpu(window: &winit::window::Window) -> surreal::Result<Self> {
    Ok(Self::from_backend(wgpu::WgpuGraphicsBackend::new(window).await?))
  }

  /// Create a [`GraphicsServer`] from the given [`GraphicsBackend`].
  pub fn from_backend(backend: impl GraphicsBackend + 'static) -> Self {
    GraphicsServer {
      backend: std::sync::Arc::new(backend),
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
  label: Option<&'a str>,
  commands: Vec<Command<'a>>,
}

impl<'a> CommandBuffer<'a> {
  /// Creates a new [`CommandBuffer`].
  pub fn new(label: &'a str) -> Self {
    Self {
      label: Some(label),
      commands: Vec::new(),
    }
  }

  /// Enqueues a [`Command`] to the buffer.
  pub fn enqueue(&mut self, command: Command<'a>) {
    self.commands.push(command);
  }

  /// Dequeues a [`Command`] to the buffer.
  pub fn dequeue(&mut self) -> Option<Command<'a>> {
    self.commands.pop()
  }
}

/// A single command in a [`CommandBuffer`].
pub enum Command<'a> {
  /// Reads the contents of the given [`TextureId`] into a [`Vec`] of [`u8`].
  ReadTexturePixels { texture_id: TextureId, pixels: &'a mut Vec<u8> },
  /// Writes the given [`Vec`] of [`u8`] pixel data into the given
  /// [`TextureId`].
  WriteTexturePixels { texture_id: TextureId, pixels: &'a [u8] },
  /// Sets the view matrix on the underlying pipeline.
  SetViewMatrix { view_matrix: [f32; 4 * 4] },
  /// Sets the projection matrix on the underlying pipeline.
  SetProjectionMatrix { projection_matrix: [f32; 4 * 4] },
  /// Sets the given global [`UniformValue`] for all materials.
  SetGlobalUniform {
    uniform_name: &'a str,
    uniform_value: UniformValue,
  },
  /// Sets the given viewport size on the underlying pipeline.
  SetViewport { viewport_size: winit::dpi::PhysicalSize<u32> },
  /// Sets the given render target as the active one for rendering.
  SetRenderTarget {
    render_target_id: Option<RenderTargetId>,
    clear_color: Option<Color>,
    depth_value: Option<f32>,
  },
  /// Begins sampling command information in the profiling system with the given
  /// name.
  BeginSample { sample_name: Cow<'a, str> },
  /// Stops sampling command information in the profiling system with the given
  /// name.
  EndSample { sample_name: Cow<'a, str> },
  /// Draws a mesh with the given material and optional material properties.
  DrawMesh {
    mesh_id: MeshId,
    material_id: MaterialId,
    material_props: &'a [UniformValue],
    sub_mesh_index: usize,
  },
  /// Performs an indirect draw call with the given material and vertex/index
  /// counts.
  DrawIndirect {
    material_id: MaterialId,
    vertices: Range<u32>,
    instances: Range<u32>,
  },
}

/// An abstraction on top of the underlying graphics API.
///
/// This is a mid-level abstraction that makes use of 'opaque' resource IDs to
/// hide away implementation details and lifetimes. The backend forms the
/// foundation of higher-level abstractions that make it simpler to build
/// graphics programs.
pub trait GraphicsBackend {
  /// Executes the given [`CommandBuffer`] against the backend.
  fn execute_commands(&self, commands: &mut CommandBuffer) -> surreal::Result<()>;

  /// Notifies the backend that the main viewport has resized to a new physical size.
  fn resize_viewport(&self, new_size: winit::dpi::PhysicalSize<u32>) -> surreal::Result<()>;

  // shader operations
  fn shader_create(&self, descriptor: &ShaderDescriptor) -> surreal::Result<ShaderId>;
  fn shader_delete(&self, shader_id: ShaderId) -> surreal::Result<()>;

  // material operations
  fn material_create(&self, descriptor: &MaterialDescriptor) -> surreal::Result<MaterialId>;
  fn material_delete(&self, material_id: MaterialId) -> surreal::Result<()>;

  // mesh operations
  // fn mesh_create(&self) -> surreal::Result<MeshId>;
  // fn mesh_delete(&self, mesh_id: MeshId) -> surreal::Result<()>;

  // texture operations
  fn texture_create(&self, descriptor: &TextureDescriptor) -> surreal::Result<TextureId>;
  fn texture_delete(&self, texture_id: TextureId) -> surreal::Result<()>;

  // render target operations
  fn render_target_create(&self, label: Option<&str>, size: (u32, u32), format: TextureFormat) -> surreal::Result<RenderTargetId>;
  fn render_target_delete(&self, render_target_id: RenderTargetId) -> surreal::Result<()>;

  // TODO: lighting
  // TODO: sdf
  // TODO: skybox
}
