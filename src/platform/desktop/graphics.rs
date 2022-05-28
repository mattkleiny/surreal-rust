use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;

use anyhow::anyhow;
use raw_gl_context::{GlConfig, GlContext};
use winit::window::Window;

use crate::graphics::*;
use crate::maths::Rectangle;

/// The graphics server for the desktop platform.
pub struct DesktopGraphicsBackend {
  context: GlContext,
  internal_state: RefCell<InternalState>,
}

/// Internally managed state for the backend.
struct InternalState {
  sampler_cache: HashMap<TextureSampler, u32>,
}

impl DesktopGraphicsBackend {
  pub fn new(window: &Window, vsync_enabled: bool) -> GraphicsServer {
    // prepare and load opengl bindings
    let config = GlConfig {
      vsync: vsync_enabled,
      ..Default::default()
    };

    let context = GlContext::create(window, config).unwrap();
    context.make_current();
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    GraphicsServer::new(Box::new(Self {
      context,
      internal_state: RefCell::new(InternalState {
        sampler_cache: HashMap::new(),
      }),
    }))
  }
}

impl GraphicsBackend for DesktopGraphicsBackend {
  #[macros::profile_function]
  fn begin_frame(&self) {
    self.context.make_current();
  }

  #[macros::profile_function]
  fn end_frame(&self) {
    unsafe {
      gl::Flush();
    }

    self.context.swap_buffers();
    self.context.make_not_current();
  }

  fn get_viewport_size(&self) -> (usize, usize) {
    unsafe {
      let mut size = [0i32; 4];
      gl::GetIntegerv(gl::VIEWPORT, size.as_mut_ptr());

      (size[2] as usize, size[3] as usize)
    }
  }

  fn set_viewport_size(&self, (width, height): (usize, usize)) {
    unsafe {
      self.context.make_current();
      gl::Viewport(0, 0, width as i32, height as i32);
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
        CullingMode::Disabled => {
          gl::Disable(gl::CULL_FACE)
        }
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
        ScissorMode::Enabled { left, bottom: top, width, height } => {
          gl::Enable(gl::SCISSOR_TEST);
          gl::Scissor(left, top, width, height);
        }
      }
    }
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

