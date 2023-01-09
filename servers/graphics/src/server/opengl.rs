//! OpenGL support for the engine.

use super::*;

/// A [`GraphicsServerBackend`] implementation for OpenGL.
pub struct OpenGLBackend {
  _context: glutin::ContextWrapper<glutin::PossiblyCurrent, ()>,
}

impl OpenGLBackend {
  /// Builds a new [`OpenGLBackend`] for the given raw window handles.
  pub fn new(_window: winit::window::WindowBuilder) -> surreal::Result<Self> {
    todo!();
  }
}

#[allow(unused_variables)]
impl GraphicsServerBackend for OpenGLBackend {
  fn shader_create(&self) -> surreal::Result<GraphicsId> {
    todo!()
  }

  fn shader_set_code(&self, shader_id: GraphicsId, code: &str) -> surreal::Result<()> {
    todo!()
  }

  fn shader_get_code(&self, shader_id: GraphicsId) -> surreal::Result<String> {
    todo!()
  }

  fn shader_set_metadata(&self, shader_id: GraphicsId, metadata: ShaderMetadata) -> surreal::Result<()> {
    todo!()
  }

  fn shader_get_metadata(&self, shader_id: GraphicsId) -> surreal::Result<ShaderMetadata> {
    todo!()
  }

  fn shader_delete(&self, shader_id: GraphicsId) -> surreal::Result<()> {
    todo!()
  }

  fn material_create(&self) -> surreal::Result<GraphicsId> {
    todo!()
  }

  fn material_set_shader(&self, material_id: GraphicsId, shader_id: GraphicsId) -> surreal::Result<()> {
    todo!()
  }

  fn material_get_shader(&self, material_id: GraphicsId) -> surreal::Result<GraphicsId> {
    todo!()
  }

  fn material_set_metadata(&self, material_id: GraphicsId, metadata: MaterialMetadata) -> surreal::Result<()> {
    todo!()
  }

  fn material_get_metadata(&self, material_id: GraphicsId) -> surreal::Result<MaterialMetadata> {
    todo!()
  }

  fn material_delete(&self, material_id: GraphicsId) -> surreal::Result<()> {
    todo!()
  }

  fn mesh_create(&self) -> surreal::Result<GraphicsId> {
    todo!()
  }

  fn mesh_get_surface_count(&self, mesh_id: GraphicsId) -> surreal::Result<usize> {
    todo!()
  }

  fn mesh_add_surface(&self, mesh_id: GraphicsId, surface_data: SurfaceData) -> surreal::Result<()> {
    todo!()
  }

  fn mesh_get_surface(&self, mesh_id: GraphicsId, surface_index: usize) -> surreal::Result<SurfaceData> {
    todo!()
  }

  fn mesh_get_surface_material(&self, mesh_id: GraphicsId, surface_index: usize) -> surreal::Result<GraphicsId> {
    todo!()
  }

  fn mesh_set_surface_material(&self, mesh_id: GraphicsId, surface_index: usize, material_id: GraphicsId) -> surreal::Result<()> {
    todo!()
  }

  fn mesh_clear(&self, mesh_id: GraphicsId) -> surreal::Result<()> {
    todo!()
  }

  fn mesh_delete(&self, mesh_id: GraphicsId) -> surreal::Result<()> {
    todo!()
  }

  fn light_create(&self, light_type: LightType) -> surreal::Result<GraphicsId> {
    todo!()
  }

  fn light_get_type(&self, light_id: GraphicsId) -> surreal::Result<LightType> {
    todo!()
  }

  fn light_set_parameter(&self, light_id: GraphicsId, parameter: LightParameter) -> surreal::Result<()> {
    todo!()
  }

  fn light_delete(&self, light_id: GraphicsId) -> surreal::Result<()> {
    todo!()
  }
}
