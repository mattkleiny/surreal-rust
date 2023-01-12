use surreal::utilities::ResourceStorage;

use super::*;

mod wgpu {
  pub use wgpu::*;
}

/// The [`GraphicsBackend`] for WGPU.
pub struct WgpuBackend {
  state: std::sync::Mutex<WgpuState>,
  _shader_storage: ResourceStorage<ShaderId, WgpuShader>,
  _material_storage: ResourceStorage<MaterialId, WgpuMaterial>,
  _mesh_storage: ResourceStorage<MeshId, WgpuMesh>,
  _light_storage: ResourceStorage<LightId, WgpuLight>,
  texture_storage: ResourceStorage<TextureId, WgpuTexture>,
  target_storage: ResourceStorage<RenderTargetId, WgpuRenderTarget>,
}

/// Top-level lockable state for the [`WgpuBackend`].
struct WgpuState {
  surface: wgpu::Surface,
  device: wgpu::Device,
  _queue: wgpu::Queue,
  surface_config: wgpu::SurfaceConfiguration,
}

/// Internal data for a shader in the [`WgpuBackend`].
struct WgpuShader {}

/// Internal data for a material in the [`WgpuBackend`].
struct WgpuMaterial {}

/// Internal data for a mesh in the [`WgpuBackend`].
struct WgpuMesh {}

/// Internal data for a light in the [`WgpuBackend`].
struct WgpuLight {}

/// Internal data for a texture in the [`WgpuBackend`].
struct WgpuTexture {
  _texture: wgpu::Texture,
}

/// Internal data for a render target in the [`WgpuBackend`].
struct WgpuRenderTarget {}

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
        _queue: queue,
        surface_config,
      }),
      _shader_storage: ResourceStorage::default(),
      _material_storage: ResourceStorage::default(),
      _mesh_storage: ResourceStorage::default(),
      _light_storage: ResourceStorage::default(),
      texture_storage: ResourceStorage::default(),
      target_storage: ResourceStorage::default(),
    })
  }
}

impl GraphicsBackend for WgpuBackend {
  fn execute_commands(&self, commands: &mut CommandBuffer) -> surreal::Result<()> {
    // let state = self.state.lock().unwrap();
    //
    // let surface = state.surface.get_current_texture()?;
    // let descriptor = wgpu::CommandEncoderDescriptor { label: commands.label };
    //
    // let mut encoder = state.device.create_command_encoder(&descriptor);
    //
    // while let Some(command) = commands.dequeue() {
    //   match command {
    //     Command::WriteTexturePixels { .. } => {}
    //     Command::ReadTexturePixels { .. } => {}
    //     Command::SetGlobalUniform { .. } => {}
    //     Command::SetViewMatrix { .. } => {}
    //     Command::SetProjectionMatrix { .. } => {}
    //     Command::SetViewport { .. } => {}
    //     Command::SetRenderTarget { .. } => {}
    //     Command::BeginSample { .. } => {}
    //     Command::EndSample { .. } => {}
    //     Command::DrawMesh { .. } => {}
    //     Command::DrawIndirect {
    //       material_id,
    //       vertices,
    //       instances,
    //     } => {
    //       let view = surface.texture.create_view(&wgpu::TextureViewDescriptor::default());
    //
    //       let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: Some("Draw Indirect"),
    //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //           view: &view,
    //           resolve_target: None,
    //           ops: wgpu::Operations {
    //             load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
    //             store: true,
    //           },
    //         })],
    //         depth_stencil_attachment: None,
    //       });
    //
    //       render_pass.draw(vertices.clone(), instances.clone());
    //     }
    //   }
    // }
    //
    // state.queue.submit(Some(encoder.finish()));
    // surface.present();

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

  fn texture_create_1d(&self, label: Option<&str>, size: u32, format: TextureFormat) -> surreal::Result<TextureId> {
    let state = self.state.lock().unwrap();

    let texture = state.device.create_texture(&wgpu::TextureDescriptor {
      label,
      size: wgpu::Extent3d {
        width: size,
        height: 1,
        depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D1,
      format: match format {
        TextureFormat::RGBA8 => wgpu::TextureFormat::Rgba8Unorm,
        _ => todo!("Not yet implemented"),
      },
      usage: wgpu::TextureUsages::TEXTURE_BINDING,
    });

    Ok(self.texture_storage.insert(WgpuTexture { _texture: texture }))
  }

  fn texture_create_2d(&self, label: Option<&str>, size: UVec2, format: TextureFormat) -> surreal::Result<TextureId> {
    let state = self.state.lock().unwrap();

    let texture = state.device.create_texture(&wgpu::TextureDescriptor {
      label,
      size: wgpu::Extent3d {
        width: size.x,
        height: size.y,
        depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: match format {
        TextureFormat::RGBA8 => wgpu::TextureFormat::Rgba8Unorm,
        _ => todo!("Not yet implemented"),
      },
      usage: wgpu::TextureUsages::TEXTURE_BINDING,
    });

    Ok(self.texture_storage.insert(WgpuTexture { _texture: texture }))
  }

  fn texture_create_3d(&self, label: Option<&str>, size: UVec3, format: TextureFormat) -> surreal::Result<TextureId> {
    let state = self.state.lock().unwrap();

    let texture = state.device.create_texture(&wgpu::TextureDescriptor {
      label,
      size: wgpu::Extent3d {
        width: size.x,
        height: size.y,
        depth_or_array_layers: size.z,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D3,
      format: match format {
        TextureFormat::RGBA8 => wgpu::TextureFormat::Rgba8Unorm,
        _ => todo!("Not yet implemented"),
      },
      usage: wgpu::TextureUsages::TEXTURE_BINDING,
    });

    Ok(self.texture_storage.insert(WgpuTexture { _texture: texture }))
  }

  fn texture_read(&self, texture_id: TextureId) -> surreal::Result<Vec<u8>> {
    todo!()
  }

  fn texture_write(&self, texture_id: TextureId, pixels: &[u8]) -> surreal::Result<()> {
    todo!()
  }

  fn texture_delete(&self, texture_id: TextureId) -> surreal::Result<()> {
    self.texture_storage.remove(texture_id);

    Ok(())
  }

  fn target_create(&self, label: Option<&str>, size: UVec2, format: TextureFormat) -> surreal::Result<RenderTargetId> {
    Ok(self.target_storage.insert(WgpuRenderTarget {}))
  }

  fn target_delete(&self, render_target_id: RenderTargetId) -> surreal::Result<()> {
    self.target_storage.remove(render_target_id);

    Ok(())
  }
}
