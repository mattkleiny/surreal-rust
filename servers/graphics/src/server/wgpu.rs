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
  texture_storage: Storage<TextureId, WgpuTexture>,
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

/// Internal data for a texture in the [`WgpuBackend`].
struct WgpuTexture {
  texture: wgpu::Texture,
}

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
      texture_storage: Storage::default(),
    })
  }
}

impl GraphicsServerBackend for WgpuBackend {
  fn execute_commands(&self, commands: &mut CommandBuffer) -> surreal::Result<()> {
    let state = self.state.lock().unwrap();

    let surface = state.surface.get_current_texture()?;
    let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    while let Some(command) = commands.dequeue() {
      match command {
        Command::WriteTexture { texture_id, pixels } => {
          self.texture_storage.read(texture_id, |texture| {
            state.queue.write_texture(
              wgpu::ImageCopyTexture {
                texture: &texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
              },
              pixels,
              wgpu::ImageDataLayout::default(),
              wgpu::Extent3d {
                width: 1024,
                height: 1024,
                depth_or_array_layers: 0,
              },
            );
          });
        }
        Command::DrawIndirect {
          material_id,
          vertices,
          instances,
        } => {
          let view = surface.texture.create_view(&wgpu::TextureViewDescriptor::default());

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
        }
      }
    }

    state.queue.submit(Some(encoder.finish()));
    surface.present();

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

  fn texture_create(&self) -> surreal::Result<TextureId> {
    let texture_id = self.texture_storage.create(|| {
      let state = self.state.lock().unwrap();

      let texture = state.device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d::default(),
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING,
      });

      WgpuTexture { texture }
    });

    Ok(texture_id)
  }

  fn texture_write(&self, texture_id: TextureId, pixels: &[u8]) -> surreal::Result<()> {
    self.texture_storage.read(texture_id, |texture| {
      let state = self.state.lock().unwrap();

      state.queue.write_texture(
        wgpu::ImageCopyTexture {
          texture: &texture.texture,
          mip_level: 0,
          origin: wgpu::Origin3d::ZERO,
          aspect: wgpu::TextureAspect::All,
        },
        pixels,
        wgpu::ImageDataLayout::default(),
        wgpu::Extent3d {
          width: 1024,
          height: 1024,
          depth_or_array_layers: 0,
        },
      );
    });

    Ok(())
  }

  fn texture_delete(&self, texture_id: TextureId) -> surreal::Result<()> {
    self.texture_storage.remove(texture_id);

    Ok(())
  }
}
