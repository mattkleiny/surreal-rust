//! A simple tilemap example for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

/// Represents a tile in our simple tile map.
#[derive(Default, Copy, Clone, Debug)]
enum MapTile {
  #[default]
  Empty,
  Cactus,
  Rock,
}

impl Tile for MapTile {
  type Id = u8;

  fn from_id(id: Self::Id) -> &'static Self {
    match id {
      0 => &Self::Empty,
      1 => &Self::Cactus,
      2 => &Self::Rock,
      _ => panic!(),
    }
  }

  fn to_id(&self) -> Self::Id {
    *self as Self::Id
  }
}

fn main() {
  let configuration = Configuration {
    title: "Tile Maps",
    update_continuously: false,
    ..Default::default()
  };

  Engine::start(configuration, |engine| {
    let graphics = &engine.graphics;

    // set-up assets and rendering
    let sprite: Texture = engine
      .assets
      .load_asset("assets/sprites/tiles_desert.png")
      .expect("Failed to load sprite image");

    let atlas = TextureAtlas::new(16, 16, &sprite);
    let palette = load_built_in_palette(BuiltInPalette::Demichrome4);

    let mut renderer = RenderManager::new(graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::create_orthographic(256., 144., 0., 100.),
      palette: Some(palette.clone()),
      ..Default::default()
    });

    // set-up tile map
    let mut map = TileMap::new(16, 9);

    map.set_sprite(&MapTile::Empty, atlas.get_region(0, 3));
    map.set_sprite(&MapTile::Cactus, atlas.get_region(3, 0));
    map.set_sprite(&MapTile::Rock, atlas.get_region(2, 2));

    map.fill(|_, _| {
      if bool::random() {
        if bool::random() {
          &MapTile::Rock
        } else {
          &MapTile::Cactus
        }
      } else {
        &MapTile::Empty
      }
    });

    engine.run_variable_step(|engine, tick| {
      engine.graphics.clear_color_buffer(palette[0]);

      renderer.render(&map);

      if engine.input.keyboard.is_key_pressed(Key::Space) {
        map.clear();
        map.fill(|_, _| {
          if bool::random() {
            if bool::random() {
              &MapTile::Rock
            } else {
              &MapTile::Cactus
            }
          } else {
            &MapTile::Empty
          }
        });
      }

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}
