//! A simple sprite batching engine for Surreal.

use crate::graphics::{GraphicsDevice, ShaderKind};

const DEFAULT_VERTEX_SHADER: &'static [u8] = include_bytes!("../../assets/shaders/spritebatch.vert.glsl");
const DEFAULT_FRAGMENT_SHADER: &'static [u8] = include_bytes!("../../assets/shaders/spritebatch.frag.glsl");

pub struct SpriteBatch<D: GraphicsDevice> {
  shader_program: D::Program,
}

impl<D: GraphicsDevice> SpriteBatch<D> {
  pub fn new(graphics_device: &D, max_sprites: usize) -> Self {
    unsafe {
      let vertex_shader = graphics_device.create_shader_from_source(DEFAULT_VERTEX_SHADER, ShaderKind::Vertex);
      let fragment_shader = graphics_device.create_shader_from_source(DEFAULT_FRAGMENT_SHADER, ShaderKind::Fragment);
      let shader_program = graphics_device.create_program_from_shaders(vertex_shader, fragment_shader);

      Self { shader_program }
    }
  }

  pub fn begin(&mut self) {}
  pub fn end(&mut self) {}
}
