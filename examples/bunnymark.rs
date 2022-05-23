//! A simple sprite benchmark for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Bunnymark",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let shader = ShaderProgram::new(&game.host.graphics);
    let _material = Material::new(&game.host.graphics, &shader);
    let _batch = SpriteBatch::new(&game.host.graphics);

    shader.reload("assets/shaders/standard.glsl").expect("Failed to load shader program");

    game.run_variable_step(move |context| {
      context.host.graphics.clear_color_buffer(Color::BLACK);

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
