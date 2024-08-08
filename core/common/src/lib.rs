//! Core components for the Surreal game engine.
//!
//! This crate contains common utilities, collections, diagnostics and other
//! general purpose code for use in other systems.

#![allow(async_fn_in_trait)]
#![feature(allocator_api)]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(noop_waker)]
#![feature(async_closure)]

extern crate self as common;

pub use abstractions::*;
pub use collections::*;
pub use concurrency::*;
pub use diagnostics::*;
pub use io::*;
pub use maths::*;
pub use memory::*;
pub use network::*;
pub use strings::*;
pub use utilities::*;

mod abstractions;
mod collections;
mod concurrency;
mod diagnostics;
mod io;
pub mod lua;
mod maths;
mod memory;
mod network;
mod strings;
mod utilities;

pub use macros::{profiling, Asset, Deserialize, Reflect, Serialize, Singleton, Trace};
