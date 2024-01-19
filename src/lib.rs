//! Root project for the Surreal engine.
//!
//! Surreal is a lightweight game engine built with Rust. It offers an
//! opinionated set of basic functionalities that game developers require to
//! get-things-done, without over-emphasising non goals such as performance or
//! feature-completeness.
//!
//! The engine is built on top of a number of core libraries, which are all
//! available as separate crates. This root project can be added as a
//! dependency, and features enabled/disabled to toggle the set of requirements
//! needed for any particular project.

#[cfg(feature = "assets")]
pub extern crate assets;
#[cfg(feature = "audio")]
pub extern crate audio;
pub extern crate common;
#[cfg(feature = "editor")]
pub extern crate editor;
#[cfg(feature = "graphics")]
pub extern crate graphics;
#[cfg(feature = "input")]
pub extern crate input;
#[cfg(feature = "physics")]
pub extern crate physics;
#[cfg(feature = "scene")]
pub extern crate scene;
#[cfg(feature = "ui")]
pub extern crate ui;

pub mod backends {
  #[cfg(feature = "sdl")]
  pub extern crate sdl;
}

pub mod prelude {
  #[cfg(feature = "assets")]
  pub use assets::*;
  #[cfg(feature = "audio")]
  pub use audio::*;
  pub use common::*;
  #[cfg(feature = "editor")]
  pub use editor::*;
  #[cfg(feature = "graphics")]
  pub use graphics::*;
  #[cfg(feature = "input")]
  pub use input::*;
  #[cfg(feature = "physics")]
  pub use physics::*;
  #[cfg(feature = "scene")]
  pub use scene::*;
  #[cfg(feature = "ui")]
  pub use ui::*;
}
