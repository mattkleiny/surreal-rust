//! Mathematical utilities.
//!
//! This is a set of common utilities for scalar and linear 2d and 3d mathematics.

pub use directions::*;
pub use interpolation::*;
pub use grids::*;
pub use linear::*;
pub use random::*;

mod directions;
mod interpolation;
mod grids;
mod linear;
mod random;

