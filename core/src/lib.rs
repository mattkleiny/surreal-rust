//! A lightweight game engine for Rust.
//!
//! Surreal is designed to be a simple and flexible game engine, not unlike libGDX or MonoGame.
//!
//! It's opinionated, but small in scope and is intended to form a solid 'library'-like toolkit
//! for constructing small but fast games. A lot of the work is left to the author as to how they'd
//! like to glue things together.

#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(const_refs_to_cell)]

#[macro_use]
extern crate serde;

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
pub mod scripting;
pub mod ui;
pub mod utilities;

/// Represents a result type in any part of the engine.
pub type Result<T> = anyhow::Result<T>;
