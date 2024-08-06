//! A graphics backend implementation based on wgpu.

use std::error::Error;

use common::{Color, Rectangle, UVec2};
use graphics::{
  BlendState, BufferError, BufferId, BufferKind, BufferUsage, CullingMode, GraphicsBackend, MemoryBarrier, MeshError,
  MeshId, PrimitiveTopology, ScissorMode, ShaderError, ShaderId, ShaderKernel, ShaderUniform, TargetError, TargetId,
  TextureError, TextureFilter, TextureFormat, TextureId, TextureSampler, VertexDescriptor,
};
use wgpu::{Backends, CompositeAlphaMode, InstanceDescriptor, InstanceFlags, StoreOp, SurfaceTargetUnsafe};

/// A wgpu-based graphics backend.
pub struct WgpuGraphicsBackend {
  instance: wgpu::Instance,
  adapter: wgpu::Adapter,
  device: wgpu::Device,
  queue: wgpu::Queue,
  surface: wgpu::Surface<'static>,
}

impl WgpuGraphicsBackend {
  /// Creates a new wgpu-based graphics backend.
  pub async fn new(window: &winit::window::Window) -> Result<Self, Box<dyn Error>> {
    // acquire the instance
    let instance = wgpu::Instance::new(InstanceDescriptor {
      backends: Backends::all(),
      flags: InstanceFlags::default(),
      ..InstanceDescriptor::default()
    });

    // acquire the adapter
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
      })
      .await
      .unwrap();

    // acquire the device and queue
    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          label: None,
          required_features: wgpu::Features::empty(),
          required_limits: wgpu::Limits::default(),
          memory_hints: wgpu::MemoryHints::default(),
        },
        None,
      )
      .await
      .unwrap();

    // build the surface (drop the lifetime bound on the window)
    let surface = unsafe {
      let surface_target = SurfaceTargetUnsafe::from_window(window)?;
      instance.create_surface_unsafe(surface_target)
    }?;

    let size = window.inner_size();

    surface.configure(&device, &wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: wgpu::TextureFormat::Bgra8Unorm,
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::AutoVsync,
      desired_maximum_frame_latency: 0,
      alpha_mode: CompositeAlphaMode::PostMultiplied,
      view_formats: Default::default(),
    });

    Ok(Self {
      instance,
      adapter,
      device,
      queue,
      surface,
    })
  }
}

impl GraphicsBackend for WgpuGraphicsBackend {
  fn begin_frame(&self) {
    // no-op
  }

  fn end_frame(&self) {
    // no-op
  }

