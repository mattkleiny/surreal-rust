use std::ops::Deref;

use super::*;

/// A point in the path-finding grid.
pub type Point = Vector2<i32>;

/// A cost for path searches and relative queries.
pub type Cost = f32;

/// A heuristic function for path-finding.
pub type Heuristic = fn(&Point, &Point) -> Cost;

/// Represents a path of points.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path(Vec<Point>);

impl Path {
  /// The start point of the path.
  pub fn start(&self) -> Option<Point> {
    self.0.first().map(|value| *value)
  }

  /// The goal point of the path.
  pub fn goal(&self) -> Option<Point> {
    self.0.last().map(|value| *value)
  }
}

impl Deref for Path {
  type Target = [Point];

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

/// Permits exploratory path-finding over some connected grid.
pub trait PathFindingGrid {
  /// Gets the pathing cost between the given two points.
  fn get_cost(&self, from: Point, to: Point) -> Cost;

  /// Gets the potential neighbours around the given point.
  fn get_neighbours(&self, center: Point) -> Vec<Point>;

  /// Locates a path using A* from from the given start point to the given goal.
  fn find_path(&self, _start: Point, _goal: Point, _heuristic: Heuristic) -> Option<Path> {
    todo!()
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