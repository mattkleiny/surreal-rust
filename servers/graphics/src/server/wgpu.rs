use crate::server::utilities::Storage;

use super::*;

mod wgpu {
  pub use wgpu::*;
}

/// The [`GraphicsServerBackend`] for WGPU.
pub struct WgpuBackend {
  state: std::sync::Mutex<WgpuState>,
  _shader_storage: Storage<ShaderId, WgpuShader>,
  material_storage: Storage<MaterialId, WgpuMaterial>,
  _mesh_storage: Storage<MeshId, WgpuMesh>,
  _light_storage: Storage<LightId, WgpuLight>,
}

/// Top-level lockable state for the [`WgpuBackend`].
struct WgpuState {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  surface_config: wgpu::SurfaceConfiguration,
}

/// Internal data for a shader in the [`WgpuBackend`].
struct WgpuShader {}

/// Internal data for a material in the [`WgpuBackend`].
struct WgpuMaterial {
  render_pipeline: wgpu::RenderPipeline,
}

/// Internal data for a mesh in the [`WgpuBackend`].
struct WgpuMesh {}

/// Internal data for a light in the [`WgpuBackend`].
struct WgpuLight {}

impl WgpuBackend {
  /// Creates a new [`WgpuBackend`] for the given [`winit::window::Window`].
  pub async fn new(window: &winit::window::Window) -> surreal::Result<Self> {
    // initialize the wgpu backend
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(window) };

    // determine physical adapter
    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .ok_or(surreal::anyhow!("Unable to select appropriate adapter"))?;

    // create main device and queue
    let (device, queue) = adapter
      .request_device(
        &wgpu::DeviceDescriptor {
          features: wgpu::Features::empty(),
          limits: wgpu::Limits::default(),
          label: None,
        },
        None, // Trace path
      )
      .await?;

    // build the main render surface
    let size = window.inner_size();

    let surface_config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_supported_formats(&adapter)[0],
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
    };

    surface.configure(&device, &surface_config);

    Ok(Self {
      state: std::sync::Mutex::new(WgpuState {
        surface,
        device,
        queue,
        surface_config,
      }),
      _shader_storage: Storage::default(),
      material_storage: Storage::default(),
      _mesh_storage: Storage::default(),
      _light_storage: Storage::default(),
    })
  }
}

impl GraphicsServerBackend for WgpuBackend {
  fn execute_commands(&self, commands: &mut CommandBuffer) -> surreal::Result<()> {
    while let Some(command) = commands.dequeue() {
      match command {
        Command::DrawIndirect {
          material_id,
          vertices,
          instances,
        } => {
          let state = self.state.lock().unwrap();

          let surface = state.surface.get_current_texture()?;
          let view = surface.texture.create_view(&wgpu::TextureViewDescriptor::default());

          let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

          self.material_storage.read(material_id, |material| {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
              label: Some("Draw Indirect"),
              color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                  load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                  store: true,
                },
              })],
              depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&material.render_pipeline);
            render_pass.draw(vertices.clone(), instances.clone());
          });

          state.queue.submit(Some(encoder.finish()));
        }
      }
    }

    Ok(())
  }

  fn resize_viewport(&self, new_size: winit::dpi::PhysicalSize<u32>) -> surreal::Result<()> {
    if new_size.width > 0 && new_size.height > 0 {
      let mut state = self.state.lock().unwrap();

      state.surface_config.width = new_size.width;
      state.surface_config.height = new_size.height;

      state.surface.configure(&state.device, &state.surface_config);
    }

    Ok(())
  }
}
