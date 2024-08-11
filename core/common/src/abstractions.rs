//! Shared abstractions for different modules of the engine.

pub use assets::*;
pub use callbacks::*;
pub use platform::*;
pub use services::*;
pub use variant::*;

mod assets;
mod callbacks;
mod platform;
mod services;
mod variant;
