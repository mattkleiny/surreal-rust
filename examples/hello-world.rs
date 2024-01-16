use core::{
  maths::{FromRandom, Lerp, PingPong},
  utilities::DeltaClock,
};

use surreal::graphics::{Color, GraphicsEngine, Window, WindowSettings};

fn main() -> surreal::core::Result<()> {
  let window = Window::new(&WindowSettings {
    title: "Hello World!",
    ..Default::default()
  })?;

  let graphics = GraphicsEngine::create_opengl(&window);

  let mut clock = DeltaClock::default();
  let mut total_time = 0.0;

  let color1 = Color::random();
  let color2 = Color::random();

  while window.update() {
    let delta_time = clock.tick();
    total_time += delta_time;

    graphics.clear_color_buffer(Color::lerp(color1, color2, total_time.ping_pong()));
    window.present();
  }

  Ok(())
}
