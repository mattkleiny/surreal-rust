//! A simple framework for game development.

pub use animation::*;
pub use entities::*;
pub use sprites::*;
pub use states::*;

use super::*;

mod animation;
mod assets;
mod sprites;
mod states;
mod entities;
