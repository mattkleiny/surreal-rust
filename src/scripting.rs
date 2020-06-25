//! Scripting support for Surreal.

// TODO: implement an in-game console based on the script engine (lua).
// TODO: implement implicit entity/component binding access (entity1.health or entity1.sprite.pivot = 50) to allow easy mutation from scripts
// TODO: implement broadcast groups (ala Godot) to allow simple event-like system.
// TODO: support 'interactive debugging' using an in-game console.

use rlua::prelude::*;

/// A simple script engine built with Lua.
pub struct ScriptEngine {
  lua: Lua
}

impl ScriptEngine {
  pub fn new() -> Self {
    Self { lua: Lua::new() }
  }

  pub fn execute<S: AsRef<str>>(&self, code: S) {
    self.lua.context(|lua| {
      lua.load(code.as_ref()).eval().unwrap_or_else(|_| panic!("Failed to execute!"));
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_execute_simple_scripts() {
    let engine = ScriptEngine::new();

    engine.execute("print 'Hello, World!'");
  }
}