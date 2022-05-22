use crate::maths::{Numeric, vec2, Vector2};

/// Provides a von neumann neighbour expansion for points in 2-space.
pub trait VonNeumannNeighbourhood<T> {
  type Output;

  fn get_von_neumann_neighbours(&self) -> Self::Output;
}

impl<T> VonNeumannNeighbourhood<T> for Vector2<T> where T: Numeric {
  type Output = [Vector2<T>; 4];

  fn get_von_neumann_neighbours(&self) -> Self::Output {
    [
      vec2(self.x - T::ONE, self.y), // left
      vec2(self.x, self.y + T::ONE), // top
      vec2(self.x + T::ONE, self.y), // right
      vec2(self.x, self.y - T::ONE), // bottom
    ]
  }
}

/// Provides a moore neighbour expansion for points in 2-space.
pub trait MooreNeighbourhood<T> {
  type Output;

  fn get_moore_neighbours(&self) -> Self::Output;
}

impl<T> MooreNeighbourhood<T> for Vector2<T> where T: Numeric {
  type Output = [Vector2<T>; 8];

  fn get_moore_neighbours(&self) -> Self::Output {
    [
      vec2(self.x - T::ONE, self.y - T::ONE), // bottom left
      vec2(self.x - T::ONE, self.y), // left
      vec2(self.x - T::ONE, self.y + T::ONE), // top left
      vec2(self.x, self.y + T::ONE), // top
      vec2(self.x + T::ONE, self.y + T::ONE), // top right
      vec2(self.x + T::ONE, self.y), // right
      vec2(self.x + T::ONE, self.y - T::ONE), // bottom right
      vec2(self.x, self.y - T::ONE), // bottom
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
