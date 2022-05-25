use crate::maths::Grid;

pub trait Tile: 'static {
  type Id: Copy + Default + Sized;

  fn from_id(id: Self::Id) -> &'static Self;
  fn to_id(&self) -> Self::Id;
}

struct TileMap<T> where T: Tile {
  tiles: Grid<T::Id>,
}

impl<T> TileMap<T> where T: Tile {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      tiles: Grid::new(width, height),
    }
  }

  pub fn get_tile(&self, x: usize, y: usize) -> &T {
    T::from_id(self.tiles[(x, y)])
  }

  pub fn set_tile(&mut self, x: usize, y: usize, tile: &T) {
    self.tiles[(x, y)] = tile.to_id();
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