  fn clear_color_buffer(&self, color: Color) {
    let surface = self.surface.get_current_texture().unwrap();
    let view = surface.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("Main encoder"),
    });

    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: None,
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Clear(wgpu::Color {
            r: color.r as f64,
            g: color.g as f64,
            b: color.b as f64,
            a: color.a as f64,
          }),
          store: StoreOp::Store,
        },
      })],
      depth_stencil_attachment: None,
      timestamp_writes: None,
      occlusion_query_set: None,
    });

    self.queue.submit(std::iter::once(encoder.finish()));

    surface.present();
  }

  fn clear_depth_buffer(&self, depth: f32) {
    todo!()
  }

  fn viewport_size(&self) -> (usize, usize) {
    todo!()
  }

  fn set_viewport_size(&self, size: UVec2) {
    // resize the surface
    self.surface.configure(&self.device, &wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: wgpu::TextureFormat::Bgra8Unorm,
      width: size.x,
      height: size.y,
      present_mode: wgpu::PresentMode::AutoVsync,
      desired_maximum_frame_latency: 0,
      alpha_mode: CompositeAlphaMode::PostMultiplied,
      view_formats: Default::default(),
    });
  }

  fn set_blend_state(&self, blend_state: BlendState) {
    todo!()
  }

  fn set_culling_mode(&self, culling_mode: CullingMode) {
    todo!()
  }

  fn set_scissor_mode(&self, scissor_mode: ScissorMode) {
    todo!()
  }

  fn buffer_create(&self) -> Result<BufferId, BufferError> {
    todo!()
  }

  fn buffer_read_data(
    &self,
    buffer: BufferId,
    offset: usize,
    length: usize,
    pointer: *mut u8,
  ) -> Result<(), BufferError> {
    todo!()
  }

  fn buffer_write_data(
    &self,
    buffer: BufferId,
    usage: BufferUsage,
    kind: BufferKind,
    length: usize,
    pointer: *const u8,
  ) -> Result<(), BufferError> {
    todo!()
  }

  fn buffer_delete(&self, buffer: BufferId) -> Result<(), BufferError> {
    todo!()
  }

  fn texture_create(&self, sampler: &TextureSampler) -> Result<TextureId, TextureError> {
    todo!()
  }

  fn texture_set_options(&self, texture: TextureId, sampler: &TextureSampler) -> Result<(), TextureError> {
    todo!()
  }

  fn texture_initialize(
    &self,
    texture: TextureId,
    width: u32,
    height: u32,
    format: TextureFormat,
  ) -> Result<(), TextureError> {
    todo!()
  }

  fn texture_read_data(
    &self,
    texture: TextureId,
    length: usize,
    pixel_format: TextureFormat,
    pixels: *mut u8,
    mip_level: usize,
  ) -> Result<(), TextureError> {
    todo!()
  }

  fn texture_write_data(
    &self,
    texture: TextureId,
    width: u32,
    height: u32,
    pixels: *const u8,
    internal_format: TextureFormat,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) -> Result<(), TextureError> {
    todo!()
  }

  fn texture_write_sub_data(
    &self,
    texture: TextureId,
    region: &Rectangle,
    pixels: *const u8,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) -> Result<(), TextureError> {
    todo!()
  }

  fn texture_delete(&self, texture: TextureId) -> Result<(), TextureError> {
    todo!()
  }

  fn shader_create(&self) -> Result<ShaderId, ShaderError> {
    todo!()
  }

  fn shader_link(&self, shader: ShaderId, kernels: &[ShaderKernel]) -> Result<(), ShaderError> {
    todo!()
  }

  fn shader_uniform_location(&self, shader: ShaderId, name: &str) -> Option<usize> {
    todo!()
  }

  fn shader_set_uniform(&self, shader: ShaderId, location: usize, value: &ShaderUniform) -> Result<(), ShaderError> {
    todo!()
  }

  fn shader_activate(&self, shader: ShaderId) -> Result<(), ShaderError> {
    todo!()
  }

  fn shader_dispatch_compute(&self, shader: ShaderId, x: u32, y: u32, z: u32) -> Result<(), ShaderError> {
    todo!()
  }

  fn shader_memory_barrier(&self, barrier: MemoryBarrier) -> Result<(), ShaderError> {
    todo!()
  }

  fn shader_delete(&self, shader: ShaderId) -> Result<(), ShaderError> {
    todo!()
  }

  fn mesh_create(
    &self,
    vertices: BufferId,
    indices: BufferId,
    descriptors: &[VertexDescriptor],
  ) -> Result<MeshId, MeshError> {
    todo!()
  }

  fn mesh_draw(
    &self,
    mesh: MeshId,
    topology: PrimitiveTopology,
    vertex_count: usize,
    index_count: usize,
  ) -> Result<(), MeshError> {
    todo!()
  }

  fn mesh_delete(&self, mesh: MeshId) -> Result<(), MeshError> {
    todo!()
  }

  fn target_create(
    &self,
    color_attachment: TextureId,
    depth_attachment: Option<TextureId>,
    stencil_attachment: Option<TextureId>,
  ) -> Result<TargetId, TargetError> {
    todo!()
  }

  fn target_activate(&self, target: TargetId) -> Result<(), TargetError> {
    todo!()
  }

  fn target_set_default(&self) -> Result<(), TargetError> {
    todo!()
  }

  fn target_blit_to_active(
    &self,
    target: TargetId,
    source_rect: Option<Rectangle>,
    dest_rect: Option<Rectangle>,
    filter: TextureFilter,
  ) -> Result<(), TargetError> {
    todo!()
  }

  fn target_delete(&self, target: TargetId) -> Result<(), TargetError> {
    todo!()
  }
}
