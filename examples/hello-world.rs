use graphics::{Material, SHADER_SPRITE_MULTITEX};
use surreal::{
  common::{
    maths::{FromRandom, Lerp, PingPong},
    utilities::DeltaClock,
  },
  graphics::{Color, GraphicsEngine},
  sdl2::{Window, WindowSettings},
};

fn main() -> surreal::common::Result<()> {
  let window = Window::new(&WindowSettings {
    title: "Hello World!",
    ..Default::default()
  })?;

  let graphics = GraphicsEngine::opengl(&window);

  let mut clock = DeltaClock::default();
  let mut total_time = 0.0;

  let color1 = Color::random();
  let color2 = Color::random();

  let shader = SHADER_SPRITE_MULTITEX.to_program(&graphics)?;
  let _material = Material::new(&graphics, &shader);

  while window.update() {
    let delta_time = clock.tick();
    total_time += delta_time;

    graphics.clear_color_buffer(Color::lerp(color1, color2, total_time.ping_pong()));
    window.present();
  }

  Ok(())
}
