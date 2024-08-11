use surreal::{backends::desktop::*, common::*, graphics::*};

fn main() {
  let mut window = Window::new(WindowSettings {
    title: "Hello World!".to_string(),
    ..Default::default()
  })
  .expect("Failed to create window");

  let mut clock = DeltaClock::default();
  let mut total_time = 0.0;

  let color1 = Color::random();
  let color2 = Color::random();

  while window.update() {
    let delta_time = clock.tick();
    total_time += delta_time;

    graphics().clear_color_buffer(Color::lerp(color1, color2, total_time.ping_pong()));

    window.present();
  }
}
