//! A simple 'hello-world' of the wgpu backend.

use std::error::Error;

use common::{Color, DeltaClock, FromRandom, Lerp, PingPong};
use graphics::graphics;
use surreal_backend_wgpu::{Application, WindowSettings};

fn main() -> Result<(), Box<dyn Error>> {
  let mut clock = DeltaClock::new();
  let mut total_time = 0.0;

  let color1 = Color::random();
  let color2 = Color::random();

  Application::start(
    WindowSettings {
      title: "Hello, WGPU!".to_string(),
      width: 1920,
      height: 1080,
      resizable: true,
    },
    move || {
      let delta_time = clock.tick();
      total_time += delta_time;

      graphics().clear_color_buffer(Color::lerp(color1, color2, total_time.ping_pong()));
    },
  )
}
