use crate::collections::Array2D;
use crate::graphics::*;
use crate::maths::{Circle, Grid, vec2, Vector2};
use crate::utilities::{IntervalTimer, TimeSpan};

/// A simple canvas of pixels that can be rendered to the screen.
pub struct PixelCanvas {
  pub texture: Texture,
  pub mesh: Mesh<Vertex2>,
  pub pixels: Array2D<Color>,
  timer: IntervalTimer,
}

impl PixelCanvas {
  /// Creates a new pixel canvas with the given dimensions.
  pub fn new(server: &GraphicsServer, width: usize, height: usize) -> Self {
    Self {
      texture: Texture::new(server),
      mesh: Mesh::create_quad(server, 1.),
      pixels: Array2D::new(width, height),
      timer: IntervalTimer::new(TimeSpan::from_millis(10.)),
    }
  }

  /// Draws a circle of pixels.
  pub fn draw_circle(&mut self, Vector2 { x, y }: Vector2<f32>, radius: f32, color: Color) {
    let shape = Circle {
      center: vec2(
        x.floor() as isize,
        y.floor() as isize
      ),
      radius: radius as isize,
    };

    self.pixels.draw_shape(&shape, color);
  }

  /// Updates the pixel simulation.
  pub fn simulate(&mut self, delta_time: f32) {
    if self.timer.tick(delta_time) {
      self.timer.reset();

      for y in (0..self.pixels.height()).rev() {
        for x in 0..self.pixels.width() {
          let pixel = self.pixels.get((x, y));
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

    let target = self.pixels.get((to_x, to_y));

    if target.a <= 0. {
      self.pixels.set((to_x, to_y), self.pixels.get((from_x, from_y)).clone());
      self.pixels.set((from_x, from_y), Color::CLEAR);

      return true;
    }

    return false;
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

  /// Clears the canvas.
  pub fn clear(&mut self) {
    self.pixels.fill(Color::CLEAR)
  }
}
