//! A lightweight game engine for Rust.

#![allow(dead_code)]
#![allow(unused_variables)]

extern crate chrono;
extern crate enumflags2;
#[macro_use]
extern crate enumflags2_derive;
#[cfg(feature = "opengl")]
extern crate gl;
extern crate glam;
#[cfg(feature = "framework")]
extern crate hotwatch;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate log;
extern crate rand;
#[cfg(feature = "scripting")]
extern crate rlua;
extern crate sdl2;
#[cfg(feature = "framework")]
extern crate specs;

pub mod audio;
pub mod diagnostics;
#[cfg(feature = "editor")]
pub mod editor;
#[cfg(feature = "framework")]
pub mod framework;
pub mod collections;
pub mod graphics;
pub mod input;
pub mod maths;
pub mod platform;
#[cfg(feature = "scripting")]
pub mod scripting;
pub mod timing;

pub mod prelude {
  pub use crate::diagnostics::*;
}