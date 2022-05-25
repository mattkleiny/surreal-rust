use std::collections::HashMap;
use std::hash::Hash;

use crate::graphics::Renderable;
use crate::maths::{Grid, Numeric, vec2};

use super::*;

/// Represents a tile that can be used in a [`TileMap`].
pub trait Tile: 'static {
  type Id: Numeric + Hash + Eq;

  fn from_id(id: Self::Id) -> &'static Self;
  fn to_id(&self) -> Self::Id;
}

/// A 2d map of [`Tile`]s.
pub struct TileMap<T> where T: Tile {
  tiles: Grid<T::Id>,
  sprites: HashMap<T::Id, TextureRegion>,
}

impl<T> TileMap<T> where T: Tile {
  /// Creates a new [`TileMap`] with the given dimensions.
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      tiles: Grid::new(width, height),
      sprites: HashMap::new(),
    }
  }

  /// Returns the width of the [`TileMap`].
  pub fn width(&self) -> usize {
    self.tiles.width()
  }

  /// Returns the height of the [`TileMap`].
  pub fn height(&self) -> usize {
    self.tiles.height()
  }

  /// Returns the [`Tile`] at the given coordinates.
  pub fn get_tile(&self, x: usize, y: usize) -> &T {
    T::from_id(self.tiles[(x, y)])
  }

  /// Sets the [`Tile`] at the given coordinates.
  pub fn set_tile(&mut self, x: usize, y: usize, tile: &T) {
    self.tiles[(x, y)] = tile.to_id();
  }

  /// Sets the [`TextureRegion`] to be used for the given [`Tile`].
  pub fn set_sprite(&mut self, tile: &T, sprite: &TextureRegion) {
    self.sprites.insert(tile.to_id(), sprite.clone());
  }

  /// Clears the [`TileMap`] of all tiles.
  pub fn clear(&mut self) {
    self.tiles.fill(T::Id::default());
  }
}

impl<T> Renderable<SpriteContext> for TileMap<T> where T: Tile {
  /// Renders this [`TileMap`] with to a sprite batch.
  fn render(&self, context: &mut SpriteContext) {
    let half_width = self.tiles.width() as f32 / 2.;
    let half_height = self.tiles.height() as f32 / 2.;

    for y in 0..self.tiles.height() {
      for x in 0..self.tiles.width() {
        let id = self.tiles[(x, y)];

        if let Some(sprite) = self.sprites.get(&id) {
          // TODO: sprite pivots
          // TODO: transforms for tilemaps and sprites
          let position = vec2(
            (x as f32 + 0.5) * sprite.size.x as f32 - half_width * sprite.size.x as f32,
            (y as f32 + 0.5) * sprite.size.y as f32 - half_height * sprite.size.y as f32,
          );

          context.batch.draw(sprite, SpriteOptions {
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

    map.set_tile(0, 0, &ExampleTile::WALL);

    let tile = map.get_tile(0, 0);

    assert_eq!(tile.1, "Wall");
  }
}