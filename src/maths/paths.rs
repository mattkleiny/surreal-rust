use std::collections::HashMap;

use crate::collections::MinHeap;

/// A point in the path-finding grid.
pub type Point = super::Vector2<i32>;

/// A cost for path searches and relative queries.
pub type Cost = f64;

/// A heuristic function for path-finding.
pub type Heuristic = fn(&Point, &Point) -> Cost;

/// Represents a path of points.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path(Vec<Point>);

impl Path {
  /// The start point of the path.
  #[inline]
  pub fn start(&self) -> Point { self.0[0] }

  /// The goal point of the path.
  #[inline]
  pub fn goal(&self) -> Point { self.0[self.0.len() - 1] }

  /// Returns all points in the path as a slice.
  #[inline]
  pub fn as_slice(&self) -> &[Point] { &self.0 }
}

/// Permits exploratory path-finding over some connected grid.
pub trait PathFindingGrid {
  /// Gets the pathing cost between the given two points.
  fn get_cost(&self, from: Point, to: Point) -> Cost;

  /// Gets the potential neighbours around the given point.
  fn get_neighbours(&self, center: Point) -> Vec<Point>;

  /// Locates a path using A* from from the given start point to the given goal.
  fn find_path(&self, start: Point, goal: Point, heuristic: Heuristic) -> Option<Path> {
    /// Represents a node that's already been visited in the path.
    struct Segment {
      from: Point,
      cost: Cost,
    }

    /// Rebuilds the path taken to get to the destination.
    fn rebuild_path(start: Point, goal: Point, mut segments: HashMap<Point, Segment>) -> Path {
      let mut result = Vec::new();
      let mut current = goal;

      while current != start {
        result.push(current);

        if current == start {
          break;
        }

        current = segments.remove(&current).unwrap().from;
      }

      result.push(start);
      result.reverse();

      Path(result)
    }

    let mut frontier = MinHeap::new();
    let mut segments = HashMap::new();

    frontier.push(start, 0.);
    segments.insert(start, Segment { from: start, cost: 0. });

    while frontier.size() > 0 {
      match frontier.pop() {
        None => break,
        Some(current) if current == goal => {
          return Some(rebuild_path(start, goal, segments));
        }
        Some(current) => for neighbour in self.get_neighbours(current) {
          let new_cost = segments[&current].cost + self.get_cost(current, neighbour);

          if !segments.contains_key(&neighbour) || new_cost < segments[&neighbour].cost {
            segments.insert(neighbour, Segment { from: current, cost: new_cost });

            let priority = new_cost + heuristic(&neighbour, &goal);

            frontier.push(neighbour, new_cost);
          }
        }
      }
    };

    None
  }
}

// Generic implementation for any grid.
impl<G> PathFindingGrid for G where G: crate::maths::Grid {
  #[inline(always)]
  fn get_cost(&self, from: Point, to: Point) -> f64 { 1. }

  fn get_neighbours(&self, center: Point) -> Vec<Point> {
    use super::automata::MooreNeighbourhood;

    let mut results = Vec::new();

    for neighbour in center.get_moore_neighbours() {
      let point = (neighbour.x as usize, neighbour.y as usize);

      if self.is_valid(point) {
        results.push(neighbour);
      }
    }

    results
  }
}

pub mod heuristics {
  //! Path-finding heuristic functions.

  use super::*;

  /// A constant distance
  pub fn constant(from: &Point, to: &Point) -> Cost { 1. }

  /// The straight-line distance between two points.
  pub fn euclidean_distance(from: &Point, to: &Point) -> Cost {
    let dx = to.x - from.x;
    let dy = to.y - from.y;

    (dx * dx + dy * dy) as Cost
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{DenseGrid, vec2};

  use super::*;

  #[test]
  fn grid_should_find_a_simple_path() {
    let grid = DenseGrid::new(16, 16, 1.);

    let start = vec2(0, 0);
    let goal = vec2(15, 15);

    let path = grid.find_path(start, goal, heuristics::euclidean_distance)
        .expect("Expected to locate a valid path!");
  }
}