//! Scripting support for Surreal.

use surreal::{io::VirtualPath, utilities::Variant};

#[cfg(feature = "gdscript")]
pub mod gdscript;
#[cfg(feature = "lua")]
pub mod lua;

/// A server abstraction for managing application scripts.
pub trait ScriptServer {
  /// The name of the scripting backend, for editor tooling/etc.
  fn name(&self) -> &str;

  /// The file extensions supported by this script server.
  fn extensions(&self) -> &[&str];

  // script management
  fn script_create(&self) -> surreal::Result<ScriptId>;
  fn script_load(&self, script_id: ScriptId, path: &VirtualPath) -> surreal::Result<()>;
  fn script_execute(&self, script_id: ScriptId, method: &str, arguments: &[Variant]) -> surreal::Result<Variant>;
  fn script_delete(&self, script_id: ScriptId) -> surreal::Result<()>;
}

surreal::impl_rid!(ScriptId);
