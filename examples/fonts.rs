//! An example of bitmap fonts in Surreal.

use std::ops::Deref;
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
    let bitmap_font = BitmapFont::load(&assets, "assets/fonts/IBM.font").unwrap();
    let _ttf_font = TrueTypeFont::load(&assets, "assets/fonts/bitboy8_v1.otf").unwrap();
    let mut renderer = RenderManager::new(graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::orthographic(256., 144., 0., 100.),
      ..Default::default()
    });

    let color1 = Color32::random();
    let color2 = Color32::random();

    engine.run_variable_step(|engine, tick| {
      let graphics = &engine.graphics;
      let time_step = (tick.time.total_time.sin() + 1.) / 2.;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      // render some text
      renderer.with(|context: &mut SpriteBatchContext| {
        context.batch.draw_text(
          bitmap_font.deref(),
          "HELLO, SURREAL!",
          &SpriteOptions {
            position: vec2(0., 0.),
            color: Color32::lerp(color1, color2, time_step),
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
