//! A simple sprite example for Surreal

use surreal::prelude::*;

fn main() {
  let configuration = Configuration {
    title: "Sprites",
    transparent_window: true,
    log_level: LevelFilter::Trace,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    let graphics = &engine.graphics;

    // set-up assets and rendering
    let sprites1 = Texture::load(&assets, "assets/sprites/tiles_desert.png").unwrap();
    let sprites1 = TextureAtlas::new(16, 16, &sprites1);
    let sprites2 = Texture::load(&assets, "assets/sprites/spawner-idle.png").unwrap();
    let sprites2 = TextureAtlas::new(16, 16, &sprites2);
    let sprites3 = Texture::load(&assets, "assets/sprites/spawner-walk.png").unwrap();
    let sprites3 = TextureAtlas::new(16, 16, &sprites3);

    let mut renderer = RenderContextManager::new(graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::orthographic(256., 144., 0., 100.),
      palette: Some(load_built_in_palette(BuiltInPalette::Hollow4)),
      ..Default::default()
    });

    // set-up tile map
    let mut map = TileMap::new(16, 9);

    map.set_sprite(0, sprites1.get_region(0, 3));
    map.set_sprite(1, sprites2.get_region(0, 0));
    map.set_sprite(2, sprites3.get_region(0, 0));

    map.fill(|_, _| u8::random() % 3);

    engine.run_variable_step(|engine, tick| {
      let graphics = &engine.graphics;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));
      renderer.render(&map);

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}
