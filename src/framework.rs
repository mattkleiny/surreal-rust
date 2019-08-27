//! A simple framework for game development.

pub use assets::*;
pub use entities::*;
pub use states::*;

use super::*;

mod assets;
mod states;
mod entities;

// TODO: support animation utilities.