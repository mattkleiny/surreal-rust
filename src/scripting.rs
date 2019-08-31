//! Scripting support for Surreal.

use rlua::prelude::*;
pub use rlua::prelude::LuaFunction;

use crate::diagnostics::*;

// TODO: implement an in-game console based on the script engine (lua).
// TODO: support 'interactive debugging' using an in-game console.
// TODO: implement implicit entity/component binding access (entity1.health or entity1.sprite.pivot = 50) to allow easy mutation from scripts
// TODO: replace specs with something more generally mutable, as this would simplify interaction from scripts and more rapid prototyping
// TODO: implement broadcast groups (ala godot) to allow simple event-like system.
// TODO: build a console utility using imgui that will allow execution of arbitrary commands/display log output.

/// A scripting engine for Lua.
///
/// This engine wraps the rlua interpreter with some default behaviour, and provides
/// a simple API for extension and usability.
pub struct ScriptEngine {
  lua: Lua,
}

impl ScriptEngine {
  pub fn new() -> Self {
    Self {
      lua: Self::create_interpreter(),
    }
  }

  /// Creates a new interpreter for the script engine.
  fn create_interpreter() -> Lua {
    let lua = Lua::new();

    // initialize the global table
    let result: rlua::Result<()> = lua.context(|context| {
      let globals = context.globals();

      let trace = context.create_function(|_, message: LuaString| {
        trace!("{}", message.to_str()?);
        Ok(())
      })?;
      let debug = context.create_function(|_, message: LuaString| {
        debug!("{}", message.to_str()?);
        Ok(())
      })?;
      let warn = context.create_function(|_, message: LuaString| {
        warn!("{}", message.to_str()?);
        Ok(())
      })?;
      let error = context.create_function(|_, message: LuaString| {
        error!("{}", message.to_str()?);
        Ok(())
      })?;

      globals.set("trace", trace)?;
      globals.set("debug", debug)?;
      globals.set("warn", warn)?;
      globals.set("error", error)?;

      Ok(())
    });

    match result {
      Err(error) => panic!("{}", error),
      _ => {}
    }

    lua
  }

  /// Sets the memory limit on the interpreter.
  pub fn set_memory_limit(&mut self, limit: Option<usize>) {
    self.lua.set_memory_limit(limit);
  }

  /// Evaluates the given code in the script runtime and returns the result.
  pub fn evaluate<F, R>(&mut self, body: F) -> R
    where F: FnOnce(LuaContext) -> R {
    self.lua.context(body)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn engine_should_evaluate_basic_instructions_without_fault() {
    let mut engine = ScriptEngine::new();

    engine.evaluate(|context| {
      context.load("print 'Hello, World!'").exec().unwrap();
    });
  }

  #[test]
  fn engine_should_execute_native_log_methods() {
    let mut engine = ScriptEngine::new();

    engine.evaluate(|context| {
      context.load("trace 'Hello, World!'").exec().unwrap();
      context.load("debug 'Hello, World!'").exec().unwrap();
      context.load("warn 'Hello, World!'").exec().unwrap();
      context.load("error 'Hello, World!'").exec().unwrap();
    });
  }
}