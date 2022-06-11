//! A simple tilemap example for Surreal.

use surreal::prelude::*;

/// Represents a tile in our simple tile map.
#[derive(Copy, Clone, Debug)]
enum Tile {
  Empty,
  Cactus,
  Rock,
}

impl Default for Tile {
  fn default() -> Self {
    Tile::Empty
  }
}

impl TileMapTile for Tile {
  type Id = u8;

  fn from_id(id: Self::Id) -> Option<Self> {
    match id {
      0 => Some(Self::Empty),
      1 => Some(Self::Cactus),
      2 => Some(Self::Rock),
      _ => None,
    }
  }

  fn to_id(&self) -> Option<Self::Id> {
    Some(*self as Self::Id)
  }
}

impl FromRandom for Tile {
  fn from_random(random: &mut Random) -> Self {
    Self::from_id(random.next::<u8>() % 3).unwrap()
  }
}

fn main() {
  let configuration = Configuration {
    title: "Tile Maps",
    update_continuously: false,
    transparent_window: true,
    log_level: LevelFilter::Trace,
    ..Default::default()
  };

  Engine::start(configuration, |engine, assets| {
    let graphics = &engine.graphics;

    // set-up assets and rendering
    let texture = Texture::load(&assets, "assets/sprites/tiles_desert.png").unwrap();
    let sprites = TextureAtlas::new(16, 16, &texture);
    let palette = load_built_in_palette(BuiltInPalette::Demichrome4);

    let mut renderer = RenderContextManager::new(graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::orthographic(256., 144., 0., 100.),
      palette: Some(palette.clone()),
      ..Default::default()
    });

    // set-up tile map
    let mut map = TileMap::new(16, 9);

    map.set_sprite(Tile::Empty, sprites.get_region(0, 3));
    map.set_sprite(Tile::Cactus, sprites.get_region(3, 0));
    map.set_sprite(Tile::Rock, sprites.get_region(2, 2));

    map.fill(|_, _| Tile::random());

    engine.run_variable_step(|engine, tick| {
      let graphics = &engine.graphics;

      graphics.clear_color_buffer(Color::rgba(0.2, 0.2, 0.2, 0.8));
      renderer.render(&map);

      if engine.input.keyboard.is_key_pressed(Key::Space) {
        map.clear();
        map.fill(|_, _| Tile::random());
      }

      if engine.input.keyboard.is_key_pressed(Key::Escape) {
        tick.exit();
      }
    });
  });
}
