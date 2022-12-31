//! Core components for the Surreal game engine.
//!
//! This crate contains common utilities, collections, diagnostics and other
//! general purpose code for use in other systems.

#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(const_refs_to_cell)]
#![feature(type_alias_impl_trait)]

#[macro_use]
extern crate serde;

pub use macros;

pub mod assets;
pub mod audio;
pub mod collections;
pub mod diagnostics;
pub mod engine;
pub mod graphics;
pub mod input;
pub mod io;
pub mod maths;
pub mod scene;
pub mod ui;
pub mod utilities;

/// Represents a result type in any part of the engine.
pub type Result<T> = anyhow::Result<T>;
