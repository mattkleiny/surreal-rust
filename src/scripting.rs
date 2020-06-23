//! Scripting support for Surreal.

// TODO: implement an in-game console based on the script engine (lua).
// TODO: implement implicit entity/component binding access (entity1.health or entity1.sprite.pivot = 50) to allow easy mutation from scripts
// TODO: implement broadcast groups (ala godot) to allow simple event-like system.
// TODO: support 'interactive debugging' using an in-game console.

/// An abstraction over a scripting engine for use in scripted applications.
///
/// This is a low-level abstraction that allows us to decouple specific scripting back-ends from interaction with those
/// backends. This interface assumes a very simple scripting provider, and is designed to be simple to consume and
/// work with from a high level.
pub trait ScriptEngine {
  type Error;
  type Code;

  /// Attempts to execute the given raw code string on the engine.
  fn execute<C: AsRef<str>>(&mut self, code: C) -> Result<(), Self::Error>;
}