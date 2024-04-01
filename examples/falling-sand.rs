use common::{DeltaClock, FromRandom, Lerp, PingPong};
use graphics::{Color32, GraphicsEngine, Texture};
use surreal::backends::sdl::*;

fn main() {
  let mut window = Window::new(WindowSettings {
    title: "Falling Sand",
    ..Default::default()
  })
  .expect("Failed to create window");

  let graphics = GraphicsEngine::opengl(&window);

  let mut bitmap = Bitmap::new(256, 144);
  let mut texture = Texture::new(&graphics).unwrap();

  let color1 = Color32::random();
  let color2 = Color32::random();

  let mut total_time = 0.;
  let mut delta_clock = DeltaClock::default();

  while window.update() {
    total_time += delta_clock.tick();

    bitmap.fill(Color32::BLACK);

    for y in 0..144 {
      for x in 0..256 {
        if x % 2 == 0 && y % 2 == 0 {
          bitmap.pixels[y * 256 + x] = Color32::WHITE;
        } else {
          bitmap.pixels[y * 256 + x] = Color32::lerp(color1, color2, (total_time * 0.5).ping_pong());
        }
      }
    }

    bitmap.blit_to_texture(&mut texture);
    texture.blit_to_display();

    window.present();
  }
}

/// A simple bitmap of pixels.
#[derive(Clone)]
struct Bitmap {
  width: u32,
  height: u32,
  pixels: Vec<Color32>,
}

impl Bitmap {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height,
      pixels: vec![Color32::BLACK; (width * height) as usize],
    }
  }

  pub fn blit_to_texture(&self, texture: &mut Texture) {
    texture.write_pixels(self.width, self.height, &self.pixels);
  }

  pub fn fill(&mut self, color: Color32) {
    self.pixels.fill(color);
  }
}
