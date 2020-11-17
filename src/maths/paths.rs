use std::collections::HashMap;

use smallvec::SmallVec;

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
  fn get_cost(&self, from: Point, to: Point) -> Cost;
  fn get_neighbours(&self, center: Point) -> SmallVec<[Point; 8]>;

  /// Locate a path from the given start point to the given goal via a heuristic.
  fn find_path(&self, start: Point, goal: Point, heuristic: Heuristic) -> Option<Path> {
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();

    let mut frontier = MinHeap::new();

    frontier.push(start, 0.);

    came_from.insert(start, start);
    cost_so_far.insert(start, 0.);

    while frontier.size() > 0 {
      match frontier.pop() {
        None => break,
        Some(current) if current == goal => {
          return Some(rebuild_path(start, goal, came_from));
        }
        Some(current) => for neighbour in self.get_neighbours(current) {
          let new_cost = cost_so_far[&current] + self.get_cost(current, neighbour);

          if !cost_so_far.contains_key(&neighbour) || new_cost < cost_so_far[&neighbour] {
            cost_so_far.insert(neighbour, new_cost);
            came_from.insert(neighbour, current);

            let priority = new_cost + heuristic(&neighbour, &goal);

            frontier.push(neighbour, new_cost);
          }
        }
      }
    };

    None
  }
}

/// Rebuilds the path taken to get to the destination.
fn rebuild_path(start: Point, goal: Point, mut came_from: HashMap<Point, Point>) -> Path {
  let mut result = Vec::new();
  let mut current = goal;

  while current != start {
    result.push(current);

    if current == start {
      break;
    }

    current = came_from.remove(&current).unwrap();
  }

  result.push(start);
  result.reverse();

  Path(result)
}

// Generic implementation for any grid space.
impl<T> PathFindingGrid for super::DenseGrid<T> {
  #[inline(always)]
  fn get_cost(&self, from: Point, to: Point) -> f64 { 1. }

  fn get_neighbours(&self, center: Point) -> SmallVec<[Point; 8]> {
    use super::automata::*;

    center.get_moore_neighbours()
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