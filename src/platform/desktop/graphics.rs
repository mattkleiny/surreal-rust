use std::ffi::c_void;

use anyhow::anyhow;
use raw_gl_context::{GlConfig, GlContext};
use winit::window::Window;

use crate::graphics::{
  BlendFactor, BlendState, BufferKind, BufferUsage, Color, GraphicsHandle, GraphicsResult,
  GraphicsServer, PrimitiveTopology, Shader, ShaderKind, ShaderUniform, TextureFilter,
  TextureFormat, TextureSampler, TextureWrap, VertexDescriptor, VertexKind,
};

/// The graphics server for the desktop platform.
pub struct DesktopGraphicsServer {
  context: GlContext,
}

impl DesktopGraphicsServer {
  pub fn new(window: &Window, vsync_enabled: bool) -> Self {
    // prepare and load opengl functionality
    let config = GlConfig {
      vsync: vsync_enabled,
      ..Default::default()
    };

    let context = GlContext::create(window, config).unwrap();
    context.make_current();
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    Self { context }
  }
}

impl GraphicsServer for DesktopGraphicsServer {
  fn begin_frame(&self) {
    self.context.make_current();
  }

  fn end_frame(&self) {
    self.context.swap_buffers();
    self.context.make_not_current();
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
        BlendFactor::OneMinusSrcAlpha => gl::ONE_MINUS_SRC_ALPHA,
        BlendFactor::OneMinusSrcColor => gl::ONE_MINUS_SRC_COLOR,
        BlendFactor::OneMinusDstAlpha => gl::ONE_MINUS_DST_ALPHA,
        BlendFactor::OneMinusDstColor => gl::ONE_MINUS_DST_COLOR,
      }
    }

    unsafe {
      match blend_state {
        BlendState::Disabled => gl::Disable(gl::BLEND),
        BlendState::Enabled { source, dest } => {
          let source = convert_blend_factor(source);
          let dest = convert_blend_factor(dest);

          gl::Enable(gl::BLEND);
          gl::BlendFunc(source, dest);
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

  fn flush_commands(&self) {
    unsafe {
      gl::Flush();
    }
  }

  fn create_buffer(&self) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;
      gl::GenBuffers(1, &mut id);
      GraphicsHandle { id }
    }
  }

  fn read_buffer_data(
    &self,
    buffer: GraphicsHandle,
    kind: BufferKind,
    offset: usize,
    length: usize,
  ) -> Vec<u8> {
    unsafe {
      let kind = match kind {
        BufferKind::Element => gl::ARRAY_BUFFER,
        BufferKind::Index => gl::ELEMENT_ARRAY_BUFFER,
        BufferKind::Uniform => gl::UNIFORM_BUFFER,
      };

      let mut data = Vec::with_capacity(length);
      let pointer = data.as_mut_ptr() as *mut c_void;

      gl::BindBuffer(kind, buffer.id);
      gl::BufferSubData(kind, offset as isize, length as isize, pointer);

      data
    }
  }

  fn write_buffer_data(
    &self,
    buffer: GraphicsHandle,
    usage: BufferUsage,
    kind: BufferKind,
    data: *const u8,
    length: usize,
  ) {
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

      gl::BindBuffer(kind, buffer.id);
      gl::BufferData(kind, length as isize, data as *const _, usage);
    }
  }

  fn delete_buffer(&self, buffer: GraphicsHandle) {
    unsafe {
      gl::DeleteBuffers(1, &buffer.id);
    }
  }

  fn create_texture(&self, sampler: &TextureSampler) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;

      gl::GenTextures(1, &mut id);
      gl::BindTexture(gl::TEXTURE_2D, id);

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

      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_mode as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_mode as i32);

      gl::BindTexture(gl::TEXTURE_2D, 0);

      GraphicsHandle { id }
    }
  }

  fn write_texture_data(
    &self,
    texture: GraphicsHandle,
    width: usize,
    height: usize,
    pixels: *const u8,
    format: TextureFormat,
    mip_level: usize,
  ) {
    unsafe {
      let internal_format = match format {
        TextureFormat::RGBA => gl::RGBA32F,
      };

      gl::BindTexture(gl::TEXTURE_2D, texture.id);
      gl::TexImage2D(
        gl::TEXTURE_2D,
        mip_level as i32,
        internal_format as i32,
        width as i32,
        height as i32,
        0, // border
        gl::RGBA,
        gl::FLOAT,
        pixels as *const _,
      );
    }
  }

  fn delete_texture(&self, texture: GraphicsHandle) {
    unsafe {
      gl::DeleteTextures(1, &texture.id);
    }
  }

  fn create_shader(&self) -> GraphicsHandle {
    unsafe {
      let id = gl::CreateProgram();
      GraphicsHandle { id }
    }
  }

  fn link_shaders(&self, shader: GraphicsHandle, shaders: Vec<Shader>) -> GraphicsResult<()> {
    unsafe {
      gl::UseProgram(shader.id);

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

        gl::AttachShader(shader.id, shader_id);
        shader_ids.push(shader_id);
      }

      // link the kernels in the main program
      let mut link_status = 0;

      gl::LinkProgram(shader.id);
      gl::GetProgramiv(shader.id, gl::LINK_STATUS, &mut link_status);

      if link_status == 0 {
        let mut info_log_length = 0;
        gl::GetProgramiv(shader.id, gl::INFO_LOG_LENGTH, &mut info_log_length);

        let mut info_log = Vec::with_capacity(info_log_length as usize);
        info_log.set_len(info_log_length as usize);

        gl::GetProgramInfoLog(
          shader.id,
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
      let name: *const i8 = std::mem::transmute(name.as_bytes().as_ptr());
      let location = gl::GetUniformLocation(shader.id, name);

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
          gl::ProgramUniform1i(shader.id, location as i32, *value as i32);
        }
        ShaderUniform::Floating(value) => {
          gl::ProgramUniform1f(shader.id, location as i32, *value);
        }
        ShaderUniform::Point2(value) => {
          gl::ProgramUniform2i(shader.id, location as i32, value.x, value.y);
        }
        ShaderUniform::Point3(value) => {
          gl::ProgramUniform3i(shader.id, location as i32, value.x, value.y, value.z);
        }
        ShaderUniform::Point4(value) => {
          gl::ProgramUniform4i(
            shader.id,
            location as i32,
            value.x,
            value.y,
            value.z,
            value.w,
          );
        }
        ShaderUniform::Vector2(value) => {
          gl::ProgramUniform2f(shader.id, location as i32, value.x, value.y);
        }
        ShaderUniform::Vector3(value) => {
          gl::ProgramUniform3f(shader.id, location as i32, value.x, value.y, value.z);
        }
        ShaderUniform::Vector4(value) => {
          gl::ProgramUniform4f(
            shader.id,
            location as i32,
            value.x,
            value.y,
            value.z,
            value.w,
          );
        }
        ShaderUniform::Matrix2x2(value) => {
          gl::ProgramUniformMatrix2fv(
            shader.id,
            location as i32,
            1,
            gl::TRUE,
            value.as_slice().as_ptr(),
          );
        }
        ShaderUniform::Matrix3x3(value) => {
          gl::ProgramUniformMatrix3fv(
            shader.id,
            location as i32,
            1,
            gl::TRUE,
            value.as_slice().as_ptr(),
          );
        }
        ShaderUniform::Matrix4x4(value) => {
          gl::ProgramUniformMatrix4fv(
            shader.id,
            location as i32,
            1,
            gl::TRUE,
            value.as_slice().as_ptr(),
          );
        }
        ShaderUniform::Texture(texture, slot, _) => {
          // TODO: process sampler settings, too
          gl::ActiveTexture(gl::TEXTURE0 + *slot as u32);
          gl::BindTexture(gl::TEXTURE_2D, texture.id);
          gl::ProgramUniform1i(shader.id, location as i32, *slot as i32);
        }
      }
    }
  }

  fn set_active_shader(&self, shader: GraphicsHandle) {
    unsafe {
      gl::UseProgram(shader.id);
    }
  }

  fn delete_shader(&self, shader: GraphicsHandle) {
    unsafe {
      gl::DeleteProgram(shader.id);
    }
  }

  fn create_mesh(
    &self,
    vertex_buffer: GraphicsHandle,
    index_buffer: GraphicsHandle,
    descriptors: &[VertexDescriptor],
  ) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;
      gl::GenVertexArrays(1, &mut id);

      gl::BindVertexArray(id);
      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer.id);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.id);

      let stride: usize = descriptors
        .iter()
        .map(|descriptor| descriptor.count * descriptor.kind.size())
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

      GraphicsHandle { id }
    }
  }

  fn draw_mesh(
    &self,
    mesh: GraphicsHandle,
    topology: PrimitiveTopology,
    vertex_count: usize,
    index_count: usize,
  ) {
    // TODO: variable index type?

    unsafe {
      gl::BindVertexArray(mesh.id);

      let topology = match topology {
        PrimitiveTopology::Points => gl::POINTS,
        PrimitiveTopology::Lines => gl::LINES,
        PrimitiveTopology::Triangles => gl::TRIANGLES,
      };

      if index_count > 0 {
        gl::DrawElements(
          topology,
          index_count as i32,
          gl::UNSIGNED_INT,
          std::ptr::null(),
        );
      } else {
        gl::DrawArrays(topology, 0, vertex_count as i32);
      }
    }
  }

  fn delete_mesh(&self, mesh: GraphicsHandle) {
    unsafe {
      gl::DeleteVertexArrays(1, &mesh.id);
    }
  }
}
