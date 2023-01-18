use surreal::{collections::Grid, graphics::*};

use super::*;

/// A very simple canvas of pixels that can be rendered to the screen.
pub struct PixelCanvas<P> {
  pub pixels: Grid<P>,
  texture: Texture,
  material: Material,
}

impl<P: Pixel + Texel> PixelCanvas<P> {
  /// Creates a new pixel canvas with the size.
  pub fn new(graphics: &GraphicsServer, width: usize, height: usize) -> Self {
    let pixels = Grid::new(width, height);
    let texture = Texture::new(graphics);

    let mut material = load_built_in_material(graphics, BuiltInShader::SpriteStandard);

    material.set_uniform(UNIFORM_PROJECTION_VIEW, &Mat4::IDENTITY);
    material.set_texture(UNIFORM_MAIN_TEXTURE, &texture, None);

    material.set_blend_state(BlendState::Enabled {
      source: BlendFactor::SrcAlpha,
      destination: BlendFactor::OneMinusSrcAlpha,
    });

    Self { pixels, texture, material }
  }

  /// Draws the canvas to the screen.
  pub fn draw(&mut self) {
    let (width, height) = (self.pixels.width(), self.pixels.height());

    self.texture.write_pixels(width, height, self.pixels.as_slice());
    self.material.draw_fullscreen_quad();
  }
}
