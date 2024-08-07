//! Mathematical utilities for linear algebra.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub use aabb::*;
pub use frustum::*;
pub use glam::*;
pub use planes::*;
pub use rays::*;
pub use scalars::*;
pub use vectors::*;

use super::*;

mod aabb;
mod frustum;
mod planes;
mod rays;
mod scalars;
mod vectors;

/// Represents a numerical space with identity constants
pub trait Identity {
  const ZERO: Self;
  const ONE: Self;
  const MIN: Self;
  const MAX: Self;
}
