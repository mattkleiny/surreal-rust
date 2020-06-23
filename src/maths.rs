//! Mathematical utilities.
//!
//! This is a set of common utilities for scalar and linear 2d and 3d mathematics.

pub use cgmath::*;

pub use directions::*;
pub use interp::*;
pub use grids::*;
pub use random::*;

mod directions;
mod interp;
mod grids;
mod random;

