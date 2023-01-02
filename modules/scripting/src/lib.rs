//! Scripting support for Surreal.

use surreal::utilities::{Variant, RID};

pub mod lua;

/// The [`RID`] for a script in a [`ScriptServer`].
pub type ScriptId = RID;

/// A server abstraction for managing application scripts.
pub trait ScriptServer {
  // script management
  fn create_script(&self, prelude: &str) -> surreal::Result<ScriptId>;
  fn update_script(&self, script_id: ScriptId, source_code: &str) -> surreal::Result<()>;
  fn execute_script(&self, script_id: ScriptId, parameters: &[Variant]) -> surreal::Result<Variant>;
  fn delete_script(&self, script_id: ScriptId) -> surreal::Result<()>;
}
