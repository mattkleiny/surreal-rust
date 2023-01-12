//! An example of bitmap fonts in Surreal.

use std::ops::Deref;
use surreal::prelude::*;
use surreal::prototype::*;

fn main() {
  let configuration = EngineConfig {
    title: "Fonts".to_string(),
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    let graphics = &engine.graphics;

    // set-up rendering
    let bitmap_font = BitmapFont::load(&assets, "assets/fonts/IBM.font").unwrap();
    let vector_font = VectorFont::load(&assets, "assets/fonts/bitboy8_v1.otf").unwrap();
    let mut renderer = Renderer::new(graphics);

    renderer.add_descriptor(SpriteContextDescriptor {
      projection_view: Mat4::orthographic_rh_gl(-256. / 2., 256. / 2., 144. / 2., -144. / 2., 0., 100.),
      ..Default::default()
    });

    let color1 = Color32::random();
    let color2 = Color32::random();

    engine.run_variable_step(|engine, time| {
      let time_step = (time.total_time.sin() + 1.) / 2.;

      renderer.begin_frame();

      // render some text
      renderer.with(|context: &mut SpriteContext| {
        context.batch.draw_text(
          bitmap_font.deref(),
          "HELLO, SURREAL!",
          &SpriteOptions {
            position: vec2(0., 8.),
            color: Color32::lerp(color1, color2, time_step),
            ..Default::default()
          },
        );

        context.batch.draw_text(
          vector_font.deref(),
          "HELLO, SURREAL!",
          &SpriteOptions {
            position: vec2(0., -8.),
            color: Color32::lerp(color1, color2, time_step),
            ..Default::default()
          },
        )
      });

      renderer.end_frame();

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::Escape) {
          engine.quit();
        }
      }
    });
  });
}
