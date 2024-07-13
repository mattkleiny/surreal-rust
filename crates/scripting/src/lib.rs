//! Scripting engine for Surreal

pub use lang::*;

mod lang;

/// Represents a script in a scripting language
pub trait Script {}
