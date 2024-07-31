//! Lua support for Surreal

use common::{ToVirtualPath, Variant};
pub use mlua::prelude::*;
use mlua::{Error, MultiValue, StdLib, Value};

use crate::{runtime::ScriptValue, Callback, ScriptValueMulti, ToScriptValue};

/// Possible errors when interacting with Lua.
#[derive(Debug)]
pub enum LuaError {
  ScriptError,
  ExecutionError(Error),
}

/// Lua scripting engine.
pub struct LuaScriptEngine {
  lua: Lua,
}

impl LuaScriptEngine {
  /// Creates a new Lua engine.
  pub fn new() -> Result<Self, LuaError> {
    let lua = Lua::new();
    lua.load_from_std_lib(StdLib::ALL_SAFE)?;

    Ok(Self { lua })
  }

  /// Runs the given script.
  pub fn run(&mut self, script: &str) -> Result<(), LuaError> {
    self.lua.load(script).exec()?;

    Ok(())
  }

  /// Loads the given script.
  pub fn load(&mut self, path: impl ToVirtualPath) -> Result<(), LuaError> {
    let script = path
      .to_virtual_path()
      .read_all_text()
      .map_err(|_| LuaError::ScriptError)?;

    self.run(&script)?;

    Ok(())
  }

  /// Gets the global table from the Lua state.
  pub fn globals(&self) -> LuaScriptTable {
    LuaScriptTable {
      lua: &self.lua,
      table: self.lua.globals(),
    }
  }
}

/// A wrapper over a lua table for simplified access.
pub struct LuaScriptTable<'lua> {
  lua: &'lua Lua,
  table: LuaTable<'lua>,
}

impl<'lua> LuaScriptTable<'lua> {
  /// Gets a value from the table.
  pub fn get<R: FromLua<'lua>>(&self, name: &str) -> Result<R, mlua::prelude::LuaError> {
    self.table.get(name)
  }

  /// Gets a table from the table.
  pub fn get_table(&self, name: &str) -> Result<Self, mlua::prelude::LuaError> {
    let table = self.table.get::<_, LuaTable>(name)?;
    Ok(LuaScriptTable { lua: self.lua, table })
  }

  /// Sets a value in the table.
  pub fn set<R: IntoLua<'lua>>(&self, name: &str, value: R) -> Result<(), mlua::prelude::LuaError> {
    self.table.set(name, value)
  }

  /// Sets a function in the table.
  pub fn set_function<R>(&self, name: &str, callback: impl Callback<R> + 'static) {
    let body = move |lua, args: ScriptValueMulti| {
      let result = callback
        .call(&args.0)
        .map_err(|_| Error::RuntimeError("It didn't work".to_string()))?;

      Ok(result.into_lua(lua)?)
    };

    let function = self.lua.create_function(body).unwrap();

    self.table.set(name, function).unwrap();
  }
}

/// Allows a `ScriptValue` to be converted to Lua.
impl<'lua> IntoLua<'lua> for &ScriptValue {
  fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
    Ok(match &self.0 {
      Variant::Null => Value::Nil,
      Variant::Bool(value) => Value::Boolean(*value),
      Variant::Char(_) => todo!(),
      Variant::U8(_) => todo!(),
      Variant::U16(_) => todo!(),
      Variant::U32(_) => todo!(),
      Variant::U64(_) => todo!(),
      Variant::I8(_) => todo!(),
      Variant::I16(_) => todo!(),
      Variant::I32(_) => todo!(),
      Variant::I64(_) => todo!(),
      Variant::F32(_) => todo!(),
      Variant::F64(_) => todo!(),
      Variant::String(value) => Value::String(lua.create_string(value)?),
      Variant::StringName(value) => Value::String(lua.create_string(value.to_string())?),
      Variant::Vec2(_) => todo!(),
      Variant::Vec3(_) => todo!(),
      Variant::Vec4(_) => todo!(),
      Variant::Quat(_) => todo!(),
      Variant::Color(_) => todo!(),
      Variant::Color32(_) => todo!(),
    })
  }
}

/// Allows a `ScriptValue` to be converted from Lua.
impl<'lua> FromLua<'lua> for ScriptValue {
  fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> mlua::Result<Self> {
    Ok(match value {
      Value::Nil => ScriptValue(Variant::Null),
      Value::Boolean(value) => ScriptValue(Variant::Bool(value)),
      Value::LightUserData(_) => todo!(),
      Value::Integer(value) => ScriptValue(Variant::I64(value)),
      Value::Number(value) => ScriptValue(Variant::F64(value)),
      Value::String(value) => ScriptValue(Variant::String(value.to_str()?.to_string())),
      Value::Table(_) => todo!(),
      Value::Function(_) => todo!(),
      Value::Thread(_) => todo!(),
      Value::UserData(_) => todo!(),
      Value::Error(_) => todo!(),
      _ => todo!(),
    })
  }
}

/// Allows a `ScriptValueMulti` to be converted from Lua.
impl<'lua> FromLuaMulti<'lua> for ScriptValueMulti {
  fn from_lua_multi(values: LuaMultiValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
    let mut result = Vec::new();

    for value in values {
      result.push(ScriptValue::from_lua(value, lua)?);
    }

    Ok(ScriptValueMulti(result))
  }
}

/// Allows a `ScriptValueMulti` to be converted to Lua.
impl<'lua> IntoLuaMulti<'lua> for ScriptValueMulti {
  fn into_lua_multi(self, lua: &'lua Lua) -> LuaResult<LuaMultiValue<'lua>> {
    let mut result = Vec::new();

    for value in self.0 {
      result.push(value.into_lua(lua)?);
    }

    Ok(LuaMultiValue::from_vec(result))
  }
}

impl From<Error> for LuaError {
  #[inline(always)]
  fn from(error: Error) -> Self {
    Self::ExecutionError(error)
  }
}
