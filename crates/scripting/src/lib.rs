//! Scripting engine for Surreal

pub use lang::*;
pub use variant::*;

mod lang;
mod variant;

/// Represents a script in a scripting language
pub trait Script {}
