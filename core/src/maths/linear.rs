//! Mathematical utilities for linear algebra.

pub use matrices::*;
pub use planes::*;
pub use quaternions::*;
pub use rays::*;
pub use vectors::*;

use super::*;

mod matrices;
mod planes;
mod quaternions;
mod rays;
mod vectors;

/// An integral point in 2-space.
pub type Point2 = Vector2<i32>;

/// An integral point in 3-space.
pub type Point3 = Vector2<i32>;
