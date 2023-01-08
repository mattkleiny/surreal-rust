//! A simple tile-map example for Surreal

use surreal::prelude::*;
use surreal::prototype::*;

fn main() {
  let configuration = Configuration {
    title: "Tile-maps",
    transparent_window: true,
    log_level: LevelFilter::Trace,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    let graphics = &engine.graphics;

    // set-up assets and rendering
    let sprites1 = TextureAtlas::load(&assets, 16, 16, "assets/sprites/tiles_desert.png").unwrap();
    let sprites2 = TextureAtlas::load(&assets, 16, 16, "assets/sprites/spawner-idle.png").unwrap();
    let sprites3 = TextureAtlas::load(&assets, 16, 16, "assets/sprites/spawner-walk.png").unwrap();

    let mut renderer = Renderer::new(graphics);

    renderer.add_descriptor(SpriteContextDescriptor {
      projection_view: Mat4::orthographic_rh_gl(-256. / 2., 256. / 2., 144. / 2., -144. / 2., 0., 100.),
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
      let graphics = &engine.graphics;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));

      renderer.begin_frame();
      renderer.render(&map);
      renderer.end_frame();

      if let Some(keyboard) = &engine.input.keyboard {
        if keyboard.is_key_pressed(Key::Space) {
          map.fill(|_, _| u8::random() % 4);
        }

        if keyboard.is_key_pressed(Key::Escape) {
          return TickResponse::Exit;
        }
      }

      TickResponse::Continue
    });
  });
}
