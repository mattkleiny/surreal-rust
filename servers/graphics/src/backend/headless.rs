//! Headless graphics support for the engine.

use std::sync::atomic::AtomicU64;

use super::*;

/// A headless, no-op [`GraphicsServerBackend`].
#[derive(Default)]
pub struct HeadlessBackend {
  next_shader_id: AtomicU64,
  next_material_id: AtomicU64,
  next_mesh_id: AtomicU64,
  next_light_id: AtomicU64,
}

#[allow(unused_variables)]
impl GraphicsServerBackend for HeadlessBackend {
  fn shader_create(&self) -> surreal::Result<GraphicsId> {
    Ok(GraphicsId::new(self.next_shader_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed)))
  }

  fn shader_set_code(&self, shader_id: GraphicsId, code: &str) -> surreal::Result<()> {
    Ok(())
  }

  fn shader_get_code(&self, shader_id: GraphicsId) -> surreal::Result<String> {
    Ok(String::from(""))
  }

  fn shader_delete(&self, shader_id: GraphicsId) -> surreal::Result<()> {
    Ok(())
  }

  fn material_create(&self) -> surreal::Result<GraphicsId> {
    Ok(GraphicsId::new(self.next_material_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed)))
  }

  fn material_set_shader(&self, material_id: GraphicsId, shader_id: GraphicsId) -> surreal::Result<()> {
    Ok(())
  }

  fn material_delete(&self, material_id: GraphicsId) -> surreal::Result<()> {
    Ok(())
  }

  fn mesh_create(&self) -> surreal::Result<GraphicsId> {
    Ok(GraphicsId::new(self.next_mesh_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed)))
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
    Ok(GraphicsId::new(self.next_light_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed)))
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