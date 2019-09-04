//! The OpenGL SPI for the Surreal graphics engine.
//!
//! This implementation is based directly on the unsafe 'gl' crate and is not meant to achieve
//! Rust-grade 'safety', but is instead designed to be fast and robust for the purpose of building
//! small games and applications.

use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint, GLvoid};
use sdl2::video::GLContext;

use crate::diagnostics::*;

use super::*;

/// Guards the given expression against a GL assertion and panics if it fails.
macro_rules! checked (
  ($func:expr) => ({
    let result = $func;
    glassert();
    result
  });
);

/// The OpenGL graphics device implementation.
///
/// This require a valid context object on the active window, which is usually provided via SDL2.
///
/// No lifetime guarantees are made across instances fabricated by this device; this means it's
/// possible for a shader/texture/etc resource to outlive the lifecycle of this device, causing
/// an error after it's dropped.
pub struct OpenGLGraphicsDevice {
  context: GLContext,
  default_framebuffer: GLuint,
}

impl OpenGLGraphicsDevice {
  pub fn new(context: GLContext, default_framebuffer: GLuint) -> Self {
    Self {
      context,
      default_framebuffer,
    }
  }

  pub fn set_default_framebuffer(&mut self, framebuffer: GLuint) {
    self.default_framebuffer = framebuffer;
  }

