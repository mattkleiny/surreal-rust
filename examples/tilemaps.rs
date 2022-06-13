//! A simple tilemap example for Surreal.

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Tile Maps",
    transparent_window: true,
    log_level: LevelFilter::Trace,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    let graphics = &engine.graphics;

    // set-up assets and rendering
    let sprites = TextureAtlas::load(&assets, 16, 16, "assets/sprites/tiles_desert.png").unwrap();
    let palette = load_built_in_palette(BuiltInPalette::Demichrome4);

    let mut renderer = RenderContextManager::new(graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::orthographic(256., 144., 0., 100.),
      palette: Some(palette.clone()),
      ..Default::default()
    });

    // set-up tile map
    let mut map = TileMap::new(16, 9);

    map.set_sprite(0, sprites.get_region(0, 3));
    map.set_sprite(1, sprites.get_region(3, 0));
    map.set_sprite(2, sprites.get_region(2, 2));

    map.fill(|_, _| u8::random() % 3);

    engine.run_variable_step(|engine, tick| {
      let graphics = &engine.graphics;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));
      renderer.render(&map);

      if engine.input.keyboard.is_key_pressed(Key::Space) {
        map.clear();
        map.fill(|_, _| u8::random() % 3);
      }

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}
