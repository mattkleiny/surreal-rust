use std::collections::VecDeque;
use std::hash::Hash;

use super::*;
use crate::collections::{FastHashMap, PriorityQueue};

/// Arbitrary upper limit on the number of steps to use in the find_path
/// function.
const MAXIMUM_STEPS: usize = 128;

/// A cost for path searches and relative queries.
pub type Cost = f32;

/// A heuristic function for path-finding.
pub type Heuristic<T> = fn(&T, &T) -> Cost;

/// Represents a small stack-allocated set of points used in path-finding steps.
pub type NeighbourList<T> = smallvec::SmallVec<[T; 9]>;

/// Permits exploratory path-finding over some connected grid.
pub trait PathFindingGrid<T: Copy + Hash + Eq = IVec2> {
  /// Gets the pathing cost between the given two points.
  fn get_cost(&self, _from: T, _to: T) -> Cost {
    1. // no cost function by default
  }

  /// Gets the potential neighbours around the given point.
  fn get_neighbours(&self, center: T, results: &mut NeighbourList<T>);

  /// Locates a path using A* from from the given start point to the given goal.
  fn find_path(&self, start: T, goal: T, heuristic: Heuristic<T>) -> Option<VecDeque<T>> {
    let mut frontier = PriorityQueue::new();
    let mut came_from = FastHashMap::default();
    let mut cost_so_far = FastHashMap::default();

    came_from.insert(start, start);
    cost_so_far.insert(start, 0.);

    frontier.push(start, 0.);

    let mut neighbours = NeighbourList::new();

    while let Some(current) = frontier.pop() {
      // dont search too far afield
      if cost_so_far.len() >= MAXIMUM_STEPS {
        return None;
      }

      // have we reached our target?
      if current == goal {
        // retrace path
        let mut path = VecDeque::new();
        let mut current = goal;

        while current != start {
          path.push_front(current);
          current = came_from[&current];
        }

        path.push_front(start);

        return Some(path);
      }

      // depth-first search along neighbours, use heuristic
      neighbours.clear();
      self.get_neighbours(current, &mut neighbours);

      for neighbour in &neighbours {
        let new_cost = cost_so_far[&current] + self.get_cost(current, *neighbour);

        if !cost_so_far.contains_key(neighbour) || new_cost < cost_so_far[neighbour] {
          // back track along neighbours?
          if cost_so_far.contains_key(neighbour) {
            cost_so_far.remove(neighbour);
            came_from.remove(neighbour);
          }

          cost_so_far.insert(*neighbour, new_cost);
          came_from.insert(*neighbour, current);

          let priority = new_cost + heuristic(neighbour, &goal);

          frontier.push(*neighbour, priority);
        }
      }
    }

    None
  }
}

pub mod heuristics {
  //! Heuristic functions for path-finding.

  use super::*;

  /// A constant distance
  pub fn constant(_from: &IVec2, _to: &IVec2) -> Cost {
    1.
  }

  /// The straight-line distance between two points.
  pub fn euclidean_distance(from: &IVec2, to: &IVec2) -> Cost {
    let dx = to.x - from.x;
    let dy = to.y - from.y;

    (dx * dx + dy * dy) as f32
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::collections::Grid;

  impl PathFindingGrid for Grid<bool> {
    fn get_neighbours(&self, center: IVec2, results: &mut NeighbourList<IVec2>) {
      for neighbour in center.von_neighbours() {
        if self.is_valid(neighbour.x, neighbour.y) {
          unsafe {
            if *self.get_unchecked(neighbour.x, neighbour.y) {
              results.push(neighbour);
            }
          }
        }
      }
    }
  }

  #[test]
  fn test_find_path() {
    let mut grid = Grid::new(4, 4);

    grid.fill(true);

    let start = ivec2(0, 0);
    let goal = ivec2(3, 3);

    let path = grid.find_path(start, goal, heuristics::euclidean_distance);

    assert!(path.is_some());
  }
}
