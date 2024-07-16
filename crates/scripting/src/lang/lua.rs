//! Lua scripting language support

use common::Variant;
use mlua::{Lua, StdLib};

use super::*;

/// A runtime for executing Lua scripts.
pub struct LuaScriptRuntime {
  lua: Lua,
}

impl LuaScriptRuntime {
  pub fn new() -> Self {
    let lua = Lua::new();

    lua.load_from_std_lib(StdLib::ALL_SAFE).unwrap();

    Self { lua }
  }
}

impl ScriptRuntime for LuaScriptRuntime {
  fn eval(&self, code: &str) -> Result<Variant, ScriptError> {
    self
      .lua
      .load(code)
      .eval()
      .map_err(|it| ScriptError::ExecutionError(it.to_string()))
  }

  fn eval_as<R: FromScriptValue>(&self, _code: &str) -> Result<R, ScriptError> {
    todo!()
  }

  fn add_callback<F>(&mut self, _name: &str, _callback: impl ScriptCallback<F> + 'static) {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_lua_evaluation() {
    let runtime = LuaScriptRuntime::new();

    let result = runtime.eval("return 42").unwrap();

    assert_eq!(result, Variant::I32(42));
  }
}
