#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Mindustry",
    icon: Some(include_bytes!("../mindustry.ico")),
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    game.run_variable_step(|context| unsafe {
      context.host.graphics.clear_color_buffer(Color::BLACK);

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
