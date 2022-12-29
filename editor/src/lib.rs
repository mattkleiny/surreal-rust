//! Editor support for Surreal
//!
//! Editing include scene management, resource loaders, hot loading, plugins,
//! inspectors, reflection and a central message bus as well as a UI that
//! can be composed for differing workflows depending on the game being built.

#![feature(anonymous_lifetime_in_impl_trait)]

pub use assets::*;
pub use reflection::*;
pub use resources::*;

mod assets;
mod reflection;
mod resources;
