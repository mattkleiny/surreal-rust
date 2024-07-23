//! Core components for the Surreal game engine.
//!
//! This crate contains common utilities, collections, diagnostics and other
//! general purpose code for use in other systems.

#![allow(incomplete_features)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(noop_waker)]
#![feature(ptr_as_ref_unchecked)]
#![feature(box_into_inner)]

pub use abstractions::*;
pub use collections::*;
pub use concurrency::*;
pub use diagnostics::*;
pub use io::*;
pub use maths::*;
pub use scenes::*;
pub use strings::*;
pub use utilities::*;

mod abstractions;
mod collections;
mod concurrency;
mod diagnostics;
mod io;
mod maths;
mod scenes;
mod strings;
mod utilities;

// Re-export macros for use in other core.
pub use macros::{profiling, Asset, Deserialize, Reflect, Serialize, Singleton, Trace};
