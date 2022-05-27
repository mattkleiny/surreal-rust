//! A scripting example for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Scripting Test",
    ..Default::default()
  });

  Game::start(platform, |mut game, assets| {
    let scripting = LuaScriptBackend::new();

    assets.add_loader(ScriptLoader::new(&scripting));

    let script: &Script = assets.load_asset("assets/scripts/test.lua").expect("Failed to load script");

    game.run_variable_step(|context| {
      context.host.graphics.clear_color_buffer(Color::BLACK);

      script.execute();

      if let Some(keyboard) = context.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }
      }
    });
  });
}
