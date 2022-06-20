//! A simple Hello, World for Surreal.

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Hello, World!",
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let material = load_built_in_material(&engine.graphics, BuiltInShader::Wire);
    let mut batch = GeometryBatch::new(&engine.graphics);

    let color1 = Color32::random();
    let color2 = Color32::random();

    engine.run_variable_step(|engine, tick| {
      engine.graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      let color = Color32::lerp(color1, color2, (tick.time.total_time.sin() + 1.) / 2.);

      batch.begin(&material);
      batch.draw_circle(vec2(0., 0.), 0.75, 64, color);
      batch.flush();

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}
