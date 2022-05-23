//! A fun little falling sand simulation for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

// TODO: clean up patterns used in here

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Falling Sand",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let shader = ShaderProgram::new(&game.host.graphics)
      .reload("assets/shaders/standard.glsl")
      .expect("Failed to load shader program");

    let mut material = Material::new(&game.host.graphics, &shader);
    let mut canvas = PixelCanvas::new(&game.host.graphics, 256, 144);

    let palette = ColorPalette::from_file("assets/palettes/hollow-4.pal").expect("Failed to load color palette");

    material.set_uniform("u_projectionView", Matrix4x4::IDENTITY);
    material.set_texture("u_texture", canvas.texture.handle, 0, None);

    canvas.clear();

    game.run_variable_step(|context| {
      context.host.graphics.clear_color_buffer(palette[0]);

      canvas.update(context.time.delta_time);
      canvas.draw(&material);

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }

        if keyboard.is_key_pressed(Key::Space) {
          canvas.clear();
        }
      }

      if let Some(mouse) = context.host.input.primary_mouse_device() {
        if mouse.is_button_down(MouseButton::Left) {
          let colors = &palette.as_slice()[1..4];

          canvas.draw_sand(mouse.normalised_position(), 6., *colors.select_randomly());
        } else if mouse.is_button_down(MouseButton::Right) {
          canvas.draw_sand(mouse.normalised_position(), 6., Color::CLEAR);
        }
      }
    });
  });
}

/// A simple canvas of pixels that can be rendered to the screen.
struct PixelCanvas<G> where G: GraphicsImpl {
  texture: Texture<G>,
  mesh: Mesh<G, Vertex2>,
  timer: IntervalTimer,
  pub pixels: Grid<Color>,
}

impl<G> PixelCanvas<G> where G: GraphicsImpl {
  /// Creates a new pixel canvas with the given dimensions.
  pub fn new(server: &GraphicsServer<G>, width: usize, height: usize) -> Self {
    Self {
      texture: Texture::new(server),
      mesh: Mesh::create_quad(server, 1.),
      timer: IntervalTimer::new(TimeSpan::from_millis(10.)),
      pixels: Grid::new(width, height),
    }
  }

  /// Draws a circle of sand.
  pub fn draw_sand(&mut self, center: Vector2<f32>, radius: f32, color: Color) {
    // TODO: clean this up
    let Vector2 { x, y } = center;

    let x = (x * self.pixels.width() as f32).floor() as usize;
    let y = (y * self.pixels.height() as f32).floor() as usize;

    let x = clamp(x, 0, self.pixels.width() - 1) as isize;
    let y = clamp(y, 0, self.pixels.height() - 1) as isize;

    self.pixels.draw_circle(vec2(x, y), radius as isize, color);
  }

  /// Updates the sand simulation.
  pub fn update(&mut self, delta_time: f32) {
    if self.timer.tick(delta_time) {
      self.timer.reset();

      for y in (0..self.pixels.height()).rev() {
        for x in 0..self.pixels.width() {
          let pixel = self.pixels[(x, y)];
          if pixel.a <= 0. {
            continue;
          }

          match () {
            _ if self.simulate_sand((x, y), (x as isize, y as isize + 1)) => (),
            _ if self.simulate_sand((x, y), (x as isize - 1, y as isize + 1)) => (),
            _ if self.simulate_sand((x, y), (x as isize + 1, y as isize + 1)) => (),
            _ => {}
          }
        }
      }
    }
  }

  fn simulate_sand(&mut self, (from_x, from_y): (usize, usize), (to_x, to_y): (isize, isize)) -> bool {
    if to_x < 0 || to_x > (self.pixels.width() - 1) as isize { return false; }
    if to_y < 0 || to_y > (self.pixels.height() - 1) as isize { return false; }

    let to_x = to_x as usize;
    let to_y = to_y as usize;

    let target = self.pixels[(to_x, to_y)];

    if target.a <= 0. {
      self.pixels[(to_x, to_y)] = self.pixels[(from_x, from_y)];
      self.pixels[(from_x, from_y)] = Color::CLEAR;

      return true;
    }

    return false;
  }

  /// Draws the canvas to the screen.
  pub fn draw(&mut self, material: &Material<G>) {
    // blit pixel data to the GPU
    self.texture.write_pixels(
      self.pixels.width(),
      self.pixels.height(),
      &self.pixels.as_slice(),
    );

    // render to the screen
    self.mesh.draw(&material, PrimitiveTopology::Triangles);
  }

  /// Clears the canvas.
  pub fn clear(&mut self) {
    self.pixels.fill(Color::CLEAR)
  }
}
