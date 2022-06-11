use std::collections::HashMap;
use std::hash::Hash;

use crate::collections::{Grid, GridPoint};
use crate::graphics::Renderable;
use crate::maths::{vec2, Numeric};

use super::*;

/// A densely packed 2d map of [`Tile`]s.
///
/// Internally tiles are represented by their [`Tile::Id`], but the public
/// API allows for direct access via the [`T`] abstraction.
pub struct TileMap<'a, T: Tile> {
  tiles: Grid<T::Id>,
  sprites: HashMap<T::Id, TextureRegion<'a>>,
}

impl<'a, T: Tile> TileMap<'a, T> {
  /// Creates a new tile map with the given dimensions.
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      tiles: Grid::new(width, height),
      sprites: HashMap::new(),
    }
  }

  /// Returns the width of the tile map.
  pub fn width(&self) -> usize {
    self.tiles.width()
  }

  /// Returns the height of the tile map.
  pub fn height(&self) -> usize {
    self.tiles.height()
  }

  /// Gets a tile in the grid.
  pub fn get(&self, point: impl Into<GridPoint>) -> Option<T> {
    self.tiles.get(point).map(|id| T::from_id(*id))
  }

  /// Sets a tile in the grid.
  pub fn set(&mut self, point: impl Into<GridPoint>, value: T) {
    self.tiles.set(point, value.to_id());
  }

  /// Gets the sprite to be used for the given tile.
  pub fn get_sprite(&self, tile: T) -> Option<&TextureRegion<'a>> {
    self.sprites.get(&tile.to_id())
  }

  /// Sets the sprite to be used for the given tile.
  pub fn set_sprite(&mut self, tile: T, sprite: impl Into<TextureRegion<'a>>) {
    self.sprites.insert(tile.to_id(), sprite.into());
  }

  /// Fills the map with the given delegate.
  pub fn fill(&mut self, body: impl Fn(usize, usize) -> T) {
    for y in 0..self.height() {
      for x in 0..self.width() {
        let tile = body(x, y);

        self.tiles.set((x, y), tile.to_id());
      }
    }
  }

  /// Clears the map of all tiles.
  pub fn clear(&mut self) {
    self.tiles.clear();
  }
}

impl<'a, T: Tile> Renderable<SpriteBatchContext> for TileMap<'a, T> {
  /// Renders this tile map with to a sprite batch.
  fn render(&self, context: &mut SpriteBatchContext) {
    let half_width = self.tiles.width() as f32 / 2.;
    let half_height = self.tiles.height() as f32 / 2.;

    for y in 0..self.tiles.height() {
      for x in 0..self.tiles.width() {
        if let Some(id) = self.tiles.get((x, y)) {
          if let Some(region) = self.sprites.get(id) {
            let position = vec2(
              (x as f32 + 0.5) * region.size.x as f32 - half_width * region.size.x as f32,
              (y as f32 + 0.5) * region.size.y as f32 - half_height * region.size.y as f32,
            );

            context.batch.draw_sprite(
              region,
              &SpriteOptions {
                position,
                ..Default::default()
              },
            );
          }
        }
      }
    }
  }
}

/// Represents a tile that can be used in a [`TileMap`].
pub trait Tile: Clone {
  type Id: Numeric + Hash + Eq;

  fn from_id(id: Self::Id) -> Self;
  fn to_id(&self) -> Self::Id;
}

/// Implements an implicit tile type (no abstraction).
macro_rules! implement_tile {
  ($type:ty) => {
    impl Tile for $type {
      type Id = $type;

      fn from_id(id: Self::Id) -> Self {
        id
      }

      fn to_id(&self) -> Self::Id {
        *self
      }
    }
  };
}

implement_tile!(u8);
implement_tile!(u16);
implement_tile!(u32);
implement_tile!(u64);
implement_tile!(u128);
implement_tile!(usize);

implement_tile!(i8);
implement_tile!(i16);
implement_tile!(i32);
implement_tile!(i64);
implement_tile!(i128);
implement_tile!(isize);

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Clone)]
  struct ExampleTile(u8, &'static str);

  impl ExampleTile {
    pub const EMPTY: Self = Self(0, "Empty");
    pub const WALL: Self = Self(1, "Wall");
    pub const FLOOR: Self = Self(2, "Floor");
    pub const DOOR: Self = Self(3, "Door");
  }

  impl Tile for ExampleTile {
    type Id = u8;

    fn from_id(id: Self::Id) -> Self {
      match id {
        0 => Self::EMPTY,
        1 => Self::WALL,
        2 => Self::FLOOR,
        3 => Self::DOOR,
        _ => panic!("Just experimenting: {:?}", id),
      }
    }

    fn to_id(&self) -> Self::Id {
      self.0
    }
  }

  #[test]
  fn tile_map_should_read_and_write() {
    let mut map = TileMap::new(16, 16);
    let position = vec2(0, 0);

    map.set(position, ExampleTile::WALL);

    let tile = map.get(position).unwrap();

    assert_eq!(tile.1, "Wall");
  }
}
