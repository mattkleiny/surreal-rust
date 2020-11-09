//! Path-finding utilities.

use std::collections::{HashMap, HashSet};

use smallvec::SmallVec;

use crate::collections::MinHeap;

pub type Point = super::Vector2<i32>;
pub type Cost = f64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path(Vec<Point>);

/// Permits exploratory path-finding over some connected grid.
pub trait PathFindingGrid {
  fn get_cost(&self, from: Point, to: Point) -> Cost;
  fn get_neighbours(&self, center: Point) -> SmallVec<[Point; 8]>;

  /// Locate a path from the given start point to the given goal via a heuristic.
  fn find_path(&self, start: Point, goal: Point, heuristic: impl Fn(Point, Point) -> Cost) -> Option<Path> {
    /// Rebuilds the path taken to get to the destination.
    fn rebuild_path(start: Point, goal: Point, came_from: HashMap<Point, Point>) -> Path {
      let mut result = Vec::new();
      let mut current = goal;

      while current != start {
        result.push(current);

        if current == start {
          break;
        }

        current = *came_from.get(&current).unwrap();
      }

      result.push(start);
      result.reverse();

      Path(result)
    }

    let mut visited = HashSet::new();

    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();

    let mut frontier = MinHeap::new();

    frontier.push(start, 0.);
    visited.insert(start);

    came_from.insert(start, start);
    cost_so_far.insert(start, 0.);

    while frontier.size() > 0 {
      match frontier.pop() {
        None => break,
        Some(current) if current == goal => {
          return Some(rebuild_path(start, goal, came_from));
        }
        Some(current) => {
          for neighbour in self.get_neighbours(current) {
            let new_cost = cost_so_far.get(&current).unwrap() + self.get_cost(current, neighbour);

            if cost_so_far.contains_key(&neighbour) || new_cost < *cost_so_far.get(&neighbour).unwrap() {
              if cost_so_far.contains_key(&neighbour) {
                cost_so_far.remove(&neighbour);
                came_from.remove(&neighbour);
              }

              cost_so_far.insert(neighbour, new_cost);
              came_from.insert(neighbour, current);

              let priority = new_cost + heuristic(neighbour, goal);

              frontier.push(neighbour, priority);
            }
          }
        }
      }
    };

    None
  }
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

mod heuristics {
  use super::*;

  #[inline]
  pub fn constant(from: Point, to: Point) -> Cost {
    1.
  }

  #[inline]
  pub fn euclidean_distance(from: Point, to: Point) -> Cost {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{DenseGrid, vec2};

  use super::*;

  #[test]
  fn it_should_find_a_simple_path() {
    let grid = DenseGrid::new(16, 16, 1.);

    let start = vec2(0, 0);
    let goal = vec2(15, 15);

    let path = grid.find_path(start, goal, heuristics::euclidean_distance)
      .expect("Expected to locate a valid path!");
  }
}