//! Editor support for Surreal
//!
//! Editing include scene management, resource loaders, hot loading, plugins,
//! inspectors, reflection and a central message bus as well as a UI that
//! can be composed for differing workflows depending on the game being built.

#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(box_into_inner)]
#![feature(const_trait_impl)]

pub use assets::*;
pub use projects::*;
pub use ui::*;

mod assets;
mod projects;
mod ui;