  unsafe fn set_texture_parameters(&self, texture: &OpenGLTexture) {
    self.bind_texture(texture, 0);

    checked!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint));
    checked!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint));
    checked!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint));
    checked!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint));
  }

  unsafe fn set_render_state(&self, render_state: &RenderState<Self>) {
    self.bind_render_target(render_state.render_target);

    let (origin, size) = (
      render_state.viewport.origin(),
      render_state.viewport.size()
    );

    gl::Viewport(origin.x, origin.y, size.x, size.y);

    if render_state.rasterizer.clear_ops.has_ops() {
      self.clear_render_target(&render_state.rasterizer.clear_ops);
    }

    self.use_program(render_state.shader_program);
    self.bind_vertex_array(render_state.vertex_array);

    for (texture_unit, texture) in render_state.textures.iter().enumerate() {
      self.bind_texture(texture, texture_unit as u32);
    }

    render_state.uniforms.iter().for_each(|(uniform, data)|
        self.set_uniform(uniform, data)
    );

    self.set_render_options(&render_state.rasterizer);
  }

  unsafe fn set_render_options(&self, render_options: &RasterizerState) {
    match render_options.blend {
      BlendState::Off => {
        checked!(gl::Disable(gl::BLEND));
      }
      BlendState::RGBOneAlphaOne => {
        checked!(gl::BlendEquation(gl::FUNC_ADD));
        checked!(gl::BlendFunc(gl::ONE, gl::ONE));
        checked!(gl::Enable(gl::BLEND));
      }
      BlendState::RGBOneAlphaOneMinusSrcAlpha => {
        checked!(gl::BlendEquation(gl::FUNC_ADD));
        checked!(gl::BlendFuncSeparate(gl::ONE, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ONE));
        checked!(gl::Enable(gl::BLEND));
      }
      BlendState::RGBSrcAlphaAlphaOneMinusSrcAlpha => {
        checked!(gl::BlendEquation(gl::FUNC_ADD));
        checked!(gl::BlendFuncSeparate(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ONE));
        checked!(gl::Enable(gl::BLEND));
      }
    }

    match render_options.depth {
      None => {
        checked!(gl::Disable(gl::DEPTH_TEST));
      }
      Some(ref state) => {
        checked!(gl::DepthFunc(state.func.to_gl_depth_func()));
        checked!(gl::DepthMask(state.write as GLboolean));
        checked!(gl::Enable(gl::DEPTH_TEST));
      }
    }

    match render_options.stencil {
      None => {
        checked!(gl::Disable(gl::STENCIL_TEST));
      }
      Some(ref state) => {
        checked!(gl::StencilFunc(
          state.func.to_gl_stencil_func(),
          state.reference as GLint,
          state.mask,
        ));

        let (pass_action, write_mask) = if state.write {
          (gl::REPLACE, state.mask)
        } else {
          (gl::KEEP, 0)
        };

        checked!(gl::StencilOp(gl::KEEP, gl::KEEP, pass_action));
        checked!(gl::StencilMask(write_mask));
        checked!(gl::Enable(gl::STENCIL_TEST));
      }
    }

    let color_mask = render_options.color_mask as GLboolean;

    checked!(gl::ColorMask(color_mask, color_mask, color_mask, color_mask));
  }

  unsafe fn set_uniform(&self, uniform: &OpenGLUniform, data: &UniformData) {
    match *data {
      UniformData::Int(value) => {
        checked!(gl::Uniform1i(uniform.location, value));
      }
      UniformData::Mat4(data) => {
        assert_eq!(std::mem::size_of::<Mat4>(), 4 * 4 * 4);
        let data_ptr: *const f32 = data.as_ref().as_ptr();
        checked!(gl::UniformMatrix4fv(uniform.location, 1, gl::FALSE, data_ptr as *const GLfloat));
      }
      UniformData::Vec2(data) => {
        checked!(gl::Uniform2f(uniform.location, data.x, data.y));
      }
      UniformData::Vec4(data) => {
        checked!(gl::Uniform4f(uniform.location, data.x, data.y, data.z, data.w));
      }
      UniformData::TextureUnit(unit) => {
        checked!(gl::Uniform1i(uniform.location, unit as GLint));
      }
    }
  }

  unsafe fn reset_render_state(&self, render_state: &RenderState<Self>) {
    self.reset_render_options(&render_state.rasterizer);

    for texture_unit in 0..(render_state.textures.len() as u32) {
      self.unbind_texture(texture_unit);
    }

    self.unuse_program();
    self.unbind_vertex_array();
  }

  unsafe fn reset_render_options(&self, render_options: &RasterizerState) {
    match render_options.blend {
      BlendState::Off => {}
      BlendState::RGBOneAlphaOneMinusSrcAlpha |
      BlendState::RGBOneAlphaOne |
      BlendState::RGBSrcAlphaAlphaOneMinusSrcAlpha => {
        checked!(gl::Disable(gl::BLEND));
      }
    }

    if render_options.depth.is_some() {
      checked!(gl::Disable(gl::DEPTH_TEST));
    }

    if render_options.stencil.is_some() {
      checked!(gl::StencilMask(!0));
      checked!(gl::Disable(gl::STENCIL_TEST));
    }

    checked!(gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE));
  }

  unsafe fn bind_render_target(&self, attachment: &RenderTarget<Self>) {
    match *attachment {
      RenderTarget::Default => self.bind_default_framebuffer(),
      RenderTarget::Framebuffer(framebuffer) => self.bind_framebuffer(framebuffer),
    }
  }

  unsafe fn bind_vertex_array(&self, vertex_array: &OpenGLVertexArray) {
    checked!(gl::BindVertexArray(vertex_array.id));
  }

  unsafe fn unbind_vertex_array(&self) {
    checked!(gl::BindVertexArray(0));
  }

  unsafe fn bind_texture(&self, texture: &OpenGLTexture, unit: u32) {
    checked!(gl::ActiveTexture(gl::TEXTURE0 + unit));
    checked!(gl::BindTexture(gl::TEXTURE_2D, texture.id));
  }

  unsafe fn unbind_texture(&self, unit: u32) {
    checked!(gl::ActiveTexture(gl::TEXTURE0 + unit));
    checked!(gl::BindTexture(gl::TEXTURE_2D, 0));
  }

  unsafe fn use_program(&self, program: &OpenGLProgram) {
    checked!(gl::UseProgram(program.id));
  }

  unsafe fn unuse_program(&self) {
    checked!(gl::UseProgram(0));
  }

  unsafe fn bind_default_framebuffer(&self) {
    checked!(gl::BindFramebuffer(gl::FRAMEBUFFER, self.default_framebuffer));
  }

  unsafe fn bind_framebuffer(&self, framebuffer: &OpenGLFramebuffer) {
    checked!(gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer.id));
  }

  unsafe fn render_target_format(&self, render_target: &RenderTarget<Self>) -> TextureFormat {
    match *render_target {
      RenderTarget::Default => TextureFormat::RGBA8,
      RenderTarget::Framebuffer(ref framebuffer) => {
        self.get_framebuffer_texture(framebuffer).format
      }
    }
  }
}

impl GraphicsDevice for OpenGLGraphicsDevice {
  type Buffer = OpenGLBuffer;
  type Framebuffer = OpenGLFramebuffer;
  type Program = OpenGLProgram;
  type Shader = OpenGLShader;
  type Texture = OpenGLTexture;
  type Uniform = OpenGLUniform;
  type VertexArray = OpenGLVertexArray;
  type VertexAttr = OpenGLVertexAttr;

