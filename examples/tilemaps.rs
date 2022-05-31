//! A simple tilemap example for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  /// Represents a tile in our simple tile map.
  #[derive(Copy, Clone, Debug)]
  enum Tile {
    Empty,
    Filled,
  }

  impl Default for Tile {
    fn default() -> Self {
      Self::Empty
    }
  }

  impl surreal::prelude::Tile for Tile {
    type Id = u8;

    fn from_id(id: Self::Id) -> &'static Self {
      match id {
        0 => &Self::Empty,
        1 => &Self::Filled,
        _ => panic!(),
      }
    }

    fn to_id(&self) -> Self::Id {
      match self {
        Tile::Empty => 0,
        Tile::Filled => 1,
      }
    }
  }

  let platform = DesktopPlatform::new(Configuration {
    title: "Tile Maps",
    update_continuously: false,
    ..Default::default()
  });

  Game::start(platform, |mut game, assets| {
    let graphics = &game.host.graphics;

    // set-up assets and rendering
    let sprite: &Texture = assets.load_asset("assets/sprites/example_tile.png").expect("Failed to load sprite image");
    let palette = load_built_in_palette(BuiltInPalette::Demichrome4);

    let mut renderer = RenderManager::new(graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::create_orthographic(256., 144., 0., 100.),
      palette: Some(palette.clone()),
      ..Default::default()
    });

    // set-up tile map
    let mut map = TileMap::new(16, 9);

    map.set_sprite(&Tile::Filled, sprite);
    map.fill(|_, _| if bool::random() { &Tile::Filled } else { &Tile::Empty });

    game.run_variable_step(|game| {
      game.host.graphics.clear_color_buffer(palette[0]);

      renderer.render(&map);

      if let Some(keyboard) = game.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Space) {
          map.clear();
          map.fill(|_, _| if bool::random() { &Tile::Filled } else { &Tile::Empty });
        }

        if keyboard.is_key_pressed(Key::Escape) {
          game.exit();
        }
      }
    });
  });
}
