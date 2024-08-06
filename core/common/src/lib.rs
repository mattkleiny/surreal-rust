//! Core components for the Surreal game engine.
//!
//! This crate contains common utilities, collections, diagnostics and other
//! general purpose code for use in other systems.

#![allow(incomplete_features)]
#![allow(async_fn_in_trait)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(noop_waker)]
#![feature(ptr_as_ref_unchecked)]
#![feature(box_into_inner)]
#![feature(allocator_api)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(downcast_unchecked)]
#![feature(async_closure)]

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

pub use macros::{profiling, Asset, Deserialize, Reflect, Serialize, Trace};