  unsafe fn get_vertex_attr(&self, program: &Self::Program, name: &str) -> Option<Self::VertexAttr> {
    let name = std::ffi::CString::new(format!("a{}", name)).unwrap();
    let attr = checked!(gl::GetAttribLocation(program.id, name.as_ptr() as *const GLchar));

    if attr < 0 {
      None
    } else {
      Some(Self::VertexAttr { attr: attr as GLuint })
    }
  }

  unsafe fn get_uniform(&self, program: &Self::Program, name: &str) -> Self::Uniform {
    let name = std::ffi::CString::new(format!("u{}", name)).unwrap();
    let location = checked!(gl::GetUniformLocation(program.id, name.as_ptr() as *const GLchar));

    Self::Uniform { location }
  }

  unsafe fn bind_buffer(&self, vertex_array: &Self::VertexArray, buffer: &Self::Buffer, target: BufferTarget) {
    self.bind_vertex_array(vertex_array);

    checked!(gl::BindBuffer(target.to_gl_target(), buffer.id));

    self.unbind_vertex_array();
  }

  unsafe fn configure_vertex_attr(&self, vertex_array: &Self::VertexArray, attr: &Self::VertexAttr, descriptor: &VertexAttrDescriptor) {
    debug_assert_ne!(descriptor.stride, 0);

    self.bind_vertex_array(vertex_array);
    let attr_type = descriptor.attr_type.to_gl_type();

    match descriptor.class {
      VertexAttrClass::Float | VertexAttrClass::FloatNorm => {
        let normalized = if descriptor.class == VertexAttrClass::FloatNorm { gl::TRUE } else { gl::FALSE };
        checked!(gl::VertexAttribPointer(
          attr.attr,
          descriptor.size as GLint,
          attr_type,
          normalized,
          descriptor.stride as GLint,
          descriptor.offset as *const GLvoid,
        ));
      }
      VertexAttrClass::Int => {
        checked!(gl::VertexAttribIPointer(
          attr.attr,
          descriptor.size as GLint,
          attr_type,
          descriptor.stride as GLint,
          descriptor.offset as *const GLvoid,
        ));
      }
    }

    checked!(gl::VertexAttribDivisor(attr.attr, descriptor.divisor));
    checked!(gl::EnableVertexAttribArray(attr.attr));

    self.unbind_vertex_array();
  }

