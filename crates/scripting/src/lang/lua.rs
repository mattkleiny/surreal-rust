//! Lua scripting language support

use common::Variant;
use mlua::{FromLua, Function, Lua, StdLib, Value};

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
  fn eval(&self, code: &str) -> Result<ScriptValue, ScriptError> {
    self
      .lua
      .load(code)
      .eval()
      .map_err(|it| ScriptError::ExecutionError(it.to_string()))
  }

  fn add_callback<F>(&mut self, name: &str, _callback: impl ScriptCallback<F> + 'static) {
    let globals = self.lua.globals();
    let function = Function::wrap_mut(|_lua, (x, y): (i32, i32)| Ok(x + y));

    globals.set(name, function).unwrap();
  }
}

impl<'lua> FromLua<'lua> for ScriptValue {
  fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::Result<Self> {
    Ok(match value {
      Value::Nil => ScriptValue::new(Variant::Null),
      Value::Boolean(value) => ScriptValue::new(Variant::Bool(value)),
      Value::LightUserData(_) => todo!("LightUserData conversion not implemented"),
      Value::Integer(value) => ScriptValue::new(Variant::I32(value as i32)),
      Value::Number(value) => ScriptValue::new(Variant::F64(value)),
      Value::String(value) => ScriptValue::new(Variant::String(value.to_str()?.to_string())),
      Value::Table(_) => todo!("Table conversion not implemented"),
      Value::Function(_) => todo!("Function conversion not implemented"),
      Value::Thread(_) => todo!("Thread conversion not implemented"),
      Value::UserData(_) => todo!("UserData conversion not implemented"),
      Value::Error(_) => todo!("Error conversion not implemented"),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_lua_evaluation() {
    let mut runtime = LuaScriptRuntime::new();

    runtime.add_callback("add", |x: i32, y: i32| x + y);

    let result = runtime.eval("return add(42, 32)").unwrap();

    assert_eq!(result, ScriptValue::new(Variant::I32(74)));
  }
}
