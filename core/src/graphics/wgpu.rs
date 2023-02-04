use super::*;
use crate::collections::ResourceStorage;

/// A [`GraphicsBackend`] implementation using the [`wgpu`] crate.
///
/// [`wgpu`]: https://crates.io/crates/wgpu
pub struct WgpuGraphicsBackend {
  _buffers: ResourceStorage<BufferId, WgpuBuffer>,
  _meshes: ResourceStorage<MeshId, WgpuMesh>,
  _shaders: ResourceStorage<ShaderId, WgpuShader>,
  textures: ResourceStorage<TextureId, WgpuTexture>,
}

struct WgpuBuffer {}
struct WgpuMesh {}
struct WgpuShader {}
struct WgpuTexture {}

impl WgpuGraphicsBackend {
  pub unsafe fn new(_window: &winit::window::Window, _vsync_enabled: bool, _samples: u8) -> crate::Result<Self> {
    Ok(Self {
      _buffers: ResourceStorage::default(),
      _meshes: ResourceStorage::default(),
      _shaders: ResourceStorage::default(),
      textures: ResourceStorage::default(),
    })
  }
}

#[allow(unused_variables)]
impl GraphicsBackend for WgpuGraphicsBackend {
  fn begin_frame(&self) {
    todo!()
  }

  fn end_frame(&self) {
    todo!()
  }

  fn clear_color_buffer(&self, color: Color) {
    todo!()
  }

  fn clear_depth_buffer(&self) {
    todo!()
  }

  fn viewport_size(&self) -> (usize, usize) {
    todo!()
  }

  fn set_viewport_size(&self, size: winit::dpi::PhysicalSize<u32>) {
    todo!()
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

  fn buffer_read_data(&self, buffer: BufferId, offset: usize, length: usize, pointer: *mut u8) -> Result<(), BufferError> {
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
    Ok(self.textures.insert(WgpuTexture {}))
  }

  fn texture_set_options(&self, texture: TextureId, sampler: &TextureSampler) -> Result<(), TextureError> {
    todo!()
  }

  fn texture_initialize(&self, texture: TextureId, width: u32, height: u32, format: TextureFormat) -> Result<(), TextureError> {
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
    self.textures.remove(texture);

    Ok(())
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

  fn shader_delete(&self, shader: ShaderId) -> Result<(), ShaderError> {
    todo!()
  }

  fn mesh_create(&self, vertices: BufferId, indices: BufferId, descriptors: &[VertexDescriptor]) -> Result<MeshId, MeshError> {
    todo!()
  }

  fn mesh_draw(&self, mesh: MeshId, topology: PrimitiveTopology, vertex_count: usize, index_count: usize) -> Result<(), MeshError> {
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

  fn target_blit(
    &self,
    from: TargetId,
    to: TargetId,
    source_rect: &Rectangle,
    dest_rect: &Rectangle,
    filter: TextureFilter,
  ) -> Result<(), TargetError> {
    todo!()
  }

  fn target_blit_to_display(
    &self,
    target: TargetId,
    source_rect: &Rectangle,
    dest_rect: &Rectangle,
    filter: TextureFilter,
  ) -> Result<(), TargetError> {
    todo!()
  }

  fn target_delete(&self, target: TargetId) -> Result<(), TargetError> {
    todo!()
  }
}
