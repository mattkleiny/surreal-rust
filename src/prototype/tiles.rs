use std::collections::HashMap;
use std::hash::Hash;

use crate::collections::Array2D;
use crate::graphics::Renderable;
use crate::maths::{Grid, GridPoint, Numeric, vec2};

use super::*;

/// Represents a tile that can be used in a tile map.
pub trait Tile: 'static {
  type Id: Numeric + Hash + Eq;

  fn from_id(id: Self::Id) -> &'static Self;
  fn to_id(&self) -> Self::Id;
}

pub struct TileMap<'a, T> where T: Tile {
  tiles: Array2D<T::Id>,
  sprites: HashMap<T::Id, TextureRegion<'a>>,
}

impl<'a, T> TileMap<'a, T> where T: Tile {
  /// Creates a new tile map with the given dimensions.
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      tiles: Array2D::new(width, height),
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

  /// Gets the sprite to be used for the given tile.
  pub fn get_sprite(&self, tile: &T) -> Option<&TextureRegion<'a>> {
    self.sprites.get(&tile.to_id())
  }

  /// Sets the sprite to be used for the given tile.
  pub fn set_sprite(&mut self, tile: &T, sprite: impl Into<TextureRegion<'a>>) {
    self.sprites.insert(tile.to_id(), sprite.into());
  }

  /// Fills the map with the given delegate.
  pub fn fill(&mut self, body: impl Fn(usize, usize) -> &'a T) {
    for y in 0..self.height() {
      for x in 0..self.width() {
        let tile = body(x, y);

        self.tiles.set((x, y), tile.to_id());
      }
    }
  }
}

impl<'a, T> Grid<T> for TileMap<'a, T> where T: Tile {
  fn stride(&self) -> usize {
    self.tiles.width()
  }

  fn length(&self) -> usize {
    self.tiles.length()
  }

  fn get(&self, point: impl Into<GridPoint>) -> &T {
    T::from_id(*self.tiles.get(point))
  }

  fn set(&mut self, point: impl Into<GridPoint>, value: T) {
    self.tiles.set(point, value.to_id());
  }
}

impl<'a, T> Renderable<SpriteBatchContext> for TileMap<'a, T> where T: Tile {
  /// Renders this tile map with to a sprite batch.
  fn render(&self, context: &mut SpriteBatchContext) {
    let half_width = self.tiles.width() as f32 / 2.;
    let half_height = self.tiles.height() as f32 / 2.;

    for y in 0..self.tiles.height() {
      for x in 0..self.tiles.width() {
        let id = self.tiles.get((x, y));

        if let Some(region) = self.sprites.get(&id) {
          // TODO: sprite pivots
          // TODO: transforms for tile maps and sprites
          // TODO: abstract over sprite instead of texture region?

          let position = vec2(
            (x as f32 + 0.5) * region.size.x as f32 - half_width * region.size.x as f32,
            (y as f32 + 0.5) * region.size.y as f32 - half_height * region.size.y as f32,
          );

          context.batch.draw(region, SpriteOptions {
            position,
            ..Default::default()
          });
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct ExampleTile(u8, &'static str);

  impl ExampleTile {
    pub const EMPTY: Self = Self(0, "Empty");
    pub const WALL: Self = Self(1, "Wall");
    pub const FLOOR: Self = Self(2, "Floor");
    pub const DOOR: Self = Self(3, "Door");
  }

  impl Tile for ExampleTile {
    type Id = u8;

    fn from_id(id: Self::Id) -> &'static Self {
      match id {
        0 => &Self::EMPTY,
        1 => &Self::WALL,
        2 => &Self::FLOOR,
        3 => &Self::DOOR,
        _ => panic!("Just experimenting: {:?}", id)
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

    let tile = map.get(position);

    assert_eq!(tile.1, "Wall");
  }
}