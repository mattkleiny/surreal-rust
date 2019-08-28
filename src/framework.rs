//! A simple framework for game development.

pub use assets::*;
pub use entities::*;
pub use states::*;

mod assets;
mod states;
mod entities;

// TODO: support animation utilities.