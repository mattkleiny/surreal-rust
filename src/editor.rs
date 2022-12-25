//! Editor support for Surreal
//!
//! Editing include scene management, resource loaders, hot loading, plugins,
//! inspectors, reflection and a central message bus.

pub use reflection::*;
pub use resources::*;
pub use scenes::*;

mod reflection;
mod resources;
mod scenes;

/// Possible errors for registering a type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TypeError {
  InvalidType,
}

/// Allows centrally registering types for use in editor subsystems.
pub struct TypeDatabase {}

impl TypeDatabase {
  /// Registers a new type in the type database, allowing it's later
  /// querying and use in other subsystems.
  pub fn register_type<T: Reflect>() -> Result<(), TypeError> {
    todo!()
  }
}
