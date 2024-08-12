//! Shared abstractions for different modules of the engine.

pub use assets::*;
pub use callbacks::*;
pub use platform::*;
pub use serialized::*;
pub use services::*;
pub use variant::*;

mod assets;
mod callbacks;
mod platform;
mod serialized;
mod services;
mod variant;
