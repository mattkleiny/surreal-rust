//! A simple tile-map example for Surreal

use surreal::{prelude::*, prototype::*};

fn main() {
  EngineBuilder::default()
    .with_title("Tile Maps")
    .with_log_level(LevelFilter::Trace)
    .start(|engine, assets| {
      // set-up assets and rendering
      let sprites1 = TextureAtlas::load(&assets, 16, 16, "assets/sprites/tiles_desert.png")?;
      let sprites2 = TextureAtlas::load(&assets, 16, 16, "assets/sprites/spawner-idle.png")?;
      let sprites3 = TextureAtlas::load(&assets, 16, 16, "assets/sprites/spawner-walk.png")?;

      let mut renderer = Renderer::new(&engine.graphics);

      renderer.add_descriptor(SpriteContextDescriptor {
        projection_view: Mat4::orthographic_rh_gl(
          -256. / 2.,
          256. / 2.,
          144. / 2.,
          -144. / 2.,
          0.,
          100.,
        ),
        palette: Some(load_built_in_palette(BuiltInPalette::Hollow4)),
        ..Default::default()
      });

      // set-up tile map
      let mut map = TileMap::new(16, 9);

      map.set_sprite(1, sprites2.get_region(0, 0));
      map.set_sprite(2, sprites3.get_region(0, 0));
      map.set_sprite(3, sprites1.get_region(3, 0));

      map.fill(|_, _| u8::random() % 4);

      engine.run_variable_step(|engine, _| {
        engine
          .graphics
          .clear_color_buffer(Color::rgb(0.1, 0.1, 0.1));

        renderer.begin_frame();
        renderer.render(&map);
        renderer.end_frame();

        if let Some(keyboard) = &engine.input.keyboard {
          if keyboard.is_key_pressed(Key::Space) {
            map.fill(|_, _| u8::random() % 4);
          }

          if keyboard.is_key_pressed(Key::Escape) {
            engine.quit();
          }
        }
      })
    })
    .expect("An unexpected error occurred");
}
