//! Vulkan support for the engine.

use std::sync::Arc;

use vulkano::device::physical::PhysicalDeviceType;
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags};
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::swapchain::{Swapchain, SwapchainCreateInfo};
use vulkano::VulkanLibrary;
use winit::window::Window;

use super::*;

/// A [`GraphicsServerBackend`] implementation for Vulkan.
pub struct VulkanBackend {
  instance: Arc<Instance>,
  device: Arc<Device>,
  queue: Arc<Queue>,
  swapchain: Arc<Swapchain>,
  images: Vec<Arc<SwapchainImage>>,
  allocator: StandardMemoryAllocator,
}

impl VulkanBackend {
  /// Creates a [`VulkanBackend`] for the given window.
  pub fn new(window: winit::window::WindowBuilder, event_loop: &winit::event_loop::EventLoop<()>) -> surreal::Result<Self> {
    use vulkano_win::VkSurfaceBuild;

    let library = VulkanLibrary::new()?;
    let extensions = vulkano_win::required_extensions(&library);

    let instance = Instance::new(
      library,
      InstanceCreateInfo {
        enabled_extensions: extensions,
        enumerate_portability: true,
        ..Default::default()
      },
    )?;

    let surface = window.build_vk_surface(&event_loop, instance.clone())?;

    let window = surface
      .object()
      .ok_or(surreal::anyhow!("Unable to access main window"))?
      .downcast_ref::<Window>()
      .ok_or(surreal::anyhow!("Unable to access main window"))?;

    let device_extensions = DeviceExtensions {
      khr_swapchain: true,
      ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = instance
      .enumerate_physical_devices()?
      .filter(|it| it.supported_extensions().contains(&device_extensions))
      .filter_map(|it| {
        it.queue_family_properties()
          .iter()
          .enumerate()
          .position(|(i, it)| it.queue_flags.intersects(QueueFlags::GRAPHICS) && it.surface_support(i as u32, &surface).unwrap_or(false))
          .map(|i| (it, i as u32))
      })
      .min_by_key(|(p, _)| match p.properties().device_type {
        PhysicalDeviceType::DiscreteGpu => 0,
        PhysicalDeviceType::IntegratedGpu => 1,
        PhysicalDeviceType::VirtualGpu => 2,
        PhysicalDeviceType::Cpu => 3,
        PhysicalDeviceType::Other => 4,
        _ => 5,
      })
      .ok_or(surreal::anyhow!("No suitable physical device found"))?;

    surreal::diagnostics::trace!(
      "Using Vulkan device: {} (type: {:?})",
      physical_device.properties().device_name,
      physical_device.properties().device_type
    );

    let (device, mut queues) = Device::new(
      physical_device,
      DeviceCreateInfo {
        enabled_extensions: device_extensions,
        queue_create_infos: vec![QueueCreateInfo {
          queue_family_index,
          ..Default::default()
        }],
        ..Default::default()
      },
    )?;

    let queue = queues.next().ok_or(surreal::anyhow!("No suitable queue found"))?;

    let (mut swapchain, images) = {
      let surface_capabilities = device
        .physical_device()
        .surface_capabilities(&surface, Default::default())
        .ok_or(surreal::anyhow!("Unable to find surface capabilities"))?;

      let image_format = Some(device.physical_device().surface_formats(&surface, Default::default())?[0].0);

      Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
          min_image_count: surface_capabilities.min_image_count,
          image_format,
          image_extent: window.inner_size().into(),
          image_usage: ImageUsage::COLOR_ATTACHMENT,
          // composite_alpha: surface_capabilities.supported_composite_alpha.into_iter().next()?,
          ..Default::default()
        },
      )?
    };

    let allocator = StandardMemoryAllocator::new_default(device.clone());

    Ok(Self {
      instance,
      device,
      queue,
      swapchain,
      images,
      allocator,
    })
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
