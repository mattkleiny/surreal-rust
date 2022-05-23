#![windows_subsystem = "windows"]

use surreal::prelude::*;

/// A simple canvas of pixels that can be rendered to the screen.
struct PixelCanvas {
  texture: Texture,
  mesh: Mesh<Vertex2>,
  pub pixels: Grid<Color>,
}

impl PixelCanvas {
  /// Creates a new pixel canvas with the given dimensions.
  pub fn new(context: &GraphicsContext, width: usize, height: usize) -> Self {
    Self {
      texture: Texture::new(context),
      mesh: Mesh::create_quad(context, 1.),
      pixels: Grid::new(width, height),
    }
  }

  /// Draws the canvas to the screen.
  pub fn draw(&mut self, material: &Material) {
    // blit pixel data to the GPU
    self.texture.write_pixels(
      self.pixels.width(),
      self.pixels.height(),
      &self.pixels.as_slice(),
    );

    // render to the screen
    self.mesh.draw(&material, PrimitiveTopology::Triangles);
  }
}

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Falling Sand",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let palette = ColorPalette::from_jasc_file("assets/palettes/hollow-4.pal")
      .expect("Failed to load color palette");
    let shader = ShaderProgram::load(&game.host.graphics, "assets/shaders/standard.glsl")
      .expect("Failed to load shader program");
    let mut material = Material::new(&game.host.graphics, &shader);
    let mut canvas = PixelCanvas::new(&game.host.graphics, 256, 144);

    material.set_uniform("u_projectionView", Matrix4x4::IDENTITY);
    material.set_texture("u_texture", canvas.texture.handle(), 0, None);

    canvas.pixels.fill(Color::WHITE);

    game.run_variable_step(|context| {
      context.host.graphics.clear_color_buffer(palette[0]);

      canvas.draw(&material);

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }

        if keyboard.is_key_pressed(Key::Space) {
          canvas.pixels.fill(Color::WHITE);
        }
      }

      if let Some(mouse) = context.host.input.primary_mouse_device() {
        if mouse.is_button_down(MouseButton::Left) {
          let Vector2 { x, y } = mouse.normalised_position();

          let x = (x * canvas.pixels.width() as f32).floor() as usize;
          let y = (y * canvas.pixels.height() as f32).floor() as usize;

          let x = clamp(x, 0, canvas.pixels.width() - 1);
          let y = clamp(y, 0, canvas.pixels.height() - 1);

          canvas.pixels[(x, y)] = Color::random();
        }
      }
    });
  });
}
