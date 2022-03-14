use num_traits::Num;

use crate::maths::{vec2, Vector2};

/// Provides a von neumann neighbour expansion for points in 2-space.
pub trait VonNeumannNeighbourhood<T> {
  type Output;

  fn get_von_neumann_neighbours(&self) -> Self::Output;
}

impl<T> VonNeumannNeighbourhood<T> for Vector2<T> where T: Copy + Num {
  type Output = [Vector2<T>; 4];

  fn get_von_neumann_neighbours(&self) -> Self::Output {
    let one = T::one();
    [
      Vector2::new(self.x - one, self.y), // left
      Vector2::new(self.x + one, self.y), // right
      Vector2::new(self.x, self.y - one), // bottom
      Vector2::new(self.x, self.y + one), // top
    ]
  }
}

/// Provides a moore neighbour expansion for points in 2-space.
pub trait MooreNeighbourhood<T> {
  type Output;

  fn get_moore_neighbours(&self) -> Self::Output;
}

impl<T> MooreNeighbourhood<T> for Vector2<T> where T: Copy + Num {
  type Output = [Vector2<T>; 8];

  fn get_moore_neighbours(&self) -> Self::Output {
    let one = T::one();
    [
      Vector2::new(self.x - one, self.y), // left
      Vector2::new(self.x + one, self.y), // right
      Vector2::new(self.x, self.y - one), // bottom
      Vector2::new(self.x, self.y + one), // top

      Vector2::new(self.x - one, self.y - one), // bottom left
      Vector2::new(self.x - one, self.y + one), // top left
      Vector2::new(self.x + one, self.y - one), // bottom right
      Vector2::new(self.x + one, self.y + one), // top right
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
