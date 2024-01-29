//! Constructive Solid Geometry (CSG) is a modeling technique that uses Boolean
//! operations like union and intersection to combine 3D solids.
//!
//! This library implements CSG operations on meshes elegantly and concisely
//! using BSP trees, and is meant to serve as an easily understandable
//! implementation of the algorithm.

/// Represents a node that allows Constructive Solid Geometry (CSG) operations.
pub trait CSG {
  fn union(self, other: &dyn CSG) -> Box<dyn CSG>;
}
