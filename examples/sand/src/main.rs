#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Falling Sand",
    ..Default::default()
  });

  let palette = ColorPalette::from_jasc_file("assets/palettes/hollow-4.pal").expect("Failed to load color palette");

  Game::start(platform, |mut game| {
    game.run_variable_step(|context| unsafe {
      context.host.graphics.clear_color_buffer(palette[0]);

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
