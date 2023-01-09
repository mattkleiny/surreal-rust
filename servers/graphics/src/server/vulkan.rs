//! Vulkan support for the engine.

use std::cell::RefCell;
use std::sync::Arc;

use vulkano::{
  device::{physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags},
  image::{ImageUsage, SwapchainImage},
  instance::{Instance, InstanceCreateInfo},
  memory::allocator::StandardMemoryAllocator,
  render_pass::RenderPass,
  swapchain::{acquire_next_image, AcquireError},
  swapchain::{Swapchain, SwapchainCreateInfo},
  sync::{self, GpuFuture},
  VulkanLibrary,
};
use winit::window::Window;

use super::*;

/// A [`GraphicsServerBackend`] implementation for Vulkan.
pub struct VulkanBackend {
  _window: Arc<Window>,
  _instance: Arc<Instance>,
  _device: Arc<Device>,
  _queue: Arc<Queue>,
  _images: Vec<Arc<SwapchainImage>>,
  _render_pass: Arc<RenderPass>,
  _allocator: StandardMemoryAllocator,
  state: RefCell<VulkanState>,
}

/// Internal state for hte [`VulkanBackend`].
struct VulkanState {
  swapchain: Arc<Swapchain>,
  previous_frame_end: Option<Box<dyn GpuFuture>>,
  swapchain_needs_rebuild: bool,
}

impl VulkanBackend {
  /// Creates a [`VulkanBackend`] for the given window.
  pub fn new(window: Arc<Window>) -> surreal::Result<Self> {
    // prepare vulkan instance
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

    // convert window to vulkan surface and retain it
    let surface = vulkano_win::create_surface_from_winit(window.clone(), instance.clone())?;

    let device_extensions = DeviceExtensions {
      khr_swapchain: true,
      ..DeviceExtensions::empty()
    };

    // query for the best physical device; prefer discrete GPUs
    let (physical_device, queue_family_index) = instance
      .enumerate_physical_devices()?
      .filter(|it| it.supported_extensions().contains(&device_extensions))
      .filter_map(|device| {
        device
          .queue_family_properties()
          .iter()
          .enumerate()
          .position(|(i, it)| {
            it.queue_flags.intersects(&QueueFlags {
              graphics: true,
              ..QueueFlags::default()
            }) && device.surface_support(i as u32, &surface).unwrap_or(false)
          })
          .map(|i| (device, i as u32))
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

    // build the vulkan device and prepare it's queues
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

    // build the main swapchain and get default swapchain images
    let (swapchain, images) = {
      let surface_capabilities = device.physical_device().surface_capabilities(&surface, Default::default())?;
      let image_format = Some(device.physical_device().surface_formats(&surface, Default::default())?[0].0);

      Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
          min_image_count: surface_capabilities.min_image_count,
          image_format,
          image_extent: window.inner_size().into(),
          image_usage: ImageUsage {
            color_attachment: true,
            ..ImageUsage::default()
          },
          ..Default::default()
        },
      )?
    };

    // prepare allocator and main render pass
    let allocator = StandardMemoryAllocator::new_default(device.clone());

    let render_pass = vulkano::single_pass_renderpass!(
      device.clone(),
      attachments: {
        color: {
          load: Clear,
          store: Store,
          format: swapchain.image_format(),
          samples: 1,
        }
      },
      pass: {
        color: [color],
        depth_stencil: {}
      }
    )?;

    let previous_frame_end = Some(sync::now(device.clone()).boxed());

    Ok(Self {
      _window: window,
      _instance: instance,
      _device: device,
      _queue: queue,
      _images: images,
      _allocator: allocator,
      _render_pass: render_pass,
      state: RefCell::new(VulkanState {
        swapchain,
        previous_frame_end,
        swapchain_needs_rebuild: false,
      }),
    })
  }
}

#[allow(unused_variables)]
impl GraphicsServerBackend for VulkanBackend {
  fn begin_frame(&self) {
    let mut state = self.state.borrow_mut();
    let dimensions = self._window.inner_size();

    state.previous_frame_end.as_mut().unwrap().cleanup_finished();

    if state.swapchain_needs_rebuild {
      let (new_swapchain, new_images) = match state.swapchain.recreate(SwapchainCreateInfo {
        image_extent: dimensions.into(),
        ..state.swapchain.create_info()
      }) {
        Ok(result) => result,
        Err(error) => panic!("{:?}", error),
      };

      state.swapchain = new_swapchain;
      state.swapchain_needs_rebuild = false;
    }

    let swapchain = &state.swapchain;

    let (image_index, suboptimal, acquire_future) = match acquire_next_image(swapchain.clone(), None) {
      Ok(result) => result,
      Err(AcquireError::OutOfDate) => {
        state.swapchain_needs_rebuild = true;
        return;
      }
      Err(error) => panic!("Failed to acquire next image: {:?}", error),
    };

    if suboptimal {
      state.swapchain_needs_rebuild = true;
    }
  }

  fn end_frame(&self) {
    // no-op
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
