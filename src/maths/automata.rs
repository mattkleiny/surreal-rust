use smallvec::SmallVec;

use crate::maths::{vec2, Vector2};

/// Provides a von neumann neighbour expansion for points.
pub trait VonNeumannNeighbourhood<T> {
  type Output;

  fn get_von_neumann_neighbours(&self) -> Self::Output;
}

impl VonNeumannNeighbourhood<i32> for Vector2<i32> {
  type Output = SmallVec<[Vector2<i32>; 4]>;

  fn get_von_neumann_neighbours(&self) -> Self::Output {
    smallvec![
      vec2(self.x - 1, self.y), // left
      vec2(self.x + 1, self.y), // right
      vec2(self.x, self.y - 1), // bottom
      vec2(self.x, self.y + 1), // top
    ]
  }
}

/// Provides a moore neighbour expansion for points.
pub trait MooreNeighbourhood<T> {
  type Output;

  fn get_moore_neighbours(&self) -> Self::Output;
}

impl MooreNeighbourhood<i32> for Vector2<i32> {
  type Output = SmallVec<[Vector2<i32>; 8]>;

  fn get_moore_neighbours(&self) -> Self::Output {
    smallvec![
      vec2(self.x - 1, self.y),     // left
      vec2(self.x + 1, self.y),     // right
      vec2(self.x, self.y - 1),     // bottom
      vec2(self.x, self.y + 1),     // top
      vec2(self.x - 1, self.y - 1), // bottom left
      vec2(self.x - 1, self.y + 1), // top left
      vec2(self.x + 1, self.y - 1), // bottom right
      vec2(self.x + 1, self.y + 1)  // top right
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_von_neumann_neighbourhood_should_produce_valid_adjacent_points() {
    assert_eq!(vec2(0, 0).get_von_neumann_neighbours().len(), 4);
  }

  #[test]
  fn get_moore_neighbourhood_should_produce_valid_adjacent_points() {
    assert_eq!(vec2(0, 0).get_moore_neighbours().len(), 8);
  }
}
