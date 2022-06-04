//! A simple tilemap example for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  /// Represents a tile in our simple tile map.
  #[derive(Copy, Clone, Debug)]
  enum Tile {
    Empty,
    Cactus,
    Rock,
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
        1 => &Self::Cactus,
        2 => &Self::Rock,
        _ => panic!(),
      }
    }

    fn to_id(&self) -> Self::Id {
      match self {
        Tile::Empty => 0,
        Tile::Cactus => 1,
        Tile::Rock => 2,
      }
    }
  }

  let platform = DesktopPlatform::new(Configuration {
    title: "Tile Maps",
    update_continuously: false,
    ..Default::default()
  });

  Game::start(platform, |game, assets| {
    let graphics = game.host.graphics();

    // set-up assets and rendering
    let sprite: &Texture = assets
      .load_asset("assets/sprites/tiles_desert.png")
      .expect("Failed to load sprite image");

    let atlas = TextureAtlas::new(16, 16, sprite);
    let palette = load_built_in_palette(BuiltInPalette::Demichrome4);

    let mut renderer = RenderManager::new(graphics);

    renderer.configure(SpriteBatchDescriptor {
      projection_view: Matrix4x4::create_orthographic(256., 144., 0., 100.),
      palette: Some(palette.clone()),
      ..Default::default()
    });

    // set-up tile map
    let mut map = TileMap::new(16, 9);

    map.set_sprite(&Tile::Empty, atlas.get_region(0, 3));
    map.set_sprite(&Tile::Cactus, atlas.get_region(3, 0));
    map.set_sprite(&Tile::Rock, atlas.get_region(2, 2));

    map.fill(|_, _| {
      if bool::random() {
        if bool::random() {
          &Tile::Rock
        } else {
          &Tile::Cactus
        }
      } else {
        &Tile::Empty
      }
    });

    game.run_variable_step(|host, tick| {
      host.graphics().clear_color_buffer(palette[0]);

      renderer.render(&map);

      if let Some(keyboard) = host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Space) {
          map.clear();
          map.fill(|_, _| {
            if bool::random() {
              if bool::random() {
                &Tile::Rock
              } else {
                &Tile::Cactus
              }
            } else {
              &Tile::Empty
            }
          });
        }

        if keyboard.is_key_pressed(Key::Escape) {
          tick.exit();
        }
      }
    });
  });
}