  unsafe fn create_framebuffer(&self, texture: Self::Texture) -> Self::Framebuffer {
    let mut id = 0;

    checked!(gl::GenFramebuffers(1, &mut id));
    checked!(gl::BindFramebuffer(gl::FRAMEBUFFER, id));

    self.bind_texture(&texture, 0);

    checked!(gl::FramebufferTexture2D(
      gl::FRAMEBUFFER,
      gl::COLOR_ATTACHMENT0,
      gl::TEXTURE_2D,
      texture.id,
      0,
    ));

    assert_eq!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER), gl::FRAMEBUFFER_COMPLETE);

    Self::Framebuffer { id, texture }
  }

  unsafe fn create_buffer(&self) -> Self::Buffer {
    let mut id = 0;

    checked!(gl::GenBuffers(1, &mut id));

    Self::Buffer { id }
  }

  unsafe fn upload_to_buffer<T>(&self, buffer: &Self::Buffer, data: BufferData<T>, target: BufferTarget, mode: BufferUploadMode) {
    let target = match target {
      BufferTarget::Vertex => gl::ARRAY_BUFFER,
      BufferTarget::Index => gl::ELEMENT_ARRAY_BUFFER,
    };

    let (ptr, len) = match data {
      BufferData::Uninitialized(len) => (std::ptr::null(), len),
      BufferData::Memory(buffer) => (buffer.as_ptr() as *const GLvoid, buffer.len()),
    };

    let len = (len * std::mem::size_of::<T>()) as GLsizeiptr;
    let usage = mode.to_gl_usage();

    checked!(gl::BindBuffer(target, buffer.id));
    checked!(gl::BufferData(target, len, ptr, usage));
  }

  unsafe fn create_shader_from_source(&self, source: &[u8], kind: ShaderKind) -> Self::Shader {
    let kind = match kind {
      ShaderKind::Vertex => gl::VERTEX_SHADER,
      ShaderKind::Fragment => gl::FRAGMENT_SHADER,
    };

    let id = checked!(gl::CreateShader(kind));
    checked!(gl::ShaderSource(id, 1, [source.as_ptr() as *const GLchar].as_ptr(), [source.len() as GLint].as_ptr()));
    checked!(gl::CompileShader(id));

    let mut compile_status = 0;
    gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut compile_status);

    if compile_status != gl::TRUE as GLint {
      let mut info_log_length = 0;
      gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut info_log_length);

      let mut info_log = vec![0; info_log_length as usize];
      gl::GetShaderInfoLog(id, info_log.len() as GLint, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);

      error!("Shader info log:\n{}", String::from_utf8_lossy(&info_log));
      panic!("{:?} shader compilation failed", kind);
    }

    Self::Shader { id }
  }

  unsafe fn create_vertex_array(&self) -> Self::VertexArray {
    let mut array = Self::VertexArray { id: 0 };

    checked!(gl::GenVertexArrays(1, &mut array.id));

    array
  }

  unsafe fn create_program_from_shaders(&self, vertex_shader: Self::Shader, fragment_shader: Self::Shader) -> Self::Program {
    let id = checked!(gl::CreateProgram());

    checked!(gl::AttachShader(id, vertex_shader.id));
    checked!(gl::AttachShader(id, fragment_shader.id));
    checked!(gl::LinkProgram(id));

    let mut link_status = 0;
    gl::GetProgramiv(id, gl::LINK_STATUS, &mut link_status);

    if link_status != gl::TRUE as GLint {
      let mut info_log_length = 0;
      gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut info_log_length);

      let mut info_log = vec![0; info_log_length as usize];
      gl::GetProgramInfoLog(id, info_log.len() as GLint, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);

      eprintln!("Program info log:\n{}", String::from_utf8_lossy(&info_log));
      panic!("Program linking failed");
    }

    Self::Program { id, vertex_shader, fragment_shader }
  }

  unsafe fn get_framebuffer_texture<'f>(&self, framebuffer: &'f Self::Framebuffer) -> &'f Self::Texture {
    &framebuffer.texture
  }

  unsafe fn read_pixels(&self, target: &RenderTarget<Self>, viewport: RectI) -> TextureData {
    let (origin, size) = (viewport.origin(), viewport.size());

    let format = self.render_target_format(target);
    self.bind_render_target(target);

    match format {
      TextureFormat::RGB8 | TextureFormat::RGBA8 => {
        let channels = format.channels();
        let mut pixels = vec![0; size.x as usize * size.y as usize * channels];

        checked!(gl::ReadPixels(
          origin.x,
          origin.y,
          size.x as GLsizei,
          size.y as GLsizei,
          format.to_gl_format(),
          format.to_gl_type(),
          pixels.as_mut_ptr() as *mut GLvoid,
        ));

        // flip them bits upside down; because opengl samples bottom to top
        flip_y(&mut pixels, size, channels);

        TextureData::U8(pixels)
      }
    }
  }

  unsafe fn create_texture(&self, format: TextureFormat, size: Vec2i) -> Self::Texture {
    let mut texture = OpenGLTexture { id: 0, size, format };

    checked!(gl::GenTextures(1, &mut texture.id));
    self.bind_texture(&texture, 0);


    checked!(gl::TexImage2D(
      gl::TEXTURE_2D,
      0,
      format.to_gl_internal_format(),
      size.x as GLsizei,
      size.y as GLsizei,
      0,
      format.to_gl_format(),
      format.to_gl_type(),
      std::ptr::null(),
    ));

    self.set_texture_parameters(&texture);

    texture
  }

  unsafe fn create_texture_from_data(&self, size: Vec2i, data: &[u8]) -> Self::Texture {
    assert!(data.len() >= size.x as usize * size.y as usize);

    let mut texture = Self::Texture { id: 0, size, format: TextureFormat::RGB8 };
    checked!(gl::GenTextures(1, &mut texture.id));

    self.bind_texture(&texture, 0);
    checked!(gl::TexImage2D(
      gl::TEXTURE_2D,
      0,
      gl::R8 as GLint,
      size.x as GLsizei,
      size.y as GLsizei,
      0,
      gl::RED,
      gl::UNSIGNED_BYTE,
      data.as_ptr() as *const GLvoid,
    ));

    self.set_texture_parameters(&texture);
    texture
  }

  unsafe fn get_texture_size(&self, texture: &Self::Texture) -> Vec2i {
    texture.size
  }

  unsafe fn upload_to_texture(&self, texture: &Self::Texture, size: Vec2i, data: &[u8]) {
    assert!(data.len() >= size.x as usize * size.y as usize * 4);

    self.bind_texture(texture, 0);
    checked!(gl::TexImage2D(
      gl::TEXTURE_2D,
      0,
      gl::RGBA as GLint,
      size.x as GLsizei,
      size.y as GLsizei,
      0,
      gl::RGBA,
      gl::UNSIGNED_BYTE,
      data.as_ptr() as *const GLvoid,
    ));

    self.set_texture_parameters(texture);
  }

  unsafe fn clear_render_target(&self, ops: &ClearOps) {
    let mut flags = 0;

    if let Some(color) = ops.color {
      checked!(gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE));
      checked!(gl::ClearColor(
        color.r as f32 / 255.,
        color.g as f32 / 255.,
        color.b as f32 / 255.,
        color.a as f32 / 255.,
      ));

      flags |= gl::COLOR_BUFFER_BIT;
    }

    if let Some(depth) = ops.depth {
      checked!(gl::DepthMask(gl::TRUE));
      checked!(gl::ClearDepthf(depth as _));

      flags |= gl::DEPTH_BUFFER_BIT;
    }

    if let Some(stencil) = ops.stencil {
      checked!(gl::StencilMask(!0));
      checked!(gl::ClearStencil(stencil as GLint));

      flags |= gl::STENCIL_BUFFER_BIT;
    }

    if flags != 0 {
      checked!(gl::Clear(flags));
    }
  }

  unsafe fn flush_commands(&self) {
    gl::Flush();
  }

  unsafe fn draw_arrays(&self, index_count: u32, render_state: &RenderState<Self>) {
    self.set_render_state(render_state);

    checked!(gl::DrawArrays(
      render_state.primitive_type.to_gl_primitive(),
      0,
      index_count as GLsizei,
    ));

    self.reset_render_state(render_state);
  }

  unsafe fn draw_elements(&self, index_count: u32, render_state: &RenderState<Self>) {
    self.set_render_state(render_state);

    checked!(gl::DrawElements(
      render_state.primitive_type.to_gl_primitive(),
      index_count as GLsizei,
      gl::UNSIGNED_INT,
      std::ptr::null(),
    ));

    self.reset_render_state(render_state);
  }

  unsafe fn draw_elements_instanced(&self, index_count: u32, instance_count: u32, render_state: &RenderState<Self>) {
    self.set_render_state(render_state);

    checked!(gl::DrawElementsInstanced(
      render_state.primitive_type.to_gl_primitive(),
      index_count as GLsizei,
      gl::UNSIGNED_INT,
      std::ptr::null(),
      instance_count as GLsizei,
    ));

    self.reset_render_state(render_state);
  }
}

