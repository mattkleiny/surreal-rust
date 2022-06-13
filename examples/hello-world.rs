//! A simple Hello, World for Surreal.

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Hello, World!",
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, _| {
    let graphics = &engine.graphics;

    let shader = load_built_in_shader(graphics, BuiltInShader::SpriteStandard);
    let texture = Texture::create_colored(graphics, 1, 1, Color32::WHITE);
    let mut material = Material::new(graphics, &shader);

    material.set_texture("u_texture", &texture, None);

    let mut batch = GeometryBatch::new(graphics);

    let color1 = Color32::random();
    let color2 = Color32::random();

    engine.run_variable_step(|engine, tick| {
      let graphics = &engine.graphics;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

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
