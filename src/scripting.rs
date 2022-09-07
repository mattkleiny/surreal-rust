//! Scripting support for the game.
//!
//! Scripting is provided via a [ScriptLanguage] trait. An implementation of
//! this trait surfaces all of the required metadata required to engage script
//! management by the engine.

/// Represents a script.
pub trait Script {
  fn language(&self) -> &dyn ScriptLanguage;
}

/// Represents a scripting language.
pub trait ScriptLanguage {
  fn name(&self) -> &str;
  fn file_extension(&self) -> &str;
}
