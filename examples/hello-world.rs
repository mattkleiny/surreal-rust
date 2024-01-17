use surreal::{
  common::{
    maths::{FromRandom, Lerp, PingPong},
    utilities::DeltaClock,
  },
  graphics::{Color, GraphicsEngine, OrthographicCamera},
  sdl2::{Window, WindowSettings},
};

fn main() -> surreal::common::Result<()> {
  let window = Window::new(&WindowSettings {
    title: "Hello World!",
    ..Default::default()
  })?;

  let graphics = GraphicsEngine::opengl(&window);
  let _camera = OrthographicCamera::default();

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
