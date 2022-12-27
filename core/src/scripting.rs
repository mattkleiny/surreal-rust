//! Scripting support for the game.
//!
//! Scripting is provided via a [ScriptLanguage] trait. An implementation of
//! this trait surfaces all of the required metadata required to engage script
//! management by the engine.

/// Represents a scripting language.
pub trait ScriptLanguage {
  /// A unique name for the language.
  fn name(&self) -> &str;

  /// The possible file extensions for scripts in this language.
  fn file_extension(&self) -> &[&str];
}

/// Represents a script.
pub trait Script {
  /// The language that this script is written in.
  fn language(&self) -> &dyn ScriptLanguage;
}
