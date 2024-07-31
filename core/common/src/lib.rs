//! Core components for the Surreal game engine.
//!
//! This crate contains common utilities, collections, diagnostics and other
//! general purpose code for use in other systems.

#![allow(incomplete_features)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(noop_waker)]
#![feature(ptr_as_ref_unchecked)]
#![feature(box_into_inner)]
#![feature(allocator_api)]

pub use abstractions::*;
pub use collections::*;
pub use concurrency::*;
pub use diagnostics::*;
pub use io::*;
pub use maths::*;
pub use memory::*;
pub use strings::*;
pub use utilities::*;

mod abstractions;
mod collections;
mod concurrency;
mod diagnostics;
mod io;
mod maths;
mod memory;
mod strings;
mod utilities;

pub use macros::{profiling, Asset, Deserialize, Reflect, Serialize, Singleton, Trace};

#[cfg(feature = "lua")]
pub mod lua {
  //! Lua support for the engine.
  use mlua::prelude::LuaMultiValue;
  pub use mlua::*;

  use crate::{Callback, Color, Color32, Quat, ToStringName, ToVirtualPath, Variant, Vec2, Vec3, Vec4};

  /// Lua scripting engine.
  pub struct LuaScriptEngine {
    lua: Lua,
  }

  impl LuaScriptEngine {
    /// Creates a new Lua engine.
    pub fn new() -> Result<Self> {
      let lua = Lua::new();

      lua.load_from_std_lib(StdLib::MATH)?;

      Ok(Self { lua })
    }

    /// Runs the given script.
    pub fn run(&self, script: &str) -> Result<()> {
      self.lua.load(script).exec()?;

      Ok(())
    }

    /// Loads the given script.
    pub fn load(&self, path: impl ToVirtualPath) -> Result<()> {
      let script = path
        .to_virtual_path()
        .read_all_text()
        .map_err(|_| Error::RuntimeError("Unable to load script".to_string()))?;

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
    table: Table<'lua>,
  }

  impl<'lua> LuaScriptTable<'lua> {
    /// Gets a value from the table.
    pub fn get<R: FromLua<'lua>>(&self, name: &str) -> Result<R> {
      self.table.get(name)
    }

    /// Gets a table from the table.
    pub fn get_table(&self, name: &str) -> Result<Self> {
      let table = self.table.get::<_, Table>(name)?;
      Ok(LuaScriptTable { lua: self.lua, table })
    }

    /// Sets a value in the table.
    pub fn set<R: IntoLua<'lua>>(&self, name: &str, value: R) -> Result<()> {
      self.table.set(name, value)
    }

