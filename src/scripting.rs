//! Scripting utilities.

// TODO: implement an in-game console based on the script engine (lua).
// TODO: implement implicit entity/component binding access (entity1.health or entity1.sprite.pivot = 50) to allow easy mutation from scripts
// TODO: implement broadcast groups (ala Godot) to allow simple event-like system.
// TODO: support 'interactive debugging' using an in-game console.
// TODO: abstract over scripting language, add debugging and profiling/etc.
// TODO: strongly emphasise duck-typing for game

pub type ScriptResult<T> = std::result::Result<T, Error>;

/// Abstracts over the scripting system supported by the engine.
pub trait ScriptEngine {}

#[derive(Debug)]
pub enum Error {
  General,
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::Scripting(error)
  }
}
