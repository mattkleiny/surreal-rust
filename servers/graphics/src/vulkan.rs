//! Vulkan support for the engine.

use ash::vk;
use std::ffi::CString;

use super::*;

/// A [`GraphicsServerBackend`] implementation for Vulkan.
pub struct VulkanBackend {
  entry: ash::Entry,
  instance: ash::Instance,
  surface: ash::extensions::khr::Surface,
}

impl VulkanBackend {
  /// Creates a [`VulkanBackend`] for the given window.
  pub fn new(window: &(impl HasRawWindowHandle + HasRawDisplayHandle)) -> surreal::Result<Self> {
    unsafe {
      let entry = ash::Entry::load()?;

      let app_info = vk::ApplicationInfo::builder()
        .application_name(CString::new("Surreal Engine").unwrap().as_c_str())
        .application_version(vk::make_version(0, 1, 0))
        .engine_name(CString::new("Surreal Engine").unwrap().as_c_str())
        .engine_version(vk::make_version(0, 1, 0))
        .api_version(vk::make_version(1, 2, 0))
        .build();

      let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        ..Default::default()
      };

      let instance = entry.create_instance(&create_info, None)?;
      let surface = ash::extensions::khr::Surface::new(&entry, &instance);

      Ok(Self { entry, instance, surface })
    }
  }
}

#[allow(unused_variables)]
impl GraphicsServerBackend for VulkanBackend {
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
