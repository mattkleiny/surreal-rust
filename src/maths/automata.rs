use smallvec::SmallVec;

use crate::maths::{vec2, Vector2};

/// A classification of automata neighbourhoods for grid traversal and simulation.
#[derive(Copy, Clone, Debug)]
pub enum Neighbourhood {
  VonNeumann,
  VonNeumannInclusive,
  Moore,
  MooreInclusive,
}

impl Neighbourhood {
  pub fn get_adjacents(&self, point: Vector2<i32>) -> SmallVec<[Vector2<i32>; 9]> {
    match self {
      Neighbourhood::VonNeumann => smallvec![
        vec2(point.x - 1, point.y), // left
        vec2(point.x + 1, point.y), // right
        vec2(point.x, point.y - 1), // bottom
        vec2(point.x, point.y + 1), // top
      ],
      Neighbourhood::VonNeumannInclusive => smallvec![
        vec2(point.x - 1, point.y), // left
        vec2(point.x + 1, point.y), // right
        vec2(point.x, point.y), // center
        vec2(point.x, point.y - 1), // bottom
        vec2(point.x, point.y + 1), // top
      ],
      Neighbourhood::Moore => smallvec![
        vec2(point.x - 1, point.y), // left
        vec2(point.x + 1, point.y), // right
        vec2(point.x, point.y - 1), // bottom
        vec2(point.x, point.y + 1), // top
        vec2(point.x - 1, point.y - 1), // bottom left
        vec2(point.x - 1, point.y + 1), // top left
        vec2(point.x + 1, point.y - 1), // bottom right
        vec2(point.x + 1, point.y + 1) // top right
      ],
      Neighbourhood::MooreInclusive => smallvec![
        vec2(point.x - 1, point.y), // left
        vec2(point.x + 1, point.y), // right
        vec2(point.x, point.y - 1), // bottom
        vec2(point.x, point.y + 1), // top
        vec2(point.x, point.y), // center
        vec2(point.x - 1, point.y - 1), // bottom left
        vec2(point.x - 1, point.y + 1), // top left
        vec2(point.x + 1, point.y - 1), // bottom right
        vec2(point.x + 1, point.y + 1) // top right
      ],
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn neighbourhood_should_produce_valid_adjacent_points() {
    assert_eq!(Neighbourhood::VonNeumann.get_adjacents(vec2(0, 0)).len(), 4);
    assert_eq!(Neighbourhood::Moore.get_adjacents(vec2(0, 0)).len(), 8);
  }
}
