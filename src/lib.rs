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

#![no_std]

#[cfg(feature = "ai")]
pub extern crate ai;
#[cfg(feature = "assets")]
pub extern crate assets;
#[cfg(feature = "audio")]
pub extern crate audio;
pub extern crate common;
#[cfg(feature = "ecs")]
pub extern crate ecs;
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
#[cfg(feature = "scene")]
pub extern crate scene;
#[cfg(feature = "scripting")]
pub extern crate scripting;
#[cfg(feature = "ui")]
pub extern crate ui;

pub mod backends {
  #[cfg(feature = "console")]
  pub extern crate console;
  #[cfg(feature = "sdl")]
  pub extern crate sdl;
}
