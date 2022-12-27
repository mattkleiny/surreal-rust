//! Editor support for Surreal
//!
//! Editing include scene management, resource loaders, hot loading, plugins,
//! inspectors, reflection and a central message bus as well as a UI that
//! can be composed for differing workflows depending on the game being built.

#[macro_use]
extern crate serde;

pub use reflection::*;
pub use resources::*;
pub use scenes::*;
pub use ui::*;

mod reflection;
mod resources;
mod scenes;
mod ui;