pub struct OpenGLVertexArray {
  pub id: GLuint,
}

impl Drop for OpenGLVertexArray {
  #[inline]
  fn drop(&mut self) {
    unsafe {
      checked!(gl::DeleteVertexArrays(1, &mut self.id));
    }
  }
}

pub struct OpenGLVertexAttr {
  attr: GLuint,
}

impl OpenGLVertexAttr {
  pub unsafe fn configure_float(&self, size: GLint, gl_type: GLuint, normalized: bool, stride: GLsizei, offset: usize, divisor: GLuint) {
    let normalized = if normalized { gl::TRUE } else { gl::FALSE };

    checked!(gl::VertexAttribPointer(self.attr, size, gl_type, normalized, stride, offset as *const GLvoid));
    checked!(gl::VertexAttribDivisor(self.attr, divisor));
    checked!(gl::EnableVertexAttribArray(self.attr));
  }

  pub unsafe fn configure_int(&self, size: GLint, gl_type: GLuint, stride: GLsizei, offset: usize, divisor: GLuint) {
    checked!(gl::VertexAttribIPointer(self.attr, size, gl_type, stride, offset as *const GLvoid));
    checked!(gl::VertexAttribDivisor(self.attr, divisor));
    checked!(gl::EnableVertexAttribArray(self.attr));
  }
}

pub struct OpenGLFramebuffer {
  pub id: GLuint,
  pub texture: OpenGLTexture,
}

impl Drop for OpenGLFramebuffer {
  fn drop(&mut self) {
    unsafe {
      checked!(gl::DeleteFramebuffers(1, &mut self.id));
    }
  }
}

pub struct OpenGLBuffer {
  pub id: GLuint,
}

