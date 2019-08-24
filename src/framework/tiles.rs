//! A lightweight tile system.

use crate::maths::DenseGrid;

/// Encapsulates a tile in a tile map.
#[derive(Copy, Clone, Debug)]
pub enum Tile<T: Copy> {
  Occupied(T),
  Empty,
}

/// A tile map is a densely packed grid of tiles.
pub type TileMap<T> = DenseGrid<Tile<T>>;