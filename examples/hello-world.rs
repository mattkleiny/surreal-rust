//! A simple Hello, World for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Hello, World!",
    ..Default::default()
  });

  Game::start(platform, |mut game, _| {
    let graphics = &game.host.graphics;

    let mut batch = GeometryBatch::new(graphics);
    let shader = load_built_in_shader(graphics, BuiltInShader::Sprite(BuiltInSpriteShader::Standard));
    let mut material = Material::new(graphics, &shader);
    let texture = Texture::create_colored(graphics, 1, 1, Color32::WHITE);

    material.set_uniform("u_texture", &texture);

    game.run_variable_step(|game| {
      game.host.graphics.clear_color_buffer(Color::BLACK);

      batch.begin(&material);
      batch.draw_line(vec2(-0.5, -0.5), vec2(0.5, 0.5), Color32::WHITE, 4.);
      batch.flush();

      if let Some(keyboard) = game.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          game.exit();
        }
      }
    });
  });
}
