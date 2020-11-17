/// Represents a single tile in the engine.
pub trait Tile {
  /// The underlying ID type for this tile; usually some sort of int.
  type Id: Copy;

  fn from_id(id: Self::Id) -> Self;
  fn to_id(&self) -> Self::Id;
}

/// A simple 2d map of tiles.
struct TileMap<T> where T: Tile {
  width: usize,
  height: usize,
  tiles: Vec<T::Id>,
}

impl<T> TileMap<T> where T: Tile {
  pub fn new(width: usize, height: usize, default: T) -> Self {
    Self { width, height, tiles: vec![default.to_id(); width * height] }
  }

  pub fn with_default(width: usize, height: usize) -> Self where T: Default {
    Self::new(width, height, T::default())
  }

  pub fn width(&self) -> usize { self.width }
  pub fn height(&self) -> usize { self.height }

  #[inline]
  pub fn get(&self, x: usize, y: usize) -> T {
    T::from_id(self.get_raw(x, y))
  }

  #[inline]
  pub fn get_raw(&self, x: usize, y: usize) -> T::Id {
    self.tiles[x + y * self.width]
  }

  #[inline]
  pub fn set(&mut self, x: usize, y: usize, tile: T) {
    self.set_raw(x, y, tile.to_id());
  }

  #[inline]
  pub fn set_raw(&mut self, x: usize, y: usize, id: T::Id) {
    self.tiles[x + y * self.width] = id;
  }

  #[inline(always)]
  pub fn as_slice(&self) -> &[T::Id] {
    &self.tiles
  }

  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [T::Id] {
    &mut self.tiles
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[repr(u8)]
  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  enum Tiles {
    Void = 0,
    Grass = 1,
    Water = 2,
  }

  impl Default for Tiles {
    fn default() -> Self {
      Self::Void
    }
  }

  impl Tile for Tiles {
    type Id = u8;

    fn from_id(id: Self::Id) -> Self {
      unsafe { std::mem::transmute(id) }
    }

    fn to_id(&self) -> Self::Id {
      *self as Self::Id
    }
  }

  #[test]
  fn tilemap_should_read_and_write() {
    let mut map = TileMap::with_default(16, 16);

    map.set(0, 0, Tiles::Grass);
    map.set(0, 1, Tiles::Water);
  }
}