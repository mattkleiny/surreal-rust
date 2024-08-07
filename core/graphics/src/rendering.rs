//! Rendering abstractions and pipelines.
//!
//! This is a series of components designed to make it simpler to build more
//! complex render pipelines than using the 'material', 'mesh', 'render targets'
//! etc. do alone.

pub use pipelines::*;
pub use queue::*;

use super::*;

mod pipelines;
mod queue;
