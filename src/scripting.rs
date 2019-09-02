//! Scripting support for Surreal.

// TODO: implement an in-game console based on the script engine (lua).
// TODO: support 'interactive debugging' using an in-game console.
// TODO: implement implicit entity/component binding access (entity1.health or entity1.sprite.pivot = 50) to allow easy mutation from scripts
// TODO: replace specs with something more generally mutable, as this would simplify interaction from scripts and more rapid prototyping
// TODO: implement broadcast groups (ala godot) to allow simple event-like system.
// TODO: build a console utility using imgui that will allow execution of arbitrary commands/display log output.

pub struct ScriptEngine {}

impl ScriptEngine {
  pub fn new() -> Self {
    Self {}
  }
}