    /// Sets a function in the table.
    pub fn set_function<R>(&self, name: &str, callback: impl Callback<R> + 'static) {
      let function_name = name.to_string_name();
      let body = move |lua, args: LuaMultiValue| {
        let args = args
          .into_iter()
          .map(|value| Variant::from_lua(value, lua))
          .collect::<Result<Vec<_>>>()?;

        let result = callback
          .call(&args)
          .map_err(|_| Error::RuntimeError(format!("An error occurred calling {}", &function_name)))?;

        Ok(result.into_lua(lua)?)
      };

      let function = self.lua.create_function(body).unwrap();

      self.table.set(name, function).unwrap();
    }
  }

  /// Allows a [`Variant`] to be converted into Lua state.
  impl<'lua> IntoLua<'lua> for Variant {
    fn into_lua(self, lua: &'lua Lua) -> Result<Value<'lua>> {
      Ok(match self {
        Variant::Null => Value::Nil,
        Variant::Bool(value) => Value::Boolean(value),
        Variant::Char(value) => Value::Integer(value as i64),
        Variant::U8(value) => Value::Integer(value as i64),
        Variant::U16(value) => Value::Integer(value as i64),
        Variant::U32(value) => Value::Integer(value as i64),
        Variant::U64(value) => Value::Integer(value as i64),
        Variant::I8(value) => Value::Integer(value as i64),
        Variant::I16(value) => Value::Integer(value as i64),
        Variant::I32(value) => Value::Integer(value as i64),
        Variant::I64(value) => Value::Integer(value),
        Variant::F32(value) => Value::Number(value as f64),
        Variant::F64(value) => Value::Number(value),
        Variant::String(value) => Value::String(lua.create_string(value)?),
        Variant::StringName(value) => Value::String(lua.create_string(value.to_string())?),
        Variant::Vec2(value) => LuaVec2(value).into_lua(lua)?,
        Variant::Vec3(value) => LuaVec3(value).into_lua(lua)?,
        Variant::Vec4(value) => LuaVec4(value).into_lua(lua)?,
        Variant::Quat(value) => LuaQuat(value).into_lua(lua)?,
        Variant::Color(value) => LuaColor(value).into_lua(lua)?,
        Variant::Color32(value) => LuaColor32(value).into_lua(lua)?,
      })
    }
  }

  /// Allows a [`Variant`] to be converted from Lua state.
  impl<'lua> FromLua<'lua> for Variant {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> Result<Self> {
      Ok(match value {
        Value::Nil => Variant::Null,
        Value::Boolean(value) => Variant::Bool(value),
        Value::LightUserData(_) => todo!(),
        Value::Integer(value) => Variant::I64(value),
        Value::Number(value) => Variant::F64(value),
        Value::String(value) => Variant::String(value.to_str()?.to_string()),
        Value::Table(_) => todo!(),
        Value::Function(_) => todo!(),
        Value::Thread(_) => todo!(),
        Value::UserData(value) => match () {
          _ if value.is::<LuaVec2>() => Variant::Vec2(value.borrow::<LuaVec2>()?.0),
          _ if value.is::<LuaVec3>() => Variant::Vec3(value.borrow::<LuaVec3>()?.0),
          _ if value.is::<LuaVec4>() => Variant::Vec4(value.borrow::<LuaVec4>()?.0),
          _ if value.is::<LuaQuat>() => Variant::Quat(value.borrow::<LuaQuat>()?.0),
          _ if value.is::<LuaColor>() => Variant::Color(value.borrow::<LuaColor>()?.0),
          _ if value.is::<LuaColor32>() => Variant::Color32(value.borrow::<LuaColor32>()?.0),
          _ => return Err(Error::RuntimeError("Unrecognized user data".to_string())),
        },
        _ => panic!("An unsupported Variant kind was provided from Lua"),
      })
    }
  }

  /// A lightweight [`UserData`] wrapper for [`Vec2`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaVec2(Vec2);

  impl UserData for LuaVec2 {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("x", |_, this| Ok(this.0.x));
      fields.add_field_method_get("y", |_, this| Ok(this.0.y));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_method("length", |_, this, ()| Ok(this.0.length()));
      methods.add_method("length_squared", |_, this, ()| Ok(this.0.length_squared()));

      methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }

  /// A lightweight [`UserData`] wrapper for [`Vec3`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaVec3(Vec3);

  impl UserData for LuaVec3 {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("x", |_, this| Ok(this.0.x));
      fields.add_field_method_get("y", |_, this| Ok(this.0.y));
      fields.add_field_method_get("z", |_, this| Ok(this.0.z));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_method("length", |_, this, ()| Ok(this.0.length()));
      methods.add_method("length_squared", |_, this, ()| Ok(this.0.length_squared()));

      methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }

  /// A lightweight [`UserData`] wrapper for [`Vec4`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaVec4(Vec4);

  impl UserData for LuaVec4 {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("x", |_, this| Ok(this.0.x));
      fields.add_field_method_get("y", |_, this| Ok(this.0.y));
      fields.add_field_method_get("z", |_, this| Ok(this.0.z));
      fields.add_field_method_get("w", |_, this| Ok(this.0.w));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_method("length", |_, this, ()| Ok(this.0.length()));
      methods.add_method("length_squared", |_, this, ()| Ok(this.0.length_squared()));

      methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }

  /// A lightweight [`UserData`] wrapper for [`Quat`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaQuat(Quat);

  impl UserData for LuaQuat {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("x", |_, this| Ok(this.0.x));
      fields.add_field_method_get("y", |_, this| Ok(this.0.y));
      fields.add_field_method_get("z", |_, this| Ok(this.0.z));
      fields.add_field_method_get("w", |_, this| Ok(this.0.w));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_method("length", |_, this, ()| Ok(this.0.length()));
      methods.add_method("length_squared", |_, this, ()| Ok(this.0.length_squared()));

      methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }

  /// A lightweight [`UserData`] wrapper for [`Color`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaColor(Color);

  impl UserData for LuaColor {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("r", |_, this| Ok(this.0.r));
      fields.add_field_method_get("g", |_, this| Ok(this.0.g));
      fields.add_field_method_get("b", |_, this| Ok(this.0.b));
      fields.add_field_method_get("a", |_, this| Ok(this.0.a));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }

  /// A lightweight [`UserData`] wrapper for [`Color32`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaColor32(Color32);

  impl UserData for LuaColor32 {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("r", |_, this| Ok(this.0.r));
      fields.add_field_method_get("g", |_, this| Ok(this.0.g));
      fields.add_field_method_get("b", |_, this| Ok(this.0.b));
      fields.add_field_method_get("a", |_, this| Ok(this.0.a));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }
}
