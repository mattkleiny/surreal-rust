//! A simple sprite benchmark for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Bunnymark",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    game.run_variable_step(move |context| {
      context.host.graphics.clear_color_buffer(Color::WHITE);

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
