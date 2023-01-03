//! OpenGL support for the engine.

use super::*;
use raw_window_handle::RawWindowHandle;

/// A [`GraphicsServerBackend`] implementation for OpenGL.
pub struct OpenGLBackend {
  context: glutin::ContextWrapper<glutin::PossiblyCurrent, ()>,
}

impl OpenGLBackend {
  /// Builds a new [`OpenGLBackend`] for the given raw window handles.
  pub fn new(window: &(impl HasRawWindowHandle + HasRawDisplayHandle)) -> surreal::Result<Self> {
    use glutin::platform::windows::RawContextExt;

    let context = unsafe {
      glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_gl_debug_flag(true)
        .build_raw_context(match window.raw_window_handle() {
          RawWindowHandle::Win32(handle) => handle.hwnd,
          RawWindowHandle::WinRt(handle) => handle.core_window,
          _ => surreal::bail!("Unsupported window handle"),
        })?
        .make_current()
        .map_err(|(_, err)| err)?
    };

    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    Ok(Self { context })
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

  fn shader_delete(&self, shader_id: GraphicsId) -> surreal::Result<()> {
    todo!()
  }

  fn material_create(&self) -> surreal::Result<GraphicsId> {
    todo!()
  }

  fn material_set_shader(&self, material_id: GraphicsId, shader_id: GraphicsId) -> surreal::Result<()> {
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
