#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let color1 = Color::random();
    let color2 = Color::random();

    game.run_variable_step(|frame| unsafe {
      let total_time = frame.time.total_time as f32;
      let color = Color::lerp(color1, color2, (total_time.sin() + 1.) / 2.);

      frame.host.graphics.clear_color_buffer(color);

      if let Some(keyboard) = frame.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          frame.exit();
        }
      }
    });
  });
}
