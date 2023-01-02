//! A spinning cube, very exciting.

use surreal::prelude::{Color, Key};

mod helpers;

fn main() {
  helpers::run_example("Spinning Cube", |engine, assets| {
    engine.run_variable_step(|engine, tick| {
      engine.graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::Escape) {
          tick.exit();
        }
      }
    })
  });
}
