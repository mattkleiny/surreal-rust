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
  fn test_von_neumann_neighbours_for_vec2() {
    let point = vec2(0.0, 0.0);

    let neighbours = point.von_neighbours();

    assert_eq!(neighbours[0], vec2(-1.0, 0.0)); // left
    assert_eq!(neighbours[1], vec2(0.0, 1.0)); // top
    assert_eq!(neighbours[2], vec2(1.0, 0.0)); // right
    assert_eq!(neighbours[3], vec2(0.0, -1.0)); // bottom
  }

  #[test]
  fn test_von_neumann_neighbours_for_ivec2() {
    let point = ivec2(0, 0);

    let neighbours = point.von_neighbours();

    assert_eq!(neighbours[0], ivec2(-1, 0)); // left
    assert_eq!(neighbours[1], ivec2(0, 1)); // top
    assert_eq!(neighbours[2], ivec2(1, 0)); // right
    assert_eq!(neighbours[3], ivec2(0, -1)); // bottom
  }

  #[test]
  fn test_moore_neighbours_for_vec2() {
    let point = vec2(0.0, 0.0);

    let neighbours = point.moore_neighbours();

    assert_eq!(neighbours[0], vec2(-1.0, -1.0)); // bottom left
    assert_eq!(neighbours[1], vec2(-1.0, 0.0)); // left
    assert_eq!(neighbours[2], vec2(-1.0, 1.0)); // top left
    assert_eq!(neighbours[3], vec2(0.0, 1.0)); // top
    assert_eq!(neighbours[4], vec2(1.0, 1.0)); // top right
    assert_eq!(neighbours[5], vec2(1.0, 0.0)); // right
    assert_eq!(neighbours[6], vec2(1.0, -1.0)); // bottom right
    assert_eq!(neighbours[7], vec2(0.0, -1.0)); // bottom
  }

  #[test]
  fn test_moore_neighbours_for_ivec2() {
    let point = ivec2(0, 0);

    let neighbours = point.moore_neighbours();

    assert_eq!(neighbours[0], ivec2(-1, -1)); // bottom left
    assert_eq!(neighbours[1], ivec2(-1, 0)); // left
    assert_eq!(neighbours[2], ivec2(-1, 1)); // top left
    assert_eq!(neighbours[3], ivec2(0, 1)); // top
    assert_eq!(neighbours[4], ivec2(1, 1)); // top right
    assert_eq!(neighbours[5], ivec2(1, 0)); // right
    assert_eq!(neighbours[6], ivec2(1, -1)); // bottom right
    assert_eq!(neighbours[7], ivec2(0, -1)); // bottom
  }
}
