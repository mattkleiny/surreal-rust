//! Scripting support for Surreal.

use rlua::{Context, Lua, MetaMethod, UserData, UserDataMethods};

use crate::maths::Vec2i;

use super::*;

pub use rlua::Function;

/// A scripting engine for Lua.
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
    lua.context(|context| {
      let globals = context.globals();
      globals.set("vec2", context.create_function(|_, (x, y): (i32, i32)| Ok(Vec2i::new(x, y))).unwrap()).unwrap()
    });

    lua
  }

  /// Sets the memory limit on the interpreter.
  pub fn set_memory_limit(&mut self, limit: Option<usize>) {
    self.lua.set_memory_limit(limit);
  }

  /// Evaluates the given code in the script runtime and returns the result.
  pub fn evaluate<F, R>(&mut self, body: F) -> R
    where F: FnOnce(Context) -> R {
    self.lua.context(body)
  }
}

impl UserData for Vec2i {
  fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
    methods.add_method("normal", |_, vec: &Vec2i, ()| { Ok(vec.x + vec.y) });
    methods.add_meta_function(MetaMethod::Add, |_, (vec1, vec2): (Vec2i, Vec2i)| {
      Ok(Vec2i::new(vec1.x + vec2.y, vec1.y + vec2.y))
    });
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_evaluate_basic_instructions_without_fault() {
    let mut engine = ScriptEngine::new();

    engine.execute("print 'Hello, World!'").unwrap();
  }
}