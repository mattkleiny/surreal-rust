//! Graphics backend for SDL2.

use std::{
  ffi::{c_void, CString},
  sync::RwLock,
};

use common::{Color, FastHashMap, Rectangle, Size, UVec2};
pub use graphics::*;

/// A graphics backend for SDL2.
pub struct SdlGraphicsBackend {
  sampler_cache: RwLock<FastHashMap<TextureSampler, u32>>,
}

impl SdlGraphicsBackend {
  /// Creates a new OpenGL graphics backend.
  pub fn new() -> Self {
    gl::load_with(|symbol| unsafe {
      let name = CString::new(symbol).unwrap();
      sdl2_sys::SDL_GL_GetProcAddress(name.as_ptr() as *const _) as *const _
    });

    Self {
      sampler_cache: RwLock::new(FastHashMap::default()),
    }
  }
}

impl GraphicsBackend for SdlGraphicsBackend {
  fn begin_frame(&self) {
    // no-op
  }

  fn end_frame(&self) {
    // no-op
  }

  fn clear_color_buffer(&self, color: Color) {
    unsafe {
      gl::ClearColor(color.r, color.g, color.b, color.a);
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
  }

  fn clear_depth_buffer(&self, _depth: f32) {
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

  fn set_viewport_size(&self, size: UVec2) {
    if size.x > 0 && size.y > 0 {
      unsafe {
        gl::Viewport(0, 0, size.x as i32, size.y as i32);
      }
    }
  }

  fn set_blend_state(&self, blend_state: BlendState) {
    fn convert_blend_factor(factor: BlendFactor) -> u32 {
      match factor {
        BlendFactor::One => gl::ONE,
        BlendFactor::SourceAlpha => gl::SRC_ALPHA,
        BlendFactor::SourceColor => gl::SRC_COLOR,
        BlendFactor::DestinationAlpha => gl::DST_ALPHA,
        BlendFactor::DestinationColor => gl::DST_COLOR,
        BlendFactor::OneMinusSourceAlpha => gl::ONE_MINUS_SRC_ALPHA,
        BlendFactor::OneMinusSourceColor => gl::ONE_MINUS_SRC_COLOR,
        BlendFactor::OneMinusDestinationAlpha => gl::ONE_MINUS_DST_ALPHA,
        BlendFactor::OneMinusDestinationColor => gl::ONE_MINUS_DST_COLOR,
      }
    }

    unsafe {
      match blend_state {
        BlendState::Disabled => gl::Disable(gl::BLEND),
        BlendState::Enabled {
          source,
          destination: dest,
        } => {
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

  fn buffer_create(&self) -> Result<BufferId, BufferError> {
    unsafe {
      let mut id: u32 = 0;

      gl::GenBuffers(1, &mut id);

      Ok(BufferId::from(id))
    }
  }

  fn buffer_read_data(
    &self,
    buffer: BufferId,
    offset: usize,
    length: usize,
    pointer: *mut u8,
  ) -> Result<(), BufferError> {
    unsafe {
      if length == 0 {
        return Ok(());
      }

      if length > 0 && pointer.is_null() {
        return Err(BufferError::NullPointer);
      }

      gl::GetNamedBufferSubData(buffer.into(), offset as isize, length as isize, pointer as *mut c_void);

      Ok(())
    }
  }

  fn buffer_write_data(
    &self,
    buffer: BufferId,
    usage: BufferUsage,
    kind: BufferKind,
    length: usize,
    pointer: *const u8,
  ) -> Result<(), BufferError> {
    unsafe {
      let kind = match kind {
        BufferKind::Element => gl::ARRAY_BUFFER,
        BufferKind::Index => gl::ELEMENT_ARRAY_BUFFER,
      };

      let usage = match usage {
        BufferUsage::Static => gl::STATIC_DRAW,
        BufferUsage::Dynamic => gl::DYNAMIC_DRAW,
      };

      gl::BindBuffer(kind, buffer.into());
      gl::BufferData(kind, length as isize, pointer as *const _, usage);

      Ok(())
    }
  }

  fn buffer_delete(&self, buffer: BufferId) -> Result<(), BufferError> {
    unsafe {
      gl::DeleteBuffers(1, &buffer.into());

      Ok(())
    }
  }

  fn texture_create(&self, sampler: &TextureSampler) -> Result<TextureId, TextureError> {
    unsafe {
      let mut id: u32 = 0;

      gl::GenTextures(1, &mut id);
      gl::BindTexture(gl::TEXTURE_2D, id);

      let id = TextureId::from(id);

      self.texture_set_options(id, sampler)?;

      Ok(id)
    }
  }

  fn texture_set_options(&self, texture: TextureId, sampler: &TextureSampler) -> Result<(), TextureError> {
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

      gl::BindTexture(gl::TEXTURE_2D, texture.into());

      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_mode as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_mode as i32);

      Ok(())
    }
  }

  fn texture_initialize(
    &self,
    texture: TextureId,
    width: u32,
    height: u32,
    format: TextureFormat,
  ) -> Result<(), TextureError> {
    unsafe {
      let (components, kind) = convert_texture_format(format);

      gl::BindTexture(gl::TEXTURE_2D, texture.into());
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

      Ok(())
    }
  }

  fn texture_read_data(
    &self,
    texture: TextureId,
    length: usize,
    pixel_format: TextureFormat,
    pixels: *mut u8,
    mip_level: usize,
  ) -> Result<(), TextureError> {
    unsafe {
      let (components, kind) = convert_texture_format(pixel_format);

      gl::BindTexture(gl::TEXTURE_2D, texture.into());
      gl::GetnTexImage(
        gl::TEXTURE_2D,
        mip_level as i32,
        components,
        kind,
        length as i32,
        pixels as *mut c_void,
      );

      Ok(())
    }
  }

  fn texture_write_data(
    &self,
    texture: TextureId,
    width: u32,
    height: u32,
    pixels: *const u8,
    internal_format: TextureFormat,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) -> Result<(), TextureError> {
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

      gl::BindTexture(gl::TEXTURE_2D, texture.into());
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

      Ok(())
    }
  }

  fn texture_write_sub_data(
    &self,
    texture: TextureId,
    region: &Rectangle,
    pixels: *const u8,
    pixel_format: TextureFormat,
    mip_level: usize,
  ) -> Result<(), TextureError> {
    unsafe {
      let (components, kind) = convert_texture_format(pixel_format);

      gl::BindTexture(gl::TEXTURE_2D, texture.into());
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

      Ok(())
    }
  }

  fn texture_delete(&self, texture: TextureId) -> Result<(), TextureError> {
    unsafe {
      gl::DeleteTextures(1, &texture.into());

      Ok(())
    }
  }

  fn shader_create(&self) -> Result<ShaderId, ShaderError> {
    Ok(ShaderId::from(unsafe { gl::CreateProgram() }))
  }

  #[allow(clippy::uninit_vec)]
  fn shader_link(&self, shader: ShaderId, shaders: &[ShaderKernel]) -> Result<(), ShaderError> {
    unsafe {
      let shader = shader.into();

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

          gl::GetShaderInfoLog(
            shader_id,
            info_log_length,
            std::ptr::null_mut(),
            info_log.as_mut_ptr() as *mut _,
          );

          return Err(ShaderError::CompileError(String::from_utf8(info_log).unwrap()));
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

        return Err(ShaderError::CompileError(String::from_utf8(info_log).unwrap()));
      }

      // TODO: parse the shader kernels for metadata

      // delete the kernels now that we've linked
      for shader_id in shader_ids {
        gl::DeleteShader(shader_id);
      }
    }

    Ok(())
  }

  fn shader_uniform_location(&self, shader: ShaderId, name: &str) -> Option<usize> {
    unsafe {
      let shader = shader.into();
      let name = CString::new(name).unwrap();
      let location = gl::GetUniformLocation(shader, name.as_ptr());

      match location {
        -1 => None,
        location => Some(location as usize),
      }
    }
  }

  fn shader_set_uniform(&self, shader: ShaderId, location: usize, value: &ShaderUniform) -> Result<(), ShaderError> {
    unsafe {
      let shader_id = shader.into();

      match value {
        ShaderUniform::Bool(value) => {
          gl::ProgramUniform1i(shader_id, location as i32, *value as i32);
        }
        ShaderUniform::I32(value) => {
          gl::ProgramUniform1i(shader_id, location as i32, *value);
        }
        ShaderUniform::U32(value) => {
          gl::ProgramUniform1i(shader_id, location as i32, *value as i32);
        }
        ShaderUniform::F32(value) => {
          gl::ProgramUniform1f(shader_id, location as i32, *value);
        }
        ShaderUniform::Vec2(value) => {
          gl::ProgramUniform2f(shader_id, location as i32, value.x, value.y);
        }
        ShaderUniform::Vec3(value) => {
          gl::ProgramUniform3f(shader_id, location as i32, value.x, value.y, value.z);
        }
        ShaderUniform::Vec4(value) => {
          gl::ProgramUniform4f(shader_id, location as i32, value.x, value.y, value.z, value.w);
        }
        ShaderUniform::DVec2(value) => gl::ProgramUniform2d(shader_id, location as i32, value.x, value.y),
        ShaderUniform::DVec3(value) => gl::ProgramUniform3d(shader_id, location as i32, value.x, value.y, value.z),
        ShaderUniform::DVec4(value) => {
          gl::ProgramUniform4d(shader_id, location as i32, value.x, value.y, value.z, value.w)
        }
        ShaderUniform::Mat2(value) => {
          gl::ProgramUniformMatrix2fv(shader_id, location as i32, 1, gl::FALSE, &value.to_cols_array()[0]);
        }
        ShaderUniform::Mat3(value) => {
          gl::ProgramUniformMatrix3fv(shader_id, location as i32, 1, gl::FALSE, &value.to_cols_array()[0]);
        }
        ShaderUniform::Mat4(value) => {
          gl::ProgramUniformMatrix4fv(shader_id, location as i32, 1, gl::FALSE, &value.to_cols_array()[0]);
        }
        ShaderUniform::DMat2(value) => {
          gl::ProgramUniformMatrix2dv(shader_id, location as i32, 1, gl::FALSE, &value.to_cols_array()[0]);
        }
        ShaderUniform::DMat3(value) => {
          gl::ProgramUniformMatrix3dv(shader_id, location as i32, 1, gl::FALSE, &value.to_cols_array()[0]);
        }
        ShaderUniform::DMat4(value) => {
          gl::ProgramUniformMatrix4dv(shader_id, location as i32, 1, gl::FALSE, &value.to_cols_array()[0]);
        }
        ShaderUniform::Quat(value) => {
          gl::ProgramUniform4f(shader_id, location as i32, value.x, value.y, value.z, value.w);
        }
        ShaderUniform::DQuat(value) => {
          gl::ProgramUniform4d(shader_id, location as i32, value.x, value.y, value.z, value.w);
        }
        ShaderUniform::Color(color) => {
          gl::ProgramUniform4f(shader_id, location as i32, color.r, color.g, color.b, color.a);
        }
        ShaderUniform::Color32(color) => {
          gl::ProgramUniform4ui(
            shader_id,
            location as i32,
            color.r as u32,
            color.g as u32,
            color.b as u32,
            color.a as u32,
          );
        }
        ShaderUniform::Texture(texture, slot, sampler) => {
          gl::ActiveTexture(gl::TEXTURE0 + *slot as u32);
          gl::BindTexture(gl::TEXTURE_2D, (*texture).into());
          gl::ProgramUniform1i(shader_id, location as i32, *slot as i32);

          if let Some(sampler) = sampler {
            let mut sampler_cache = self.sampler_cache.write().unwrap();

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
        ShaderUniform::TextureArray(entries) => {
          let start_index = location;
          let texture_ids = entries.iter().map(|entry| (*entry).into()).collect::<Vec<_>>();

          gl::ProgramUniform1uiv(
            shader_id,
            start_index as i32,
            entries.len() as i32,
            texture_ids.as_ptr() as *const _,
          );
        }
        ShaderUniform::Array(_entries) => {
          todo!()
        }
      };

      Ok(())
    }
  }

  fn shader_activate(&self, shader: ShaderId) -> Result<(), ShaderError> {
    unsafe {
      gl::UseProgram(shader.into());

      Ok(())
    }
  }

  fn shader_dispatch_compute(&self, shader: ShaderId, x: u32, y: u32, z: u32) -> Result<(), ShaderError> {
    unsafe {
      gl::UseProgram(shader.into());
      gl::DispatchCompute(x, y, z);

      Ok(())
    }
  }

  fn shader_memory_barrier(&self, barrier: MemoryBarrier) -> Result<(), ShaderError> {
    unsafe {
      gl::MemoryBarrier(match barrier {
        MemoryBarrier::ImageAccess => gl::SHADER_IMAGE_ACCESS_BARRIER_BIT,
      });

      Ok(())
    }
  }

  fn shader_delete(&self, shader: ShaderId) -> Result<(), ShaderError> {
    unsafe {
      gl::DeleteProgram(shader.into());

      Ok(())
    }
  }

  fn mesh_create(
    &self,
    vertex_buffer: BufferId,
    index_buffer: BufferId,
    descriptors: &[VertexDescriptor],
  ) -> Result<MeshId, MeshError> {
    unsafe {
      let mut id: u32 = 0;

      gl::GenVertexArrays(1, &mut id);
      gl::BindVertexArray(id);

      gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer.into());
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.into());

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

      Ok(MeshId::from(id))
    }
  }

  fn mesh_draw(
    &self,
    mesh: MeshId,
    topology: PrimitiveTopology,
    vertex_count: usize,
    index_count: usize,
  ) -> Result<(), MeshError> {
    unsafe {
      gl::BindVertexArray(mesh.into());

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

      Ok(())
    }
  }

  fn mesh_delete(&self, mesh: MeshId) -> Result<(), MeshError> {
    unsafe {
      gl::DeleteVertexArrays(1, &mesh.into());

      Ok(())
    }
  }

  fn target_create(
    &self,
    color_attachment: TextureId,
    depth_attachment: Option<TextureId>,
    stencil_attachment: Option<TextureId>,
  ) -> Result<TargetId, TargetError> {
    unsafe {
      let mut framebuffer = 0;
      gl::CreateFramebuffers(1, &mut framebuffer);

      gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer);
      gl::FramebufferTexture2D(
        gl::FRAMEBUFFER,
        gl::COLOR_ATTACHMENT0,
        gl::TEXTURE_2D,
        color_attachment.into(),
        0,
      );

      if let Some(depth_attachment) = depth_attachment {
        gl::FramebufferTexture2D(
          gl::FRAMEBUFFER,
          gl::DEPTH_ATTACHMENT,
          gl::TEXTURE_2D,
          depth_attachment.into(),
          0,
        );
      }

      if let Some(stencil_attachment) = stencil_attachment {
        gl::FramebufferTexture2D(
          gl::FRAMEBUFFER,
          gl::STENCIL_ATTACHMENT,
          gl::TEXTURE_2D,
          stencil_attachment.into(),
          0,
        );
      }

      if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
        panic!("Failed to create render target");
      }

      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

      Ok(TargetId::from(framebuffer))
    }
  }

  fn target_activate(&self, target: TargetId) -> Result<(), TargetError> {
    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, target.into());

      Ok(())
    }
  }

  fn target_set_default(&self) -> Result<(), TargetError> {
    unsafe {
      gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

      Ok(())
    }
  }

  fn target_blit_to_active(
    &self,
    target: TargetId,
    source_rect: Option<Rectangle>,
    dest_rect: Option<Rectangle>,
    filter: TextureFilter,
  ) -> Result<(), TargetError> {
    unsafe {
      gl::BindFramebuffer(gl::READ_FRAMEBUFFER, target.into());
      gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, TargetId::NONE.into());

      let source_rect = source_rect.unwrap_or(Rectangle::from_corner_points(0., 0., 1., 1.));
      let dest_rect = dest_rect.unwrap_or(Rectangle::from_corner_points(0., 0., 1., 1.));

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

      Ok(())
    }
  }

  fn target_delete(&self, target: TargetId) -> Result<(), TargetError> {
    unsafe {
      gl::DeleteFramebuffers(1, &target.into());

      Ok(())
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
