//! The OpenGL backend implementation for the graphics subsystem.

use std::{
  cell::RefCell,
  collections::HashMap,
  ffi::{c_void, CString},
  num::NonZeroU32,
};

use glutin::{
  config::ConfigTemplate,
  context::{ContextApi, ContextAttributesBuilder, Version},
  display::{Display, DisplayApiPreference},
  prelude::*,
  surface::{Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface},
};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use super::*;
use crate as surreal;
use crate::{diagnostics, maths::Rectangle, utilities::Size};

/// An OpenGL [`GraphicsBackend`] implementation.
pub struct OpenGLGraphicsBackend {
  state: RefCell<BackendState>,
}

/// Interior mutable state for the backend.
struct BackendState {
  context: glutin::context::PossiblyCurrentContext,
  surface: Surface<WindowSurface>,
  sampler_cache: HashMap<TextureSampler, u32>,
}

impl OpenGLGraphicsBackend {
  /// Creates a new OpenGL backend.
  ///
  /// # Safety
  /// This method interacts with the OpenGL platform directly via Glutin. It's possible for any
  /// number of things to go wrong when using the platform APIs.
  pub unsafe fn new(window: &winit::window::Window, vsync_enabled: bool, _samples: u8) -> crate::Result<Self> {
    let display = Display::new(
      window.raw_display_handle(),
      DisplayApiPreference::Wgl(Some(window.raw_window_handle())),
    )?;

    let config = display
      .find_configs(ConfigTemplate::default())?
      .next()
      .ok_or(crate::anyhow!("Unable to find appropriate config for display"))?;

    let context_attributes = ContextAttributesBuilder::new()
      .with_debug(true)
      .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
      .build(Some(window.raw_window_handle()));

    let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new()
      .with_srgb(None)
      .with_single_buffer(false)
      .build(
        window.raw_window_handle(),
        NonZeroU32::new(1920).unwrap(),
        NonZeroU32::new(1080).unwrap(),
      );

    let context = display.create_context(&config, &context_attributes)?;
    let surface = display.create_window_surface(&config, &surface_attributes)?;
    let context = context.make_current(&surface)?;

    surface.set_swap_interval(
      &context,
      if vsync_enabled {
        SwapInterval::Wait(NonZeroU32::new(1).unwrap())
      } else {
        SwapInterval::DontWait
      },
    )?;

    gl::load_with(|symbol| display.get_proc_address(&CString::new(symbol).unwrap()) as *const _);

    Ok(Self {
      state: RefCell::new(BackendState {
        context,
        surface,
        sampler_cache: HashMap::new(),
      }),
    })
  }
}

impl GraphicsBackend for OpenGLGraphicsBackend {
  #[diagnostics::profiling]
  fn begin_frame(&self) {
    // no-op
  }

  #[diagnostics::profiling]
  fn end_frame(&self) {
    let state = self.state.borrow();

    state.surface.swap_buffers(&state.context).unwrap();
  }

