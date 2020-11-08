//! Scripting utilities.

// TODO: implement an in-game console based on the script engine (lua).
// TODO: implement implicit entity/component binding access (entity1.health or entity1.sprite.pivot = 50) to allow easy mutation from scripts
// TODO: implement broadcast groups (ala Godot) to allow simple event-like system.
// TODO: support 'interactive debugging' using an in-game console.
// TODO: abstract over scripting language, add debugging and profiling/etc.
// TODO: strongly emphasise duck-typing for game

/// Represents an engine for scripting support.
pub trait ScriptEngine {}
