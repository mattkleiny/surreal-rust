use super::*;

pub struct OpenGLGraphicsDevice {}

impl OpenGLGraphicsDevice {
  pub fn new() -> Self {
    Self {}
  }
}

impl GraphicsDevice for OpenGLGraphicsDevice {
  type Buffer = ();
  type Framebuffer = ();
  type Program = ();
  type Shader = ();
  type Texture = ();
  type TimerQuery = ();
  type Uniform = ();
  type VertexArray = ();
  type VertexAttr = ();

  fn get_vertex_attr(&self, program: &Self::Program, name: &str) -> Option<Self::VertexAttr> {
    unimplemented!()
  }

  fn get_uniform(&self, program: &Self::Program, name: &str) -> Self::Uniform {
    unimplemented!()
  }

  fn bind_buffer(&self, vertex_array: &Self::VertexArray, buffer: &Self::Buffer, target: BufferTarget) {
    unimplemented!()
  }

  fn configure_vertex_attr(&self, vertex_array: &Self::VertexArray, attr: &Self::VertexAttr, descriptor: &VertexAttrDescriptor) {
    unimplemented!()
  }

  fn create_framebuffer(&self, texture: Self::Texture) -> Self::Framebuffer {
    unimplemented!()
  }

  fn create_buffer(&self) -> Self::Buffer {
    unimplemented!()
  }

  fn allocate_buffer<T>(&self, buffer: &Self::Buffer, data: BufferData<T>, target: BufferTarget, mode: BufferUploadMode) {
    unimplemented!()
  }

  fn create_shader(&self, kind: ShaderKind) -> Self::Shader {
    unimplemented!()
  }

  fn create_shader_from_source(&self, source: &[u8], kind: ShaderKind) -> Self::Shader {
    unimplemented!()
  }

  fn create_vertex_array(&self) -> Self::VertexArray {
    unimplemented!()
  }

  fn create_program_from_shaders(&self, vertex_shader: Self::Shader, fragment_shader: Self::Shader) -> Self::Program {
    unimplemented!()
  }

  fn get_framebuffer_texture<'f>(&self, framebuffer: &'f Self::Framebuffer) -> &'f Self::Texture {
    unimplemented!()
  }

  fn create_texture(&self, format: TextureFormat, size: Vec2i) -> Self::Texture {
    unimplemented!()
  }

  fn create_texture_from_data(&self, size: Vec2i, data: &[u8]) -> Self::Texture {
    unimplemented!()
  }

  fn get_texture_size(&self, texture: &Self::Texture) -> Vec2i {
    unimplemented!()
  }

  fn upload_to_texture(&self, texture: &Self::Texture, size: Vec2i, data: &[u8]) {
    unimplemented!()
  }

  fn read_pixels(&self, target: &RenderTarget<Self>, viewport: RectI) -> TextureData {
    unimplemented!()
  }

  fn begin_commands(&self) {
    unimplemented!()
  }

  fn end_commands(&self) {
    unimplemented!()
  }

  fn draw_arrays(&self, index_count: u32, render_state: &RenderState<Self>) {
    unimplemented!()
  }

  fn draw_elements(&self, index_count: u32, render_state: &RenderState<Self>) {
    unimplemented!()
  }

  fn draw_elements_instanced(&self, index_count: u32, instance_count: u32, render_state: &RenderState<Self>) {
    unimplemented!()
  }
}

/// Panics if there any errors in the OpenGL runtime.
#[cfg(debug_assertions)]
fn assert_gl_errors() {
  unsafe {
    // Note that ideally we should be calling gl::GetError() in a loop until it
    // returns gl::NO_ERROR, but for now we'll just report the first one we find.
    let err = gl::GetError();
    if err != gl::NO_ERROR {
      panic!("GL error: 0x{:x} ({})", err, match err {
        gl::INVALID_ENUM => "INVALID_ENUM",
        gl::INVALID_VALUE => "INVALID_VALUE",
        gl::INVALID_OPERATION => "INVALID_OPERATION",
        gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
        gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
        gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
        gl::STACK_OVERFLOW => "STACK_OVERFLOW",
        _ => "Unknown"
      });
    }
  }
}