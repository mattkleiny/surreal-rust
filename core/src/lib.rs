//! Core components for the Surreal game engine.
//!
//! This crate contains common utilities, collections, diagnostics and other
//! general purpose code for use in other systems.

#![allow(incomplete_features)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(async_fn_in_trait)]
#![feature(test)]
#![feature(trivial_bounds)]
#![feature(type_alias_impl_trait)]

#[macro_use]
extern crate serde;
extern crate test;

pub use anyhow::{anyhow, bail, Error, Result};
pub use macros;

pub mod assets;
pub mod audio;
pub mod collections;
pub mod diagnostics;
pub mod engine;
pub mod fibers;
pub mod graphics;
pub mod graphs;
pub mod input;
pub mod io;
pub mod maths;
pub mod physics;
pub mod scene;
pub mod ui;
pub mod utilities;
