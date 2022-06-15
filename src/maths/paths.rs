use std::collections::{BinaryHeap, HashMap, VecDeque};

use super::*;

/// Arbitrary upper limit on the number of steps to use in the find_path function.
const MAXIMUM_STEPS: usize = 128;

/// A point in the path-finding grid.
pub type Point = Vector2<i32>;

/// A cost for path searches and relative queries.
pub type Cost = f32;

/// A heuristic function for path-finding.
pub type Heuristic = fn(&Point, &Point) -> Cost;

/// Represents a path of points.
pub type NeighbourSet = smallvec::SmallVec<[Point; 9]>;

/// Permits exploratory path-finding over some connected grid.
pub trait PathFindingGrid {
  /// Gets the pathing cost between the given two points.
  fn get_cost(&self, from: Point, to: Point) -> Cost;

  /// Gets the potential neighbours around the given point.
  fn get_neighbours(&self, center: Point) -> NeighbourSet;

  /// Locates a path using A* from from the given start point to the given goal.
  fn find_path(&self, start: Point, goal: Point, heuristic: Heuristic) -> Option<Vec<Point>> {
    let mut frontier = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();

    came_from.insert(start, start);
    cost_so_far.insert(start, 0.);

    frontier.push(PathNode(start, 0.));

    while let Some(PathNode(current, _)) = frontier.pop() {
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
      let neighbours = self.get_neighbours(current);
      for neighbour in neighbours {
        let new_cost = cost_so_far[&current] + self.get_cost(current, neighbour);

        if !cost_so_far.contains_key(&neighbour) || new_cost < cost_so_far[&neighbour] {
          // back track along neighbours?
          if cost_so_far.contains_key(&neighbour) {
            cost_so_far.remove(&neighbour);
            came_from.remove(&neighbour);
          }

          cost_so_far.insert(neighbour, new_cost);
          came_from.insert(neighbour, current);

          let priority = new_cost + heuristic(&neighbour, &goal);

          frontier.push(PathNode(neighbour, priority));
        }
      }
    }

    None
  }
}

/// Represents a node in our potential path finding solution.
///
/// Orderable and equable for use in a `BinaryHeap`.
struct PathNode(Point, f32);

impl PartialEq for PathNode {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}

impl Eq for PathNode {}

impl PartialOrd for PathNode {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.1.partial_cmp(&other.1)
  }
}

impl Ord for PathNode {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.1.partial_cmp(&other.1).unwrap()
  }
}

pub mod heuristics {
  //! Heuristic functions for path-finding.

  use super::*;

  /// A constant distance
  pub fn constant(_from: &Point, _to: &Point) -> Cost {
    1.
  }

  /// The straight-line distance between two points.
  pub fn euclidean_distance(from: &Point, to: &Point) -> Cost {
    let dx = to.x - from.x;
    let dy = to.y - from.y;

    (dx * dx + dy * dy) as Cost
  }
}
