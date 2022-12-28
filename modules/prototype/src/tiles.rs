use std::collections::HashMap;
use std::hash::Hash;

use core::collections::Grid;
use core::graphics::Renderable;
use core::maths::{vec2, Cost, NeighbourList, Numeric, PathFindingGrid, Vector2, VonNeumannNeighbourhood};

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
  pub fn get(&self, x: i32, y: i32) -> Option<T> {
    self.tiles.get(x, y).and_then(|id| T::from_id(*id))
  }

  /// Sets a tile in the grid.
  pub fn set(&mut self, x: i32, y: i32, tile: T) {
    if let Some(id) = tile.to_id() {
      self.tiles.set(x, y, id);
    }
  }

  /// Gets the sprite to be used for the given tile.
  pub fn get_sprite(&self, tile: T) -> Option<&TextureRegion<'a>> {
    if let Some(id) = tile.to_id() {
      self.sprites.get(&id)
    } else {
      None
    }
  }

  /// Sets the sprite to be used for the given tile.
  pub fn set_sprite(&mut self, tile: T, sprite: impl Into<TextureRegion<'a>>) {
    if let Some(id) = tile.to_id() {
      self.sprites.insert(id, sprite.into());
    }
  }

  /// Fills the map with the given delegate.
  pub fn fill(&mut self, body: impl Fn(usize, usize) -> T) {
    for y in 0..self.height() {
      for x in 0..self.width() {
        let tile = body(x, y);

        if let Some(id) = tile.to_id() {
          self.tiles.set(x as i32, y as i32, id);
        }
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
        if let Some(id) = self.tiles.get(x as i32, y as i32) {
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

/// Represents a kind of tile that can be used in a [`TileMap`].
pub trait Tile: Clone {
  type Id: Numeric + Hash + Eq;

  fn from_id(id: Self::Id) -> Option<Self>;
  fn to_id(&self) -> Option<Self::Id>;
}

/// Implements an implicit entry type (no abstraction).
macro_rules! implement_tile {
  ($type:ty) => {
    impl Tile for $type {
      type Id = $type;

      fn from_id(id: Self::Id) -> Option<Self> {
        Some(id)
      }

      fn to_id(&self) -> Option<Self::Id> {
        Some(*self)
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

/// A [`Tile`] that can be used for path finding.
pub trait PathableTile: Tile {
  /// The cost of pathing through this tile.
  fn get_cost(&self) -> Cost {
    1. // no cost by default
  }

  /// Can we path through this tile?
  fn is_pathable(&self) -> bool;
}

/// Allow path finding over simple tile maps.
impl<T: PathableTile> PathFindingGrid for TileMap<'_, T> {
  fn get_cost(&self, _from: Vector2<i32>, to: Vector2<i32>) -> Cost {
    match self.get(to.x, to.y) {
      Some(tile) => tile.get_cost(),
      None => f32::MAX,
    }
  }

  fn get_neighbours(&self, center: Vector2<i32>, neighbours: &mut NeighbourList) {
    for neighbour in center.von_neighbours() {
      if let Some(tile) = self.get(neighbour.x, neighbour.y) {
        if tile.is_pathable() {
          neighbours.push(neighbour);
        }
      }
    }
  }
}

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

    fn from_id(id: Self::Id) -> Option<Self> {
      match id {
        0 => Some(Self::EMPTY),
        1 => Some(Self::WALL),
        2 => Some(Self::FLOOR),
        3 => Some(Self::DOOR),
        _ => None,
      }
    }

    fn to_id(&self) -> Option<Self::Id> {
      Some(self.0)
    }
  }

  #[test]
  fn tile_map_should_read_and_write() {
    let mut map = TileMap::new(16, 16);

    map.set(0, 0, ExampleTile::WALL);
    let tile = map.get(0, 0).unwrap();

    assert_eq!(tile.1, "Wall");
  }
}
