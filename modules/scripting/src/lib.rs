//! Scripting support for Surreal.

use surreal::impl_rid_type;
use surreal::utilities::Variant;

pub mod lua;

// A unique ID for a script in a [`ScriptServer`].
impl_rid_type!(ScriptId);

/// A server abstraction for managing application scripts.
pub trait ScriptServer {
  // script management
  fn create_script(&self, prelude: &str) -> surreal::Result<ScriptId>;
  fn update_script(&self, script_id: ScriptId, source_code: &str) -> surreal::Result<()>;
  fn execute_script(&self, script_id: ScriptId, parameters: &[Variant]) -> surreal::Result<Variant>;
  fn delete_script(&self, script_id: ScriptId) -> surreal::Result<()>;
}
