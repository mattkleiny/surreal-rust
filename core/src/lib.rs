//! Core components for the Surreal game engine.
//!
//! This crate contains common utilities, collections, diagnostics and other
//! general purpose code for use in other systems.

#![allow(incomplete_features)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(async_fn_in_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(test)]
#![feature(trivial_bounds)]
#![feature(type_alias_impl_trait)]

#[macro_use]
extern crate serde;
extern crate test;

pub use anyhow::{anyhow, bail, Error, Result};
pub use macros;

pub mod collections;
pub mod diagnostics;
pub mod io;
pub mod maths;
pub mod utilities;
