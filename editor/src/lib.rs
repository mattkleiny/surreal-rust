//! Editor support for Surreal
//!
//! Editing include scene management, resource loaders, hot loading, plugins,
//! inspectors, reflection and a central message bus as well as a UI that
//! can be composed for differing workflows depending on the game being built.

#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(box_into_inner)]

pub use assets::*;
pub use domain::*;
pub use events::*;
pub use projects::*;
pub use reflect::*;
pub use serialize::*;
pub use ui::*;

mod assets;
mod domain;
mod events;
mod projects;
mod reflect;
mod serialize;
mod ui;
