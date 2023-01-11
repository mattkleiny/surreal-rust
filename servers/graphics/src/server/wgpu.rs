use winit::dpi::PhysicalSize;

use super::*;

/// Re-export of [`wgpu`] crate.
pub mod wgpu {
  pub use wgpu::*;
}

pub struct WgpuBackend {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  config: wgpu::SurfaceConfiguration,
  render_pipeline: wgpu::RenderPipeline,
}

impl WgpuBackend {
  pub async fn new(window: Arc<winit::window::Window>) -> surreal::Result<Self> {
    let size = window.inner_size();

    // The instance is a handle to our GPU
    // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(window.as_ref()) };

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
      surface,
      device,
      queue,
      config,
      render_pipeline,
    })
  }
}

#[allow(unused_variables)]
impl GraphicsServerBackend for WgpuBackend {
  fn begin_frame(&self, color: Color) -> surreal::Result<()> {
    let output_surface = self.surface.get_current_texture()?;
    let output_view = output_surface.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = self
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

      render_pass.set_pipeline(&self.render_pipeline);
      render_pass.draw(0..3, 0..1);
    }

    self.queue.submit(Some(encoder.finish()));
    output_surface.present();

    Ok(())
  }

  fn end_frame(&self) -> surreal::Result<()> {
    Ok(())
  }

  fn resize_viewport(&mut self, new_size: PhysicalSize<u32>) -> surreal::Result<()> {
    self.config.width = new_size.width;
    self.config.height = new_size.height;

    self.surface.configure(&self.device, &self.config);

    Ok(())
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