  fn clear_color_buffer(&self, color: Color) {
    unsafe {
      gl::ClearColor(color.r, color.g, color.b, color.a);
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
  }

  fn clear_depth_buffer(&self) {
    unsafe {
      gl::Clear(gl::DEPTH_BUFFER_BIT);
    }
  }

  fn viewport_size(&self) -> (usize, usize) {
    unsafe {
      let mut size = [0i32; 4];
      gl::GetIntegerv(gl::VIEWPORT, size.as_mut_ptr());

      (size[2] as usize, size[3] as usize)
    }
  }

  fn set_viewport_size(&self, size: winit::dpi::PhysicalSize<u32>) {
    if size.width > 0 && size.height > 0 {
      let state = self.state.borrow();

      state.surface.resize(
        &state.context,
        NonZeroU32::new(size.width).unwrap(),
        NonZeroU32::new(size.height).unwrap(),
      );

      unsafe {
        gl::Viewport(0, 0, size.width as i32, size.height as i32);
      }
    }
  }

  fn set_blend_state(&self, blend_state: BlendState) {
    fn convert_blend_factor(factor: BlendFactor) -> u32 {
      match factor {
        BlendFactor::One => gl::ONE,
        BlendFactor::SrcAlpha => gl::SRC_ALPHA,
        BlendFactor::SrcColor => gl::SRC_COLOR,
        BlendFactor::DstAlpha => gl::DST_ALPHA,
        BlendFactor::DstColor => gl::DST_COLOR,
        BlendFactor::OneMinusSrcAlpha => gl::ONE_MINUS_SRC_ALPHA,
        BlendFactor::OneMinusSrcColor => gl::ONE_MINUS_SRC_COLOR,
        BlendFactor::OneMinusDstAlpha => gl::ONE_MINUS_DST_ALPHA,
        BlendFactor::OneMinusDstColor => gl::ONE_MINUS_DST_COLOR,
      }
    }

    unsafe {
      match blend_state {
        BlendState::Disabled => gl::Disable(gl::BLEND),
        BlendState::Enabled { source, destination: dest } => {
          let source = convert_blend_factor(source);
          let dest = convert_blend_factor(dest);

          gl::Enable(gl::BLEND);
          gl::BlendFunc(source, dest);
        }
      }
    }
  }

  fn set_culling_mode(&self, culling_mode: CullingMode) {
    unsafe {
      match culling_mode {
        CullingMode::Disabled => gl::Disable(gl::CULL_FACE),
        CullingMode::Front => {
          gl::Enable(gl::CULL_FACE);
          gl::CullFace(gl::FRONT);
        }
        CullingMode::Back => {
          gl::Enable(gl::CULL_FACE);
          gl::CullFace(gl::BACK);
        }
        CullingMode::Both => {
          gl::Enable(gl::CULL_FACE);
          gl::CullFace(gl::FRONT_AND_BACK);
        }
      }
    }
  }

  fn set_scissor_mode(&self, scissor_mode: ScissorMode) {
    unsafe {
      match scissor_mode {
        ScissorMode::Disabled => {
          gl::Disable(gl::SCISSOR_TEST);
        }
        ScissorMode::Enabled {
          left,
          bottom: top,
          width,
          height,
        } => {
          gl::Enable(gl::SCISSOR_TEST);
          gl::Scissor(left, top, width, height);
        }
      }
    }
  }

  fn buffer_create(&self) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;
      gl::GenBuffers(1, &mut id);
      id
    }
  }

  fn buffer_read_data(&self, buffer: GraphicsHandle, offset: usize, length: usize, pointer: *mut u8) {
    unsafe {
      gl::GetNamedBufferSubData(buffer, offset as isize, length as isize, pointer as *mut c_void);
    }
  }

  fn buffer_write_data(&self, buffer: GraphicsHandle, usage: BufferUsage, kind: BufferKind, length: usize, pointer: *const u8) {
    unsafe {
      let kind = match kind {
        BufferKind::Element => gl::ARRAY_BUFFER,
        BufferKind::Index => gl::ELEMENT_ARRAY_BUFFER,
      };

      let usage = match usage {
        BufferUsage::Static => gl::STATIC_DRAW,
        BufferUsage::Dynamic => gl::DYNAMIC_DRAW,
      };

      gl::BindBuffer(kind, buffer);
      gl::BufferData(kind, length as isize, pointer as *const _, usage);
    }
  }

  fn buffer_delete(&self, buffer: GraphicsHandle) {
    unsafe {
      gl::DeleteBuffers(1, &buffer);
    }
  }

  fn texture_create(&self, sampler: &TextureSampler) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;

      gl::GenTextures(1, &mut id);
      gl::BindTexture(gl::TEXTURE_2D, id);

      self.texture_set_options(id, sampler);

      id
    }
  }

  fn texture_set_options(&self, texture: GraphicsHandle, sampler: &TextureSampler) {
    unsafe {
      let min_filter = match sampler.minify_filter {
        TextureFilter::Nearest => gl::NEAREST,
        TextureFilter::Linear => gl::LINEAR,
      };

      let mag_filter = match sampler.magnify_filter {
        TextureFilter::Nearest => gl::NEAREST,
        TextureFilter::Linear => gl::LINEAR,
      };

      let wrap_mode = match sampler.wrap_mode {
        TextureWrap::Clamp => gl::CLAMP_TO_EDGE,
        TextureWrap::Mirror => gl::MIRRORED_REPEAT,
      };

      gl::BindTexture(gl::TEXTURE_2D, texture);

      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_mode as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_mode as i32);
    }
  }

  fn texture_initialize(&self, texture: GraphicsHandle, width: u32, height: u32, format: TextureFormat) {
    unsafe {
      let (components, kind) = convert_texture_format(format);

      gl::BindTexture(gl::TEXTURE_2D, texture);
      gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        format as i32,
        width as i32,
        height as i32,
        0,
        components,
        kind,
        std::ptr::null(),
      );
    }
  }

  fn texture_read_data(&self, texture: GraphicsHandle, length: usize, pixel_format: TextureFormat, pixels: *mut u8, mip_level: usize) {
    unsafe {
      let (components, kind) = convert_texture_format(pixel_format);

      gl::BindTexture(gl::TEXTURE_2D, texture);
      gl::GetnTexImage(
        gl::TEXTURE_2D,
        mip_level as i32,
        components,
        kind,
        length as i32,
        pixels as *mut c_void,
      );
    }
  }

  fn texture_write_data(
    &self,
    texture: GraphicsHandle,
    width: u32,
    height: u32,
    pixels: *const u8,
    internal_format: TextureFormat,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) {
    unsafe {
      let internal_format = match internal_format {
        TextureFormat::R8 => gl::R8,
        TextureFormat::RG8 => gl::RG8,
        TextureFormat::RGB8 => gl::RGB8,
        TextureFormat::RGBA8 => gl::RGBA8,
        TextureFormat::R32 => gl::R32F,
        TextureFormat::RG32 => gl::RG32F,
        TextureFormat::RGB32 => gl::RGB32F,
        TextureFormat::RGBA32 => gl::RGBA32F,
        TextureFormat::A8 => gl::ALPHA,
        TextureFormat::A32 => gl::ALPHA,
      };

      let (components, kind) = convert_texture_format(pixel_format);

      gl::BindTexture(gl::TEXTURE_2D, texture);
      gl::TexImage2D(
        gl::TEXTURE_2D,
        mip_level as i32,
        internal_format as i32,
        width as i32,
        height as i32,
        0, // border
        components,
        kind,
        pixels as *const _,
      );
    }
  }

  fn texture_write_sub_data(
    &self,
    texture: GraphicsHandle,
    region: &Rectangle,
    pixels: *const u8,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) {
    unsafe {
      let (components, kind) = convert_texture_format(pixel_format);

      gl::BindTexture(gl::TEXTURE_2D, texture);
      gl::TexSubImage2D(
        gl::TEXTURE_2D,
        mip_level as i32,
        region.left() as i32,
        region.top() as i32,
        region.width() as i32,
        region.height() as i32,
        components,
        kind,
        pixels as *const _,
      );
    }
  }

  fn texture_delete(&self, texture: GraphicsHandle) {
    unsafe {
      gl::DeleteTextures(1, &texture);
    }
  }

  fn shader_create(&self) -> GraphicsHandle {
    unsafe { gl::CreateProgram() }
  }

  #[allow(clippy::uninit_vec)]
  fn shader_link(&self, shader: GraphicsHandle, shaders: &[ShaderKernel]) -> crate::Result<()> {
    unsafe {
      gl::UseProgram(shader);

      // compile the shader kernel code
      let mut shader_ids = Vec::with_capacity(shaders.len());

      for ShaderKernel { kind, code } in shaders {
        let shader_id = gl::CreateShader(match kind {
          ShaderKind::Vertex => gl::VERTEX_SHADER,
          ShaderKind::Fragment => gl::FRAGMENT_SHADER,
          ShaderKind::Compute => gl::COMPUTE_SHADER,
        });

        let code_length = code.len() as i32;
        let code: *const i8 = std::mem::transmute(code.as_bytes().as_ptr());

        gl::ShaderSource(shader_id, 1, &code, &code_length);
        gl::CompileShader(shader_id);

        let mut compile_status = 0;
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compile_status);

        if compile_status == 0 {
          let mut info_log_length = 0;
          gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_log_length);

          let mut info_log = Vec::with_capacity(info_log_length as usize);
          info_log.set_len(info_log_length as usize);

          gl::GetShaderInfoLog(shader_id, info_log_length, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut _);

          return Err(crate::anyhow!(String::from_utf8(info_log).unwrap()));
        }

        gl::AttachShader(shader, shader_id);
        shader_ids.push(shader_id);
      }

      // link the kernels in the main program
      let mut link_status = 0;

      gl::LinkProgram(shader);
      gl::GetProgramiv(shader, gl::LINK_STATUS, &mut link_status);

      if link_status == 0 {
        let mut info_log_length = 0;
        gl::GetProgramiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_length);

        let mut info_log = Vec::with_capacity(info_log_length as usize);
        info_log.set_len(info_log_length as usize);

        gl::GetProgramInfoLog(shader, info_log_length, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut _);

        return Err(crate::anyhow!(String::from_utf8(info_log).unwrap()));
      }

      // delete the kernels now that we've linked
      for shader_id in shader_ids {
        gl::DeleteShader(shader_id);
      }
    }

    Ok(())
  }

  fn shader_uniform_location(&self, shader: GraphicsHandle, name: &str) -> Option<usize> {
    unsafe {
      let name = CString::new(name).unwrap();
      let location = gl::GetUniformLocation(shader, name.as_ptr());

      match location {
        -1 => None,
        location => Some(location as usize),
      }
    }
  }

  fn shader_set_uniform(&self, shader: GraphicsHandle, location: usize, value: &ShaderUniform) {
    unsafe {
      match value {
        ShaderUniform::Bool(value) => {
          gl::ProgramUniform1i(shader, location as i32, *value as i32);
        }
        ShaderUniform::I32(value) => {
          gl::ProgramUniform1i(shader, location as i32, *value);
        }
        ShaderUniform::U32(value) => {
          gl::ProgramUniform1i(shader, location as i32, *value as i32);
        }
        ShaderUniform::F32(value) => {
          gl::ProgramUniform1f(shader, location as i32, *value);
        }
        ShaderUniform::Vec2(value) => {
          gl::ProgramUniform2f(shader, location as i32, value.x, value.y);
        }
        ShaderUniform::Vec3(value) => {
          gl::ProgramUniform3f(shader, location as i32, value.x, value.y, value.z);
        }
        ShaderUniform::Vec4(value) => {
          gl::ProgramUniform4f(shader, location as i32, value.x, value.y, value.z, value.w);
        }
        ShaderUniform::Mat2(value) => {
          gl::ProgramUniformMatrix2fv(shader, location as i32, 1, gl::FALSE, &value.to_cols_array()[0]);
        }
        ShaderUniform::Mat3(value) => {
          gl::ProgramUniformMatrix3fv(shader, location as i32, 1, gl::FALSE, &value.to_cols_array()[0]);
        }
        ShaderUniform::Mat4(value) => {
          gl::ProgramUniformMatrix4fv(shader, location as i32, 1, gl::FALSE, &value.to_cols_array()[0]);
        }
        ShaderUniform::Color(color) => {
          gl::ProgramUniform4f(shader, location as i32, color.r, color.g, color.b, color.a);
        }
        ShaderUniform::Color32(color) => {
          gl::ProgramUniform4ui(
            shader,
            location as i32,
            color.r as u32,
            color.g as u32,
            color.b as u32,
            color.a as u32,
          );
        }
        ShaderUniform::Texture(texture, slot, sampler) => {
          gl::ActiveTexture(gl::TEXTURE0 + *slot as u32);
          gl::BindTexture(gl::TEXTURE_2D, texture.handle());
          gl::ProgramUniform1i(shader, location as i32, *slot as i32);

          if let Some(sampler) = sampler {
            // build and cache sampler settings based on hash of options
            let mut internal_state = self.state.borrow_mut();
            let sampler_cache = &mut internal_state.sampler_cache;

            let sampler_id = sampler_cache.entry(*sampler).or_insert_with(|| {
              let mut sampler_id = 0;

              gl::CreateSamplers(1, &mut sampler_id);

              let min_filter = match sampler.minify_filter {
                TextureFilter::Nearest => gl::NEAREST,
                TextureFilter::Linear => gl::LINEAR,
              };

              let mag_filter = match sampler.magnify_filter {
                TextureFilter::Nearest => gl::NEAREST,
                TextureFilter::Linear => gl::LINEAR,
              };

              let wrap_mode = match sampler.wrap_mode {
                TextureWrap::Clamp => gl::CLAMP_TO_EDGE,
                TextureWrap::Mirror => gl::MIRRORED_REPEAT,
              };

              gl::SamplerParameteri(sampler_id, gl::TEXTURE_WRAP_S, wrap_mode as i32);
              gl::SamplerParameteri(sampler_id, gl::TEXTURE_WRAP_T, wrap_mode as i32);
              gl::SamplerParameteri(sampler_id, gl::TEXTURE_MIN_FILTER, min_filter as i32);
              gl::SamplerParameteri(sampler_id, gl::TEXTURE_MAG_FILTER, mag_filter as i32);

              sampler_id
            });

            gl::BindSampler(*slot as u32, *sampler_id);
          } else {
            gl::BindSampler(*slot as u32, 0);
          }
        }
      }
    }
  }

  fn shader_activate(&self, shader: GraphicsHandle) {
    unsafe {
      gl::UseProgram(shader);
    }
  }

  fn shader_delete(&self, shader: GraphicsHandle) {
    unsafe {
      gl::DeleteProgram(shader);
    }
  }

  fn mesh_create(&self, vertex_buffer: GraphicsHandle, index_buffer: GraphicsHandle, descriptors: &[VertexDescriptor]) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;

      gl::GenVertexArrays(1, &mut id);
      gl::BindVertexArray(id);

      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);

      let stride: Size = descriptors.iter().map(|desc| desc.size()).sum();
      let mut offset = 0;

      for (index, descriptor) in descriptors.iter().enumerate() {
        let (kind, is_integral) = match descriptor.kind {
          VertexKind::U8 => (gl::UNSIGNED_BYTE, true),
          VertexKind::U16 => (gl::UNSIGNED_SHORT, true),
          VertexKind::U32 => (gl::UNSIGNED_INT, true),
          VertexKind::I16 => (gl::SHORT, true),
          VertexKind::I32 => (gl::INT, true),
          VertexKind::F32 => (gl::FLOAT, false),
          VertexKind::F64 => (gl::DOUBLE, false),
        };

        if !is_integral || descriptor.should_normalize {
          gl::VertexAttribPointer(
            index as u32,
            descriptor.count as i32,
            kind,
            match descriptor.should_normalize {
              true => gl::TRUE,
              false => gl::FALSE,
            },
            stride.as_bytes() as i32,
            offset as *const _,
          );
        } else {
          gl::VertexAttribIPointer(
            index as u32,
            descriptor.count as i32,
            kind,
            stride.as_bytes() as i32,
            offset as *const _,
          );
        }

        gl::EnableVertexAttribArray(index as u32);

        offset += descriptor.size().as_bytes();
      }

      gl::BindVertexArray(0);

      id
    }
  }

  fn mesh_draw(&self, mesh: GraphicsHandle, topology: PrimitiveTopology, vertex_count: usize, index_count: usize) {
    unsafe {
      gl::BindVertexArray(mesh);

      let topology = match topology {
        PrimitiveTopology::Points => gl::POINTS,
        PrimitiveTopology::Lines => gl::LINES,
        PrimitiveTopology::Triangles => gl::TRIANGLES,
      };

      if index_count > 0 {
        gl::DrawElements(topology, index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
      } else {
        gl::DrawArrays(topology, 0, vertex_count as i32);
      }

      gl::BindVertexArray(0);
    }
  }

  fn mesh_delete(&self, mesh: GraphicsHandle) {
    unsafe {
      gl::DeleteVertexArrays(1, &mesh);
    }
  }

  fn target_create(
    &self,
    color_attachment: GraphicsHandle,
    depth_attachment: Option<GraphicsHandle>,
    stencil_attachment: Option<GraphicsHandle>,
  ) -> GraphicsHandle {
    unsafe {
      let mut framebuffer = 0;
      gl::CreateFramebuffers(1, &mut framebuffer);

      gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
      gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, color_attachment, 0);

      if let Some(depth_attachment) = depth_attachment {
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, depth_attachment, 0);
      }

      if let Some(stencil_attachment) = stencil_attachment {
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::STENCIL_ATTACHMENT, gl::TEXTURE_2D, stencil_attachment, 0);
      }

      if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
        panic!("Failed to create render target");
      }

      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

      framebuffer
    }
  }

  fn target_activate(&self, render_target: GraphicsHandle) {
    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, render_target);
    }
  }

  fn target_set_default(&self) {
    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
  }

  fn target_blit(&self, from: GraphicsHandle, to: GraphicsHandle, source_rect: &Rectangle, dest_rect: &Rectangle, filter: TextureFilter) {
    unsafe {
      gl::BindFramebuffer(gl::READ_FRAMEBUFFER, from);
      gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, to);

      gl::BlitFramebuffer(
        source_rect.left() as i32,
        source_rect.top() as i32,
        source_rect.width() as i32,
        source_rect.height() as i32,
        dest_rect.left() as i32,
        dest_rect.top() as i32,
        dest_rect.width() as i32,
        dest_rect.height() as i32,
        gl::COLOR_BUFFER_BIT,
        match filter {
          TextureFilter::Nearest => gl::NEAREST,
          TextureFilter::Linear => gl::LINEAR,
        },
      );

      gl::BindFramebuffer(gl::READ_FRAMEBUFFER, 0);
      gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
    }
  }

  fn target_blit_to_display(&self, handle: GraphicsHandle, source_rect: &Rectangle, dest_rect: &Rectangle, filter: TextureFilter) {
    self.target_blit(handle, 0, source_rect, dest_rect, filter);
  }

  fn target_delete(&self, render_target: GraphicsHandle) {
    unsafe {
      gl::DeleteFramebuffers(1, &render_target);
    }
  }
}

fn convert_texture_format(texture_format: TextureFormat) -> (u32, u32) {
  match texture_format {
    TextureFormat::R8 => (gl::RED, gl::UNSIGNED_BYTE),
    TextureFormat::RG8 => (gl::RG, gl::UNSIGNED_BYTE),
    TextureFormat::RGB8 => (gl::RGB, gl::UNSIGNED_BYTE),
    TextureFormat::RGBA8 => (gl::RGBA, gl::UNSIGNED_BYTE),
    TextureFormat::R32 => (gl::RED, gl::FLOAT),
    TextureFormat::RG32 => (gl::RG, gl::FLOAT),
    TextureFormat::RGB32 => (gl::RGB, gl::FLOAT),
    TextureFormat::RGBA32 => (gl::RGBA, gl::FLOAT),
    TextureFormat::A8 => (gl::ALPHA, gl::UNSIGNED_BYTE),
    TextureFormat::A32 => (gl::ALPHA, gl::FLOAT),
  }
}
