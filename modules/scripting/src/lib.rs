//! Scripting support for Surreal.

pub mod lua;

// A unique ID for a script in a [`ScriptServer`].
surreal::impl_rid!(ScriptId);

/// A server abstraction for managing application scripts.
pub trait ScriptServer {
  // script management
  fn create_script(&self, prelude: &str) -> surreal::Result<ScriptId>;
  fn delete_script(&self, script_id: ScriptId) -> surreal::Result<()>;
}
