//! Shared abstractions for different modules of the engine.

pub use assets::*;
pub use memory::*;
pub use objects::*;
pub use os::*;
pub use variant::*;

mod assets;
mod memory;
mod objects;
mod os;
mod variant;
