//! A simple sprite benchmark for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Bunnymark",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let shader = ShaderProgram::load(&game.host.graphics, "assets/shaders/standard.glsl").expect("Failed to load shader program");

    let mut material = Material::new(&game.host.graphics, &shader);
    let mut batch = SpriteBatch::new(&game.host.graphics);

    material.set_uniform("u_projectionView", Matrix4x4::IDENTITY);

    game.run_variable_step(|context| {
      context.host.graphics.clear_color_buffer(Color::WHITE);

      batch.begin(&mut material);

      batch.flush();

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
