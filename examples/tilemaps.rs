//! A simple tilemap example for Surreal.

#![windows_subsystem = "windows"]

use surreal::prelude::*;

fn main() {
  let platform = DesktopPlatform::new(Configuration {
    title: "Tile Maps",
    update_continuously: false,
    ..Default::default()
  });

  Game::start(platform, |mut game| {
    let assets = &game.assets;
    let graphics = &game.host.graphics;

    // set-up assets and rendering
    let sprite: Texture = assets.load_asset("assets/sprites/example_tile.png").expect("Failed to load sprite image");
    let palette = load_standard_palette(BuiltInPalette::Demichrome4);

    let mut renderer = RenderManager::new(graphics);

    renderer.configure(SpriteContextDescriptor {
      projection_view: {
        // TODO: fix up translation matrix multiplication?
        let view = Matrix4x4::IDENTITY;
        let projection = Matrix4x4::create_orthographic(256., 144., 0., 100.);

        view * projection
      },
      palette: Some(palette.clone()),
    });

    // set-up tile map
    let mut tilemap = TileMap::new(16, 9);

    tilemap.set_sprite(&Tile::Filled, &sprite);

    for y in 0..tilemap.height() {
      for x in 0..tilemap.width() {
        if bool::random() {
          tilemap.set_tile(x, y, &Tile::Filled);
        }
      }
    }

    game.run_variable_step(|context| {
      context.host.graphics.clear_color_buffer(palette[0]);

      renderer.render(&tilemap);

      if let Some(keyboard) = context.host.input.keyboard_device() {
        if keyboard.is_key_pressed(Key::Escape) {
          context.exit();
        }

        if keyboard.is_key_pressed(Key::Space) {
          tilemap.clear();

          for y in 0..tilemap.height() {
            for x in 0..tilemap.width() {
              if bool::random() {
                tilemap.set_tile(x, y, &Tile::Filled);
              }
            }
          }
        }
      }
    });
  });
}

/// Represents a tile in our simple tile map.
enum Tile {
  Empty,
  Filled,
}

impl surreal::prelude::Tile for Tile {
  type Id = u8;

  fn from_id(id: Self::Id) -> &'static Self {
    match id {
      0 => &Self::Empty,
      1 => &Self::Filled,
      _ => panic!()
    }
  }

  fn to_id(&self) -> Self::Id {
    match self {
      Tile::Empty => 0,
      Tile::Filled => 1,
    }
  }
}
