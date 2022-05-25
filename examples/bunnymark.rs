//! A simple sprite benchmark for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Bunnymark",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let shader = load_standard_shader(&game.host.graphics);

    let mut material = Material::new(&game.host.graphics, &shader);
    let mut batch = SpriteBatch::new(&game.host.graphics);
    let mut texture = Texture::new(&game.host.graphics);
    let image = Image::from_path("assets/sprites/bunny.png", None).expect("Failed to load sprite image");

    texture.write_image(&image);

    material.set_blend_state(BlendState::Enabled {
      source: BlendFactor::SrcAlpha,
      destination: BlendFactor::OneMinusSrcAlpha,
    });

    // TODO: simplify this
    let region = TextureRegion::from(&texture);

    game.run_variable_step(move |context| {
      context.host.graphics.clear_color_buffer(Color::BLACK);

      batch.begin(&material);
      batch.draw(&region, SpriteOptions::default());
      batch.flush();

      if let Some(keyboard) = context.host.input.primary_keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
