//! A simple sprite benchmark for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Bunnymark",
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    // add asset loaders
    game.assets.add_loader(ImageLoader::new());
    game.assets.add_loader(TextureLoader::new(&game.host.graphics));
    game.assets.add_loader(MaterialLoader::new(&game.host.graphics));
    game.assets.add_loader(ShaderProgramLoader::new(&game.host.graphics));

    let mut material: Material = game.assets.load_asset("assets/shaders/standard.glsl").expect("Failed to load shader program");
    let _batch = SpriteBatch::new(&game.host.graphics);

    material.set_uniform("u_projectionView", Matrix4x4::IDENTITY);

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
