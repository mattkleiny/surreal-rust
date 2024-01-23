//! Core components for the Surreal game engine.
//!
//! This crate contains common utilities, collections, diagnostics and other
//! general purpose code for use in other systems.

#![allow(incomplete_features)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(lazy_cell)]

#[macro_use]
extern crate serde;

pub use anyhow::{anyhow, bail, Error, Result};
pub use collections::*;
pub use diagnostics::*;
pub use io::*;
pub use macros;
pub use maths::*;
pub use strings::*;
pub use utilities::*;

mod collections;
mod diagnostics;
mod io;
mod maths;
mod strings;
mod utilities;

// TODO: allow no_std as a target for the engine
