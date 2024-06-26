//! Shared abstractions for different modules of the engine.
//!
//! These abstractions allow 'interop' between disparate crates that otherwise
//! wouldn't know about each other. For example, the `graphics` module doesn't
//! know about the `scene` module, but graphics components can be present in a
//! scene.

pub use assets::*;
pub use clipboard::*;
pub use scenes::*;
pub use variant::*;

mod assets;
mod clipboard;
mod scenes;
mod variant;
