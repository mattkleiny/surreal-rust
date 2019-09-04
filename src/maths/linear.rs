//! A simple linear algebra module, with a focus on simple access for game development.

pub use matrices::*;
pub use points::*;
pub use quaternions::*;
pub use vectors::*;

use super::*;

mod matrices;
mod points;
mod quaternions;
mod vectors;
