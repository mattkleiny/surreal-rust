use std::sync::Mutex;

use winit::dpi::PhysicalSize;

use super::*;

/// Re-export the [`wgpu`] crate for consumers of Surreal.
pub mod wgpu {
  pub use wgpu::*;
}

type ShaderStorage = Storage<ShaderId, WgpuShader>;
type MaterialStorage = Storage<MaterialId, WgpuMaterial>;
type MeshStorage = Storage<MeshId, WgpuMesh>;
type LightStorage = Storage<LightId, WgpuLight>;

/// The [`GraphicsServerBackend`] for WGPU.
pub struct WgpuBackend {
  state: Mutex<WgpuState>,
  shader_storage: ShaderStorage,
  _material_storage: MaterialStorage,
  _mesh_storage: MeshStorage,
  _light_storage: LightStorage,
}

/// Top-level lockable state for the [`WgpuBackend`].
struct WgpuState {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  render_pipeline: wgpu::RenderPipeline,
}

/// Internal data for a shader in the [`WgpuBackend`].
struct WgpuShader {
  label: Option<String>,
  shader_module: wgpu::ShaderModule,
  bind_group_layout: wgpu::BindGroupLayout,
}

/// Internal data for a material in the [`WgpuBackend`].
struct WgpuMaterial {}

/// Internal data for a mesh in the [`WgpuBackend`].
struct WgpuMesh {}

/// Internal data for a light in the [`WgpuBackend`].
struct WgpuLight {}

impl WgpuBackend {
  /// Creates a new [`WgpuBackend`] for the given [`winit::window::Window`].
  pub async fn new(window: &winit::window::Window) -> surreal::Result<Self> {
    let size = window.inner_size();

    // The instance is a handle to our GPU
    // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(window) };

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      })
      .await
      .ok_or(surreal::anyhow!("Unable to select appropriate adapter"))?;

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

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_supported_formats(&adapter)[0],
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
    };

    surface.configure(&device, &config);

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("Test Shader"),
      source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/standard.wgsl").into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[],
      push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[],
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main",
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState::REPLACE),
          write_mask: wgpu::ColorWrites::ALL,
        })],
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        unclipped_depth: false,
        conservative: false,
      },
      depth_stencil: None,
      multisample: wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
      },
      multiview: None,
    });

    Ok(Self {
      state: Mutex::new(WgpuState {
        surface,
        device,
        queue,
        config,
        render_pipeline,
      }),
      shader_storage: ShaderStorage::default(),
      _material_storage: MaterialStorage::default(),
      _mesh_storage: MeshStorage::default(),
      _light_storage: LightStorage::default(),
    })
  }
}

impl GraphicsServerBackend for WgpuBackend {
  fn begin_frame(&self, color: Color) -> surreal::Result<()> {
    let state = self.state.lock().unwrap();

    let output_surface = state.surface.get_current_texture()?;
    let output_view = output_surface.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = state
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Surreal") });

    {
      let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Forward Opaque"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &output_view,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color {
              r: color.r as f64,
              g: color.g as f64,
              b: color.b as f64,
              a: color.a as f64,
            }),
            store: true,
          },
        })],
        depth_stencil_attachment: None,
      });

      render_pass.set_pipeline(&state.render_pipeline);
      render_pass.draw(0..3, 0..1);
    }

    state.queue.submit(Some(encoder.finish()));
    output_surface.present();

    Ok(())
  }

  fn end_frame(&self) -> surreal::Result<()> {
    Ok(())
  }

  fn resize_viewport(&self, new_size: PhysicalSize<u32>) -> surreal::Result<()> {
    if new_size.width > 0 && new_size.height > 0 {
      let mut state = self.state.lock().unwrap();

      state.config.width = new_size.width;
      state.config.height = new_size.height;

      state.surface.configure(&state.device, &state.config);
    }

    Ok(())
  }

  fn shader_create(&self, name: Option<&str>) -> surreal::Result<ShaderId> {
    let shader_id = self.shader_storage.create(|_| {
      let state = self.state.lock().unwrap();

      let shader_module = state.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: name,
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/standard.wgsl").into()),
      });

      let bind_group_layout = state
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: name, entries: &[] });

      WgpuShader {
        label: name.map(|it| it.to_string()),
        shader_module,
        bind_group_layout,
      }
    });

    Ok(shader_id)
  }

  fn shader_set_code(&self, shader_id: ShaderId, code: &str) -> surreal::Result<()> {
    self.shader_storage.write(shader_id, |shader| {
      let state = self.state.lock().unwrap();

      shader.shader_module = state.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: shader.label.as_deref(),
        source: wgpu::ShaderSource::Wgsl(code.into()),
      });

      shader.bind_group_layout = state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: shader.label.as_deref(),
        entries: &[],
      });
    });

    Ok(())
  }

  fn shader_delete(&self, shader_id: ShaderId) -> surreal::Result<()> {
    self.shader_storage.remove(shader_id);

    Ok(())
  }
}
