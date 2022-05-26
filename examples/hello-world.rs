//! A simple Hello, World for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    ..Default::default()
  });

  Game::start(platform, |mut game, _| {
    let color1 = Color::random();
    let color2 = Color::random();

    game.run_variable_step(|context| {
      let total_time = context.time.total_time as f32;
      let color = Color::lerp(color1, color2, (total_time.sin() + 1.) / 2.);

      context.host.graphics.clear_color_buffer(color);

      if let Some(keyboard) = context.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