  fn create_buffer(&self) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;
      gl::GenBuffers(1, &mut id);
      id
    }
  }

  fn read_buffer_data(&self, buffer: GraphicsHandle, offset: usize, length: usize, pointer: *mut u8) {
    unsafe {
      gl::GetNamedBufferSubData(buffer, offset as isize, length as isize, pointer as *mut c_void);
    }
  }

  fn write_buffer_data(&self, buffer: GraphicsHandle, usage: BufferUsage, kind: BufferKind, length: usize, pointer: *const u8) {
    unsafe {
      let kind = match kind {
        BufferKind::Element => gl::ARRAY_BUFFER,
        BufferKind::Index => gl::ELEMENT_ARRAY_BUFFER,
        BufferKind::Uniform => gl::UNIFORM_BUFFER,
      };

      let usage = match usage {
        BufferUsage::Static => gl::STATIC_DRAW,
        BufferUsage::Dynamic => gl::DYNAMIC_DRAW,
      };

      gl::BindBuffer(kind, buffer);
      gl::BufferData(kind, length as isize, pointer as *const _, usage);
    }
  }

  fn delete_buffer(&self, buffer: GraphicsHandle) {
    unsafe {
      gl::DeleteBuffers(1, &buffer);
    }
  }

  fn create_texture(&self, sampler: &TextureSampler) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;

      gl::GenTextures(1, &mut id);
      gl::BindTexture(gl::TEXTURE_2D, id);

      self.set_texture_options(id, sampler);

      id
    }
  }

  fn set_texture_options(&self, texture: GraphicsHandle, sampler: &TextureSampler) {
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

  fn read_texture_data(&self, texture: GraphicsHandle, length: usize, pixel_format: TextureFormat, pixels: *mut u8, mip_level: usize) {
    unsafe {
      let (components, kind) = match pixel_format {
        TextureFormat::R8 => (gl::RED, gl::UNSIGNED_BYTE),
        TextureFormat::RG8 => (gl::RG, gl::UNSIGNED_BYTE),
        TextureFormat::RGB8 => (gl::RGB, gl::UNSIGNED_BYTE),
        TextureFormat::RGBA8 => (gl::RGBA, gl::UNSIGNED_BYTE),
        TextureFormat::RGBA32 => (gl::RGBA, gl::FLOAT),
      };

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

  fn write_texture_data(&self, texture: GraphicsHandle, width: usize, height: usize, pixels: *const u8, internal_format: TextureFormat, pixel_format: TextureFormat, mip_level: usize) {
    unsafe {
      let internal_format = match internal_format {
        TextureFormat::R8 => gl::R8,
        TextureFormat::RG8 => gl::RG8,
        TextureFormat::RGB8 => gl::RGB8,
        TextureFormat::RGBA8 => gl::RGBA8,
        TextureFormat::RGBA32 => gl::RGBA32F,
      };

      let (components, kind) = match pixel_format {
        TextureFormat::R8 => (gl::RED, gl::UNSIGNED_BYTE),
        TextureFormat::RG8 => (gl::RG, gl::UNSIGNED_BYTE),
        TextureFormat::RGB8 => (gl::RGB, gl::UNSIGNED_BYTE),
        TextureFormat::RGBA8 => (gl::RGBA, gl::UNSIGNED_BYTE),
        TextureFormat::RGBA32 => (gl::RGBA, gl::FLOAT),
      };

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
        pixels as *const _
      );
    }
  }

  fn write_texture_sub_data(&self, texture: GraphicsHandle, region: &Rectangle<usize>, pixels: *const u8, pixel_format: TextureFormat, mip_level: usize) {
    unsafe {
      let (components, kind) = match pixel_format {
        TextureFormat::R8 => (gl::RED, gl::UNSIGNED_BYTE),
        TextureFormat::RG8 => (gl::RG, gl::UNSIGNED_BYTE),
        TextureFormat::RGB8 => (gl::RGB, gl::UNSIGNED_BYTE),
        TextureFormat::RGBA8 => (gl::RGBA, gl::UNSIGNED_BYTE),
        TextureFormat::RGBA32 => (gl::RGBA, gl::FLOAT),
      };

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
        pixels as *const _
      );
    }
  }

  fn delete_texture(&self, texture: GraphicsHandle) {
    unsafe {
      gl::DeleteTextures(1, &texture);
    }
  }

  fn create_shader(&self) -> GraphicsHandle {
    unsafe {
      gl::CreateProgram()
    }
  }

  fn link_shaders(&self, shader: GraphicsHandle, shaders: Vec<Shader>) -> crate::Result<()> {
    unsafe {
      gl::UseProgram(shader);

      // compile the shader kernel code
      let mut shader_ids = Vec::with_capacity(shaders.len());

      for Shader { kind, code } in &shaders {
        let shader_id = gl::CreateShader(match kind {
          ShaderKind::Vertex => gl::VERTEX_SHADER,
          ShaderKind::Fragment => gl::FRAGMENT_SHADER,
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

          gl::GetShaderInfoLog(
            shader_id,
            info_log_length,
            std::ptr::null_mut(),
            info_log.as_mut_ptr() as *mut _,
          );

          return Err(anyhow!(String::from_utf8(info_log).unwrap()));
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

        gl::GetProgramInfoLog(
          shader,
          info_log_length,
          std::ptr::null_mut(),
          info_log.as_mut_ptr() as *mut _,
        );

        return Err(anyhow!(String::from_utf8(info_log).unwrap()));
      }

      // delete the kernels now that we've linked
      for shader_id in shader_ids {
        gl::DeleteShader(shader_id);
      }
    }

    Ok(())
  }

  fn get_shader_uniform_location(&self, shader: GraphicsHandle, name: &str) -> Option<usize> {
    unsafe {
      let name = std::ffi::CString::new(name).unwrap();
      let location = gl::GetUniformLocation(shader, name.as_ptr());

      match location {
        -1 => None,
        location => Some(location as usize),
      }
    }
  }

  fn set_shader_uniform(&self, shader: GraphicsHandle, location: usize, value: &ShaderUniform) {
    unsafe {
      match value {
        ShaderUniform::Integer(value) => {
          gl::ProgramUniform1i(shader, location as i32, *value as i32);
        }
        ShaderUniform::Floating(value) => {
          gl::ProgramUniform1f(shader, location as i32, *value);
        }
        ShaderUniform::Point2(value) => {
          gl::ProgramUniform2i(shader, location as i32, value.x, value.y);
        }
        ShaderUniform::Point3(value) => {
          gl::ProgramUniform3i(shader, location as i32, value.x, value.y, value.z);
        }
        ShaderUniform::Point4(value) => {
          gl::ProgramUniform4i(shader, location as i32, value.x, value.y, value.z, value.w);
        }
        ShaderUniform::Vector2(value) => {
          gl::ProgramUniform2f(shader, location as i32, value.x, value.y);
        }
        ShaderUniform::Vector3(value) => {
          gl::ProgramUniform3f(shader, location as i32, value.x, value.y, value.z);
        }
        ShaderUniform::Vector4(value) => {
          gl::ProgramUniform4f(shader, location as i32, value.x, value.y, value.z, value.w);
        }
        ShaderUniform::Matrix2x2(value) => {
          gl::ProgramUniformMatrix2fv(shader, location as i32, 1, gl::TRUE, value.as_slice().as_ptr());
        }
        ShaderUniform::Matrix3x3(value) => {
          gl::ProgramUniformMatrix3fv(shader, location as i32, 1, gl::TRUE, value.as_slice().as_ptr());
        }
        ShaderUniform::Matrix4x4(value) => {
          gl::ProgramUniformMatrix4fv(shader, location as i32, 1, gl::TRUE, value.as_slice().as_ptr());
        }
        ShaderUniform::Texture(texture, slot, sampler) => {
          gl::ActiveTexture(gl::TEXTURE0 + *slot as u32);
          gl::BindTexture(gl::TEXTURE_2D, texture.handle());
          gl::ProgramUniform1i(shader, location as i32, *slot as i32);

          if let Some(sampler) = sampler {
            // build and cache sampler settings based on hash of options
            let mut internal_state = self.internal_state.borrow_mut();
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
          }
        }
      }
    }
  }

  fn set_active_shader(&self, shader: GraphicsHandle) {
    unsafe {
      gl::UseProgram(shader);
    }
  }

  fn delete_shader(&self, shader: GraphicsHandle) {
    unsafe {
      gl::DeleteProgram(shader);
    }
  }

  fn create_mesh(&self, vertex_buffer: GraphicsHandle, index_buffer: GraphicsHandle, descriptors: &[VertexDescriptor]) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;

      gl::GenVertexArrays(1, &mut id);

      gl::BindVertexArray(id);

      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);

      let stride: usize = descriptors
        .iter()
        .map(|it| it.count * it.kind.size())
        .sum();

      let mut offset = 0;

      for index in 0..descriptors.len() {
        let descriptor = descriptors[index];

        let kind = match descriptor.kind {
          VertexKind::U8 => gl::UNSIGNED_BYTE,
          VertexKind::U16 => gl::UNSIGNED_SHORT,
          VertexKind::U32 => gl::UNSIGNED_INT,
          VertexKind::I16 => gl::SHORT,
          VertexKind::I32 => gl::INT,
          VertexKind::F32 => gl::FLOAT,
          VertexKind::F64 => gl::DOUBLE,
        };

        let should_normalize = match descriptor.should_normalize {
          true => gl::TRUE,
          false => gl::FALSE,
        };

        gl::VertexAttribPointer(
          index as u32,
          descriptor.count as i32,
          kind,
          should_normalize,
          stride as i32,
          offset as *const _,
        );
        gl::EnableVertexAttribArray(index as u32);

        offset += descriptor.count * descriptor.kind.size();
      }

      gl::BindVertexArray(0);

      id
    }
  }

  fn draw_mesh(&self, mesh: GraphicsHandle, topology: PrimitiveTopology, vertex_count: usize, index_count: usize) {
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

  fn delete_mesh(&self, mesh: GraphicsHandle) {
    unsafe {
      gl::DeleteVertexArrays(1, &mesh);
    }
  }

  fn create_render_target(&self, color_attachment: GraphicsHandle, depth_attachment: Option<GraphicsHandle>, stencil_attachment: Option<GraphicsHandle>) -> GraphicsHandle {
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

  fn set_active_render_target(&self, render_target: GraphicsHandle) {
    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, render_target);
    }
  }

  fn set_default_render_target(&self) {
    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
  }

  fn delete_render_target(&self, render_target: GraphicsHandle) {
    unsafe {
      gl::DeleteFramebuffers(1, &render_target);
    }
  }
}
