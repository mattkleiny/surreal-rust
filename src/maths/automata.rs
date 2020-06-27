use smallvec::SmallVec;

use crate::maths::{vec2, Vector2};

pub fn von_neumann_neighbourhood(center: Vector2<i32>) -> SmallVec<[Vector2<i32>; 4]> {
  smallvec![
    vec2(center.x - 1, center.y), // left
    vec2(center.x + 1, center.y), // right
    vec2(center.x, center.y - 1), // bottom
    vec2(center.x, center.y + 1), // top
  ]
}

pub fn moore_neighbourhood(center: Vector2<i32>) -> SmallVec<[Vector2<i32>; 8]> {
  smallvec![
    vec2(center.x - 1, center.y), // left
    vec2(center.x + 1, center.y), // right
    vec2(center.x, center.y - 1), // bottom
    vec2(center.x, center.y + 1), // top

    vec2(center.x - 1, center.y - 1), // bottom left
    vec2(center.x - 1, center.y + 1), // top left
    vec2(center.x + 1, center.y - 1), // bottom right
    vec2(center.x + 1, center.y + 1) // top right
  ]
}

/// A classification of neighbourhoods for grid traversal and simulation.
#[derive(Copy, Clone, Debug)]
pub enum Neighbourhood {
  VonNeumann,
  VonNeumannInclusive,
  Moore,
  MooreInclusive,
}

impl Neighbourhood {
  pub fn get_neighbours(&self, center: Vector2<i32>) -> SmallVec<[Vector2<i32>; 9]> {
    match self {
      Neighbourhood::VonNeumann => smallvec![
        vec2(center.x - 1, center.y), // left
        vec2(center.x + 1, center.y), // right
        vec2(center.x, center.y - 1), // bottom
        vec2(center.x, center.y + 1), // top
      ],
      Neighbourhood::VonNeumannInclusive => smallvec![
        vec2(center.x - 1, center.y), // left
        vec2(center.x + 1, center.y), // right
        vec2(center.x, center.y), // center
        vec2(center.x, center.y - 1), // bottom
        vec2(center.x, center.y + 1), // top
      ],
      Neighbourhood::Moore => smallvec![
        vec2(center.x - 1, center.y), // left
        vec2(center.x + 1, center.y), // right
        vec2(center.x, center.y - 1), // bottom
        vec2(center.x, center.y + 1), // top
        vec2(center.x - 1, center.y - 1), // bottom left
        vec2(center.x - 1, center.y + 1), // top left
        vec2(center.x + 1, center.y - 1), // bottom right
        vec2(center.x + 1, center.y + 1) // top right
      ],
      Neighbourhood::MooreInclusive => smallvec![
        vec2(center.x - 1, center.y), // left
        vec2(center.x + 1, center.y), // right
        vec2(center.x, center.y - 1), // bottom
        vec2(center.x, center.y + 1), // top
        vec2(center.x, center.y), // center
        vec2(center.x - 1, center.y - 1), // bottom left
        vec2(center.x - 1, center.y + 1), // top left
        vec2(center.x + 1, center.y - 1), // bottom right
        vec2(center.x + 1, center.y + 1) // top right
      ],
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn neighbourhood_should_produce_valid_adjacent_points() {
    assert_eq!(Neighbourhood::VonNeumann.get_neighbours(vec2(0, 0)).len(), 4);
    assert_eq!(Neighbourhood::Moore.get_neighbours(vec2(0, 0)).len(), 8);
  }
}
