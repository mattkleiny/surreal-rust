//! Headless graphics support for the engine.

use super::*;

/// A headless, no-op [`GraphicsServerBackend`].
#[derive(Default)]
pub struct HeadlessBackend {}

#[allow(unused_variables)]
impl GraphicsServerBackend for HeadlessBackend {
  fn shader_create(&self) -> surreal::Result<GraphicsId> {
    Ok(GraphicsId::default())
  }

  fn shader_set_code(&self, shader_id: GraphicsId, code: &str) -> surreal::Result<()> {
    Ok(())
  }

  fn shader_get_code(&self, shader_id: GraphicsId) -> surreal::Result<String> {
    Ok(String::from(""))
  }

  fn shader_set_metadata(&self, shader_id: GraphicsId, metadata: ShaderMetadata) -> surreal::Result<()> {
    Ok(())
  }

  fn shader_get_metadata(&self, shader_id: GraphicsId) -> surreal::Result<ShaderMetadata> {
    Ok(ShaderMetadata::default())
  }

  fn shader_delete(&self, shader_id: GraphicsId) -> surreal::Result<()> {
    Ok(())
  }

  fn material_create(&self) -> surreal::Result<GraphicsId> {
    Ok(GraphicsId::default())
  }

  fn material_set_shader(&self, material_id: GraphicsId, shader_id: GraphicsId) -> surreal::Result<()> {
    Ok(())
  }

  fn material_get_shader(&self, material_id: GraphicsId) -> surreal::Result<GraphicsId> {
    Ok(GraphicsId::default())
  }

  fn material_set_metadata(&self, material_id: GraphicsId, metadata: MaterialMetadata) -> surreal::Result<()> {
    Ok(())
  }

  fn material_get_metadata(&self, material_id: GraphicsId) -> surreal::Result<MaterialMetadata> {
    Ok(MaterialMetadata::default())
  }

  fn material_delete(&self, material_id: GraphicsId) -> surreal::Result<()> {
    Ok(())
  }

  fn mesh_create(&self) -> surreal::Result<GraphicsId> {
    Ok(GraphicsId::default())
  }

  fn mesh_get_surface_count(&self, mesh_id: GraphicsId) -> surreal::Result<usize> {
    Ok(1)
  }

  fn mesh_add_surface(&self, mesh_id: GraphicsId, surface_data: SurfaceData) -> surreal::Result<()> {
    Ok(())
  }

  fn mesh_get_surface(&self, mesh_id: GraphicsId, surface_index: usize) -> surreal::Result<SurfaceData> {
    Ok(SurfaceData::default())
  }

  fn mesh_get_surface_material(&self, mesh_id: GraphicsId, surface_index: usize) -> surreal::Result<GraphicsId> {
    Ok(GraphicsId::new(0))
  }

  fn mesh_set_surface_material(&self, mesh_id: GraphicsId, surface_index: usize, material_id: GraphicsId) -> surreal::Result<()> {
    Ok(())
  }

  fn mesh_clear(&self, mesh_id: GraphicsId) -> surreal::Result<()> {
    Ok(())
  }

  fn mesh_delete(&self, mesh_id: GraphicsId) -> surreal::Result<()> {
    Ok(())
  }

  fn light_create(&self, light_type: LightType) -> surreal::Result<GraphicsId> {
    Ok(GraphicsId::default())
  }

  fn light_get_type(&self, light_id: GraphicsId) -> surreal::Result<LightType> {
    Ok(LightType::Directional)
  }

  fn light_set_parameter(&self, light_id: GraphicsId, parameter: LightParameter) -> surreal::Result<()> {
    Ok(())
  }

  fn light_delete(&self, light_id: GraphicsId) -> surreal::Result<()> {
    Ok(())
  }
}
