use std::ffi::c_void;
use anyhow::anyhow;

use raw_gl_context::{GlConfig, GlContext};
use winit::window::Window;

use crate::graphics::{BlendState, BufferKind, BufferUsage, Color, GraphicsHandle, GraphicsResult, GraphicsServer, Shader, ShaderKind, TextureFilter, TextureFormat, TextureWrap, VertexDescriptor};

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
      gl::Viewport(0, 0, width as i32, height as i32);
    }
  }

  fn set_blend_state(&self, _blend_state: BlendState) {
    todo!()
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

  fn read_buffer_data(&self, buffer: GraphicsHandle, kind: BufferKind, offset: usize, length: usize) -> Vec<u8> {
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

  fn write_buffer_data(&self, buffer: GraphicsHandle, usage: BufferUsage, kind: BufferKind, data: &[u8]) {
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

      let size = data.len() as isize;
      let pointer = data.as_ptr() as *const c_void;

      gl::BindBuffer(kind, buffer.id);
      gl::BufferData(kind, size, pointer, usage);
    }
  }

  fn delete_buffer(&self, buffer: GraphicsHandle) {
    unsafe {
      gl::DeleteBuffers(1, &buffer.id);
    }
  }

  fn create_texture(&self, minify_filter: TextureFilter, magnify_filter: TextureFilter, wrap_mode: TextureWrap) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;
      let target = gl::TEXTURE_2D;

      gl::GenTextures(1, &mut id);
      gl::BindTexture(target, id);

      let minify_filter = match minify_filter {
        TextureFilter::Nearest => gl::NEAREST,
        TextureFilter::Linear => gl::LINEAR,
      };

      let magnify_filter = match magnify_filter {
        TextureFilter::Nearest => gl::NEAREST,
        TextureFilter::Linear => gl::LINEAR,
      };

      let wrap_mode = match wrap_mode {
        TextureWrap::Clamp => gl::CLAMP_TO_EDGE,
        TextureWrap::Mirror => gl::MIRRORED_REPEAT,
      };

      gl::TexParameteri(target, gl::TEXTURE_MIN_FILTER, minify_filter as i32);
      gl::TexParameteri(target, gl::TEXTURE_MAG_FILTER, magnify_filter as i32);
      gl::TexParameteri(target, gl::TEXTURE_WRAP_S, wrap_mode as i32);
      gl::TexParameteri(target, gl::TEXTURE_WRAP_T, wrap_mode as i32);

      GraphicsHandle { id }
    }
  }

  fn write_texture_data(&self, texture: GraphicsHandle, width: usize, height: usize, pixels: &[u8], format: TextureFormat, mip_level: usize) {
    unsafe {
      let target = gl::TEXTURE_2D;
      let internal_format = match format {
        TextureFormat::RGBA => gl::RGBA32F
      };

      let pointer = pixels.as_ptr() as *const c_void;

      gl::BindTexture(target, texture.id);
      gl::TexImage2D(target, mip_level as i32, internal_format as i32, width as i32, height as i32, 0, gl::RGBA, gl::FLOAT, pointer);
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

          gl::GetShaderInfoLog(shader_id, info_log_length, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut _);

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

        gl::GetProgramInfoLog(shader.id, info_log_length, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut _);

        return Err(anyhow!(String::from_utf8(info_log).unwrap()));
      }

      // delete the kernels now that we've linked
      for shader_id in shader_ids {
        gl::DeleteShader(shader_id);
      }
    }

    Ok(())
  }

  fn delete_shader(&self, shader: GraphicsHandle) {
    unsafe {
      gl::DeleteProgram(shader.id);
    }
  }

  fn create_mesh(&self, _descriptors: &[VertexDescriptor]) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;
      gl::GenVertexArrays(1, &mut id);
      GraphicsHandle { id };

      todo!()
    }
  }

  fn delete_mesh(&self, mesh: GraphicsHandle) {
    unsafe {
      gl::DeleteVertexArrays(1, &mesh.id);
    }
  }
}