impl Drop for OpenGLBuffer {
  fn drop(&mut self) {
    unsafe {
      checked!(gl::DeleteBuffers(1, &mut self.id));
    }
  }
}

#[derive(Debug)]
pub struct OpenGLUniform {
  location: GLint,
}

pub struct OpenGLProgram {
  pub id: GLuint,
  vertex_shader: OpenGLShader,
  fragment_shader: OpenGLShader,
}

impl Drop for OpenGLProgram {
  fn drop(&mut self) {
    unsafe {
      checked!(gl::DeleteProgram(self.id));
    }
  }
}

pub struct OpenGLShader {
  id: GLuint,
}

impl Drop for OpenGLShader {
  fn drop(&mut self) {
    unsafe {
      checked!(gl::DeleteShader(self.id));
    }
  }
}

pub struct OpenGLTexture {
  id: GLuint,
  pub size: Vec2i,
  pub format: TextureFormat,
}

impl BufferTarget {
  fn to_gl_target(&self) -> GLuint {
    match self {
      BufferTarget::Vertex => gl::ARRAY_BUFFER,
      BufferTarget::Index => gl::ELEMENT_ARRAY_BUFFER,
    }
  }
}

impl BufferUploadMode {
  fn to_gl_usage(&self) -> GLuint {
    match self {
      BufferUploadMode::Static => gl::STATIC_DRAW,
      BufferUploadMode::Dynamic => gl::DYNAMIC_DRAW,
    }
  }
}

impl DepthFunc {
  fn to_gl_depth_func(&self) -> GLenum {
    match self {
      DepthFunc::Less => gl::LESS,
      DepthFunc::Always => gl::ALWAYS,
    }
  }
}

impl PrimitiveType {
  fn to_gl_primitive(&self) -> GLuint {
    match self {
      PrimitiveType::Triangles => gl::TRIANGLES,
      PrimitiveType::Lines => gl::LINES,
    }
  }
}

impl StencilFunc {
  fn to_gl_stencil_func(&self) -> GLenum {
    match self {
      StencilFunc::Always => gl::ALWAYS,
      StencilFunc::Equal => gl::EQUAL,
    }
  }
}

impl TextureFormat {
  fn to_gl_internal_format(&self) -> GLint {
    match self {
      TextureFormat::RGB8 => gl::RGB as GLint,
      TextureFormat::RGBA8 => gl::RGBA as GLint,
    }
  }

  fn to_gl_format(&self) -> GLuint {
    match self {
      TextureFormat::RGB8 => gl::RGB,
      TextureFormat::RGBA8 => gl::RGBA,
    }
  }

  fn to_gl_type(&self) -> GLuint {
    match self {
      TextureFormat::RGB8 => gl::UNSIGNED_BYTE,
      TextureFormat::RGBA8 => gl::UNSIGNED_BYTE,
    }
  }
}

impl VertexAttrType {
  fn to_gl_type(&self) -> GLuint {
    match self {
      VertexAttrType::F32 => gl::FLOAT,
      VertexAttrType::I16 => gl::SHORT,
      VertexAttrType::I8 => gl::BYTE,
      VertexAttrType::U16 => gl::UNSIGNED_SHORT,
      VertexAttrType::U8 => gl::UNSIGNED_BYTE,
    }
  }
}

/// Panics if there any errors in the OpenGL runtime.
#[cfg(debug_assertions)]
unsafe fn glassert() {
  // TODO: make this loop through all errors, as opposed to taking the latest
  let err = gl::GetError();
  if err != gl::NO_ERROR {
    let friendly = match err {
      gl::INVALID_ENUM => "INVALID_ENUM",
      gl::INVALID_VALUE => "INVALID_VALUE",
      gl::INVALID_OPERATION => "INVALID_OPERATION",
      gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
      gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
      gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
      gl::STACK_OVERFLOW => "STACK_OVERFLOW",
      _ => "Unknown"
    };
    panic!("GL error 0x{:x} - {}", err, friendly);
  }
}

// Flips a buffer of image data upside-down.
fn flip_y<T>(pixels: &mut [T], size: Vec2i, channels: usize) {
  let width = size.x as usize;
  let height = size.y as usize;

  let stride = width * channels;

  for y in 0..(height / 2) {
    let index_a = y * stride;
    let index_b = (height - y - 1) * stride;

    for offset in 0..stride {
      pixels.swap(index_a + offset, index_b + offset);
    }
  }
}