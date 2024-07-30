//! Root project for the Surreal engine.
//!
//! Surreal is a lightweight game engine built with Rust. It offers an
//! opinionated set of basic functionalities that game developers require to
//! get-things-done, without _over-emphasising_ non-goals such as performance or
//! feature-completeness (though these are still important, of course, we just
//! don't want to overdo it).
//!
//! The engine is built on top of a number of core libraries, which are all
//! available as separate core. This root project can be added as a
//! dependency, and features enabled/disabled to toggle the set of requirements
//! needed for any particular project.

#![no_std]

#[cfg(feature = "audio")]
pub extern crate audio;
pub extern crate common;
#[cfg(feature = "editor")]
pub extern crate editor;
#[cfg(feature = "graphics")]
pub extern crate graphics;
#[cfg(feature = "input")]
pub extern crate input;
#[cfg(feature = "networking")]
pub extern crate networking;
#[cfg(feature = "physics")]
pub extern crate physics;
#[cfg(feature = "scenes")]
pub extern crate scenes;
#[cfg(feature = "scripting")]
pub extern crate scripting;

pub mod backends {
  #[cfg(feature = "sdl")]
  pub extern crate sdl;
}
