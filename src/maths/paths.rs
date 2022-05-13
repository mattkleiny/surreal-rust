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
    todo!()
  }
}

pub mod heuristics {
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