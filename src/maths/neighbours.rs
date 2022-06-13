use super::*;

/// Provides a von neumann neighbour expansion for points in 2-space.
pub trait VonNeumannNeighbourhood<T> {
  type Output;

  fn von_neighbours(&self) -> Self::Output;
}

impl<T: Numeric> VonNeumannNeighbourhood<T> for Vector2<T> {
  type Output = [Vector2<T>; 4];

  fn von_neighbours(&self) -> Self::Output {
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

  fn moore_neighbours(&self) -> Self::Output;
}

impl<T: Numeric> MooreNeighbourhood<T> for Vector2<T> {
  type Output = [Vector2<T>; 8];

  fn moore_neighbours(&self) -> Self::Output {
    [
      vec2(self.x - T::ONE, self.y - T::ONE), // bottom left
      vec2(self.x - T::ONE, self.y),          // left
      vec2(self.x - T::ONE, self.y + T::ONE), // top left
      vec2(self.x, self.y + T::ONE),          // top
      vec2(self.x + T::ONE, self.y + T::ONE), // top right
      vec2(self.x + T::ONE, self.y),          // right
      vec2(self.x + T::ONE, self.y - T::ONE), // bottom right
      vec2(self.x, self.y - T::ONE),          // bottom
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn von_neighbours_should_produce_valid_adjacent_points() {
    assert_eq!(vec2(0, 0).von_neighbours().len(), 4);
  }

  #[test]
  fn moore_neighbours_should_produce_valid_adjacent_points() {
    assert_eq!(vec2(0, 0).moore_neighbours().len(), 8);
  }
}
