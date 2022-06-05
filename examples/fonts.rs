//! An example of bitmap fonts in Surreal.

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Bitmap Fonts",
    transparent_window: true,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    let graphics = &engine.graphics;

    // set-up rendering
    let font: Handle<BitmapFont> = assets.load_asset("assets/fonts/IBM.font").unwrap();
    let mut renderer = RenderManager::new(graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::orthographic(256., 144., 0., 100.),
      ..Default::default()
    });

    engine.run_variable_step(|engine, tick| {
      let graphics = &engine.graphics;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      // render some text
      renderer.with(|context: &mut SpriteBatchContext| {
        context.batch.draw(
          &font.get_glyph('A').unwrap(),
          &SpriteOptions {
            position: vec2(0., 0.),
            ..Default::default()
          },
        );
      });

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}
