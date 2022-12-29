use super::*;

/// Provides a Von Neumann neighbourhood expansion for points in 2-space.
pub trait VonNeumannNeighbourhood {
  type Output;

  fn von_neighbours(&self) -> Self::Output;
}

impl VonNeumannNeighbourhood for Vec2 {
  type Output = [Vec2; 4];

  fn von_neighbours(&self) -> Self::Output {
    [
      vec2(self.x - 1., self.y), // left
      vec2(self.x, self.y + 1.), // top
      vec2(self.x + 1., self.y), // right
      vec2(self.x, self.y - 1.), // bottom
    ]
  }
}

impl VonNeumannNeighbourhood for IVec2 {
  type Output = [IVec2; 4];

  fn von_neighbours(&self) -> Self::Output {
    [
      ivec2(self.x - 1, self.y), // left
      ivec2(self.x, self.y + 1), // top
      ivec2(self.x + 1, self.y), // right
      ivec2(self.x, self.y - 1), // bottom
    ]
  }
}

/// Provides a Moore neighbourhood expansion for points in 2-space.
pub trait MooreNeighbourhood {
  type Output;

  fn moore_neighbours(&self) -> Self::Output;
}

impl MooreNeighbourhood for Vec2 {
  type Output = [Vec2; 8];

  fn moore_neighbours(&self) -> Self::Output {
    [
      vec2(self.x - 1., self.y - 1.), // bottom left
      vec2(self.x - 1., self.y),      // left
      vec2(self.x - 1., self.y + 1.), // top left
      vec2(self.x, self.y + 1.),      // top
      vec2(self.x + 1., self.y + 1.), // top right
      vec2(self.x + 1., self.y),      // right
      vec2(self.x + 1., self.y - 1.), // bottom right
      vec2(self.x, self.y - 1.),      // bottom
    ]
  }
}

impl MooreNeighbourhood for IVec2 {
  type Output = [IVec2; 8];

  fn moore_neighbours(&self) -> Self::Output {
    [
      ivec2(self.x - 1, self.y - 1), // bottom left
      ivec2(self.x - 1, self.y),     // left
      ivec2(self.x - 1, self.y + 1), // top left
      ivec2(self.x, self.y + 1),     // top
      ivec2(self.x + 1, self.y + 1), // top right
      ivec2(self.x + 1, self.y),     // right
      ivec2(self.x + 1, self.y - 1), // bottom right
      ivec2(self.x, self.y - 1),     // bottom
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn von_neighbours_should_produce_valid_adjacent_points() {
    assert_eq!(vec2(0., 0.).von_neighbours().len(), 4);
  }

  #[test]
  fn moore_neighbours_should_produce_valid_adjacent_points() {
    assert_eq!(vec2(0., 0.).moore_neighbours().len(), 8);
  }
}
