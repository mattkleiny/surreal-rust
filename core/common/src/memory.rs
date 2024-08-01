//! Memory management tools

pub use gc::*;
pub use pointer::*;
pub use stack::*;

mod gc;
mod pointer;
mod stack;
