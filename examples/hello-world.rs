//! A simple 'Hello, World' for Surreal.

use surreal::prelude::*;
use surreal::prototype::*;

fn main() {
  let configuration = EngineConfig {
    title: "Hello, World!".to_string(),
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let material = load_built_in_material(&engine.graphics, BuiltInShader::Wire);
    let mut batch = GeometryBatch::new(&engine.graphics);

    let color1 = Color32::random();
    let color2 = Color32::random();

    engine.run_variable_step(|engine, time| {
      let color = Color32::lerp(color1, color2, (time.total_time.sin() + 1.) / 2.);

      batch.begin(&material);
      batch.draw_circle(vec2(0., 0.), 0.75, 64, color);
      batch.flush();

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::Escape) {
          engine.quit();
        }
      }
    });
  });
}
