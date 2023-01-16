use std::borrow::Cow;

use surreal::{
  collections::{FastHashMap, ResourceStorage},
  graphics::TextureFormat,
};

use super::*;

mod wgpu {
  pub use wgpu::*;
}

/// The [`GraphicsBackend`] for WGPU.
pub struct WgpuBackend {
  state: std::sync::Mutex<WgpuState>,
  shader_storage: ResourceStorage<ShaderId, WgpuShader>,
  material_storage: ResourceStorage<MaterialId, WgpuMaterial>,
  texture_storage: ResourceStorage<TextureId, WgpuTexture>,
  render_target_storage: ResourceStorage<RenderTargetId, WgpuRenderTarget>,
}

/// Top-level lockable state for the [`WgpuBackend`].
struct WgpuState {
  device: wgpu::Device,
  queue: wgpu::Queue,
  surface: wgpu::Surface,
  surface_config: wgpu::SurfaceConfiguration,
}

/// Internal data for a shader in the [`WgpuBackend`].
struct WgpuShader {
  _shader_module: wgpu::ShaderModule,
}

/// Internal data for a material in the [`WgpuBackend`].
struct WgpuMaterial {
  _uniforms: FastHashMap<String, UniformValue>,
  _uniform_buffer: wgpu::Buffer,
  _bind_group: wgpu::BindGroup,
}

/// Internal data for a texture in the [`WgpuBackend`].
struct WgpuTexture {
  _texture: wgpu::Texture,
  _texture_view: wgpu::TextureView,
  _sampler: wgpu::Sampler,
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
        device,
        queue,
        surface,
        surface_config,
      }),
      shader_storage: ResourceStorage::default(),
      material_storage: ResourceStorage::default(),
      texture_storage: ResourceStorage::default(),
      render_target_storage: ResourceStorage::default(),
    })
  }
}

impl GraphicsBackend for WgpuBackend {
  fn execute_commands(&self, commands: &mut CommandBuffer) -> surreal::Result<()> {
    let state = self.state.lock().unwrap();

    let surface = state.surface.get_current_texture()?;
    let descriptor = wgpu::CommandEncoderDescriptor { label: commands.label };

    let mut encoder = state.device.create_command_encoder(&descriptor);

    let view = surface.texture.create_view(&wgpu::TextureViewDescriptor::default());

    {
      let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Draw Indirect"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::RED),
            store: true,
          },
        })],
        depth_stencil_attachment: None,
      });
    }

    while let Some(command) = commands.dequeue() {
      match command {
        Command::WriteTexturePixels { .. } => {}
        Command::ReadTexturePixels { .. } => {}
        Command::SetGlobalUniform { .. } => {}
        Command::SetViewMatrix { .. } => {}
        Command::SetProjectionMatrix { .. } => {}
        Command::SetViewport { .. } => {}
        Command::SetRenderTarget { .. } => {}
        Command::BeginSample { .. } => {}
        Command::EndSample { .. } => {}
        Command::DrawMesh { .. } => {}
        Command::DrawIndirect { .. } => {}
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

  fn shader_create(&self, descriptor: &ShaderDescriptor) -> surreal::Result<ShaderId> {
    let state = self.state.lock().unwrap();

    // build shader module
    let shader_module = state.device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: descriptor.label,
      source: wgpu::ShaderSource::Wgsl(Cow::from(descriptor.shader_code)),
    });

    let shader_id = self.shader_storage.insert(WgpuShader {
      _shader_module: shader_module,
    });

    Ok(shader_id)
  }

  fn shader_delete(&self, shader_id: ShaderId) -> surreal::Result<()> {
    self.shader_storage.remove(shader_id);
    Ok(())
  }

  fn material_create(&self, descriptor: &MaterialDescriptor) -> surreal::Result<MaterialId> {
    use ::wgpu::util::DeviceExt;

    let state = self.state.lock().unwrap();

    // build uniform buffer
    let uniform_buffer = state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: descriptor.label,
      contents: &[0u8; 0],
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // build bind group
    let bind_group_layout = state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: descriptor.label,
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::all(),
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
    });

    let bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: descriptor.label,
      layout: &bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: uniform_buffer.as_entire_binding(),
      }],
    });

    Ok(self.material_storage.insert(WgpuMaterial {
      _uniforms: FastHashMap::default(),
      _uniform_buffer: uniform_buffer,
      _bind_group: bind_group,
    }))
  }

  fn material_delete(&self, material_id: MaterialId) -> surreal::Result<()> {
    self.material_storage.remove(material_id);
    Ok(())
  }

  fn texture_create(&self, descriptor: &TextureDescriptor) -> surreal::Result<TextureId> {
    let state = self.state.lock().unwrap();

    let (width, height, depth) = descriptor.size;
    let format = descriptor.format;

    let texture = state.device.create_texture(&wgpu::TextureDescriptor {
      label: descriptor.label,
      size: wgpu::Extent3d {
        width,
        height: if height == 0 { 1 } else { height },
        depth_or_array_layers: if depth == 0 { 1 } else { depth },
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: match () {
        _ if height > 0 && depth > 0 => wgpu::TextureDimension::D3,
        _ if height > 0 => wgpu::TextureDimension::D2,
        _ => wgpu::TextureDimension::D1,
      },
      format: match format {
        TextureFormat::RGBA8 => wgpu::TextureFormat::Rgba8Unorm,
        _ => todo!("Not yet implemented"),
      },
      usage: wgpu::TextureUsages::TEXTURE_BINDING,
    });

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let sampler = state.device.create_sampler(&wgpu::SamplerDescriptor {
      label: descriptor.label,
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Linear,
      min_filter: wgpu::FilterMode::Linear,
      mipmap_filter: wgpu::FilterMode::Nearest,
      lod_min_clamp: -100.0,
      lod_max_clamp: 100.0,
      compare: Some(wgpu::CompareFunction::LessEqual),
      anisotropy_clamp: None,
      border_color: None,
    });

    Ok(self.texture_storage.insert(WgpuTexture {
      _texture: texture,
      _texture_view: texture_view,
      _sampler: sampler,
    }))
  }

  fn texture_delete(&self, texture_id: TextureId) -> surreal::Result<()> {
    self.texture_storage.remove(texture_id);
    Ok(())
  }

  fn render_target_create(&self, _label: Option<&str>, _size: (u32, u32), _format: TextureFormat) -> surreal::Result<RenderTargetId> {
    Ok(self.render_target_storage.insert(WgpuRenderTarget {}))
  }

  fn render_target_delete(&self, render_target_id: RenderTargetId) -> surreal::Result<()> {
    self.render_target_storage.remove(render_target_id);
    Ok(())
  }
}
