//! An embedded LUA scripting runtime.

use rlua::prelude::*;

use crate::scripting::ScriptEngine;

/// A simple script engine built with Lua.
pub struct LuaScriptEngine {
  lua: Lua
}

impl LuaScriptEngine {
  pub fn new() -> Self {
    Self { lua: Lua::new() }
  }

  pub fn execute<S: AsRef<str>>(&self, code: S) {
    self.lua.context(|lua| {
      lua.load(code.as_ref()).eval().unwrap_or_else(|_| panic!("Failed to execute!"));
    })
  }
}

impl ScriptEngine for LuaScriptEngine {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_execute_simple_scripts() {
    let engine = LuaScriptEngine::new();

    engine.execute("print 'Hello, World!'");
  }
}
