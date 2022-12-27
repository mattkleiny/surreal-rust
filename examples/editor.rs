//! A simple editor example for Surreal.

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Hello, Editor!",
    transparent_window: true,
    update_continuously: false,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let mut interface = UserInterface::new(&engine.graphics);

    engine.run_variable_step(|engine, tick| {
      engine.graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      interface.run(engine, |egui| {
        editor(egui);
      });

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::Escape) {
          tick.exit();
        }
      }
    });
  });
}
