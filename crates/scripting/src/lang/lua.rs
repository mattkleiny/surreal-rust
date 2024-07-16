//! Lua scripting language support

use common::Variant;
use mlua::{FromLua, Lua, StdLib, Value};

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

  fn eval_as<R: FromScriptValue>(&self, code: &str) -> Result<R, ScriptError> {
    self
      .lua
      .load(code)
      .eval()
      .map(|it| R::from_script_value(&it))
      .map_err(|it| ScriptError::ExecutionError(it.to_string()))
  }

  fn add_callback<F>(&mut self, _name: &str, _callback: impl ScriptCallback<F> + 'static) {
    todo!()
  }
}

impl<'lua> FromLua<'lua> for ScriptValue {
  fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::Result<Self> {
    Ok(match value {
      Value::Nil => ScriptValue::from(Variant::Null),
      Value::Boolean(value) => ScriptValue::from(Variant::Bool(value)),
      Value::LightUserData(_) => todo!("LightUserData conversion not implemented"),
      Value::Integer(value) => ScriptValue::from(Variant::I32(value as i32)),
      Value::Number(value) => ScriptValue::from(Variant::F64(value)),
      Value::String(value) => ScriptValue::from(Variant::String(value.to_str()?.to_string())),
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
    let runtime = LuaScriptRuntime::new();

    let result = runtime.eval("return 42").unwrap();

    assert_eq!(result, ScriptValue::from(Variant::I32(42)));
  }
}
