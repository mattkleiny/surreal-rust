//! Headless graphics support for the engine.

use super::*;
use winit::dpi::PhysicalSize;

/// A headless, no-op [`GraphicsServerBackend`].
#[derive(Default)]
pub struct HeadlessBackend {}

#[allow(unused_variables)]
impl GraphicsServerBackend for HeadlessBackend {
  fn begin_frame(&self) -> surreal::Result<()> {
    todo!()
  }

  fn end_frame(&self) -> surreal::Result<()> {
    todo!()
  }

  fn resize_viewport(&mut self, new_size: PhysicalSize<u32>) -> surreal::Result<()> {
    todo!()
  }

  fn shader_create(&self) -> surreal::Result<ShaderId> {
    todo!()
  }

  fn shader_set_code(&self, shader_id: ShaderId, code: &str) -> surreal::Result<()> {
    todo!()
  }

  fn shader_get_code(&self, shader_id: ShaderId) -> surreal::Result<String> {
    todo!()
  }

  fn shader_set_metadata(&self, shader_id: ShaderId, metadata: ShaderMetadata) -> surreal::Result<()> {
    todo!()
  }

  fn shader_get_metadata(&self, shader_id: ShaderId) -> surreal::Result<ShaderMetadata> {
    todo!()
  }

  fn shader_delete(&self, shader_id: ShaderId) -> surreal::Result<()> {
    todo!()
  }

  fn material_create(&self) -> surreal::Result<MaterialId> {
    todo!()
  }

  fn material_set_shader(&self, material_id: MaterialId, shader_id: MaterialId) -> surreal::Result<()> {
    todo!()
  }

  fn material_get_shader(&self, material_id: MaterialId) -> surreal::Result<MaterialId> {
    todo!()
  }

  fn material_set_metadata(&self, material_id: MaterialId, metadata: MaterialMetadata) -> surreal::Result<()> {
    todo!()
  }

  fn material_get_metadata(&self, material_id: MaterialId) -> surreal::Result<MaterialMetadata> {
    todo!()
  }

  fn material_set_uniform(&self, material_id: MaterialId, uniform_name: &str, value: &UniformValue) -> surreal::Result<()> {
    todo!()
  }

  fn material_get_uniform(&self, material_id: MaterialId, uniform_name: &str) -> surreal::Result<Option<UniformValue>> {
    todo!()
  }

  fn material_delete(&self, material_id: MaterialId) -> surreal::Result<()> {
    todo!()
  }

  fn mesh_create(&self) -> surreal::Result<MeshId> {
    todo!()
  }

  fn mesh_get_surface_count(&self, mesh_id: MeshId) -> surreal::Result<usize> {
    todo!()
  }

  fn mesh_add_surface(&self, mesh_id: MeshId, surface_data: SurfaceData) -> surreal::Result<()> {
    todo!()
  }

  fn mesh_get_surface(&self, mesh_id: MeshId, surface_index: usize) -> surreal::Result<SurfaceData> {
    todo!()
  }

  fn mesh_get_surface_material(&self, mesh_id: MeshId, surface_index: usize) -> surreal::Result<MeshId> {
    todo!()
  }

  fn mesh_set_surface_material(&self, mesh_id: MeshId, surface_index: usize, material_id: MeshId) -> surreal::Result<()> {
    todo!()
  }

  fn mesh_clear(&self, mesh_id: MeshId) -> surreal::Result<()> {
    todo!()
  }

  fn mesh_delete(&self, mesh_id: MeshId) -> surreal::Result<()> {
    todo!()
  }

  fn light_create(&self, light_type: LightType) -> surreal::Result<LightId> {
    todo!()
  }

  fn light_get_type(&self, light_id: LightId) -> surreal::Result<LightType> {
    todo!()
  }

  fn light_set_parameter(&self, light_id: LightId, parameter: LightParameter) -> surreal::Result<()> {
    todo!()
  }

  fn light_delete(&self, light_id: LightId) -> surreal::Result<()> {
    todo!()
  }
}
