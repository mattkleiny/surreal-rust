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
pub use macros::{profiling, Asset, Deserialize, Reflect, Serialize, Singleton, Trace};
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

#[cfg(feature = "lua")]
pub mod lua {
  //! Lua support for the engine.
  pub use mlua::prelude::*;

  use crate::{Callback, Color, Color32, Quat, ToStringName, ToVirtualPath, Variant, Vec2, Vec3, Vec4};

  /// Lua scripting engine.
  ///
  /// This is a lightweight wrapper around the [`Lua`] state that provides
  /// conveniences for interacting with Lua.
  pub struct LuaScriptEngine {
    lua: Lua,
  }

  impl LuaScriptEngine {
    /// Creates a new Lua engine.
    pub fn new() -> LuaResult<Self> {
      let lua = Lua::new();

      lua.load_from_std_lib(LuaStdLib::MATH)?;

      Ok(Self { lua })
    }

    /// Gets the underlying Lua state.
    pub fn lua(&self) -> &Lua {
      &self.lua
    }

    /// Runs the given script.
    pub fn run(&self, script: &str) -> LuaResult<()> {
      self.lua.load(script).exec()?;

      Ok(())
    }

    /// Loads the given script.
    pub fn load(&self, path: impl ToVirtualPath) -> LuaResult<()> {
      let script = path
        .to_virtual_path()
        .read_all_text()
        .map_err(|_| LuaError::RuntimeError("Unable to load script".to_string()))?;

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

  /// A wrapper over a [`LuaTable`] for simplified access.
  pub struct LuaScriptTable<'lua> {
    lua: &'lua Lua,
    table: LuaTable<'lua>,
  }

  impl<'lua> LuaScriptTable<'lua> {
    /// Gets a value from the table.
    pub fn get<R: FromLua<'lua>>(&self, name: &str) -> LuaResult<R> {
      self.table.get(name)
    }

    /// Sets a value in the table.
    pub fn set<R: IntoLua<'lua>>(&self, name: &str, value: R) -> LuaResult<()> {
      self.table.set(name, value)
    }

    /// Gets a sub-table from the table.
    pub fn get_table(&self, name: &str) -> LuaResult<Self> {
      Ok(LuaScriptTable {
        lua: self.lua,
        table: self.table.get(name)?,
      })
    }

    /// Sets a function in the table.
    pub fn set_function<R>(&self, name: &str, callback: impl Callback<R> + 'static) -> LuaResult<()> {
      let function_name = name.to_string_name();
      let body = move |lua, args: LuaMultiValue| {
        let args = args
          .into_iter()
          .map(|value| Variant::from_lua(value, lua))
          .collect::<LuaResult<Vec<_>>>()?;

        let result = callback.call(&args).map_err(|error| {
          // make it clear which function caused the error
          LuaError::RuntimeError(format!("An error occurred calling {}, {:?}", &function_name, error))
        })?;

        Ok(result.into_lua(lua)?)
      };

      self.table.set(name, self.lua.create_function(body)?)?;

      Ok(())
    }
  }

  /// Allows a [`Variant`] to be converted into Lua state.
  impl<'lua> IntoLua<'lua> for Variant {
    fn into_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
      Ok(match self {
        Variant::Null => LuaValue::Nil,
        Variant::Bool(value) => LuaValue::Boolean(value),
        Variant::Char(value) => LuaValue::Integer(value as i64),
        Variant::U8(value) => LuaValue::Integer(value as i64),
        Variant::U16(value) => LuaValue::Integer(value as i64),
        Variant::U32(value) => LuaValue::Integer(value as i64),
        Variant::U64(value) => LuaValue::Integer(value as i64),
        Variant::I8(value) => LuaValue::Integer(value as i64),
        Variant::I16(value) => LuaValue::Integer(value as i64),
        Variant::I32(value) => LuaValue::Integer(value as i64),
        Variant::I64(value) => LuaValue::Integer(value),
        Variant::F32(value) => LuaValue::Number(value as f64),
        Variant::F64(value) => LuaValue::Number(value),
        Variant::String(value) => LuaValue::String(lua.create_string(value)?),
        Variant::StringName(value) => LuaValue::String(lua.create_string(value.to_string())?),
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
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
      Ok(match value {
        LuaValue::Nil => Variant::Null,
        LuaValue::Boolean(value) => Variant::Bool(value),
        LuaValue::LightUserData(_) => todo!(),
        LuaValue::Integer(value) => Variant::I64(value),
        LuaValue::Number(value) => Variant::F64(value),
        LuaValue::String(value) => Variant::String(value.to_str()?.to_string()),
        LuaValue::Table(_) => todo!(),
        LuaValue::Function(_) => todo!(),
        LuaValue::Thread(_) => todo!(),
        LuaValue::UserData(value) => match () {
          _ if value.is::<LuaVec2>() => Variant::Vec2(value.borrow::<LuaVec2>()?.0),
          _ if value.is::<LuaVec3>() => Variant::Vec3(value.borrow::<LuaVec3>()?.0),
          _ if value.is::<LuaVec4>() => Variant::Vec4(value.borrow::<LuaVec4>()?.0),
          _ if value.is::<LuaQuat>() => Variant::Quat(value.borrow::<LuaQuat>()?.0),
          _ if value.is::<LuaColor>() => Variant::Color(value.borrow::<LuaColor>()?.0),
          _ if value.is::<LuaColor32>() => Variant::Color32(value.borrow::<LuaColor32>()?.0),
          _ => return Err(LuaError::RuntimeError("Unrecognized user data".to_string())),
        },
        _ => panic!("An unsupported Variant kind was provided from Lua"),
      })
    }
  }

  /// A lightweight [`LuaUserData`] wrapper for [`Vec2`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaVec2(Vec2);

  impl LuaUserData for LuaVec2 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("x", |_, this| Ok(this.0.x));
      fields.add_field_method_get("y", |_, this| Ok(this.0.y));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_method("length", |_, this, ()| Ok(this.0.length()));
      methods.add_method("length_squared", |_, this, ()| Ok(this.0.length_squared()));

      methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }

  /// A lightweight [`LuaUserData`] wrapper for [`Vec3`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaVec3(Vec3);

  impl LuaUserData for LuaVec3 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("x", |_, this| Ok(this.0.x));
      fields.add_field_method_get("y", |_, this| Ok(this.0.y));
      fields.add_field_method_get("z", |_, this| Ok(this.0.z));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_method("length", |_, this, ()| Ok(this.0.length()));
      methods.add_method("length_squared", |_, this, ()| Ok(this.0.length_squared()));

      methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }

  /// A lightweight [`LuaUserData`] wrapper for [`Vec4`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaVec4(Vec4);

  impl LuaUserData for LuaVec4 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("x", |_, this| Ok(this.0.x));
      fields.add_field_method_get("y", |_, this| Ok(this.0.y));
      fields.add_field_method_get("z", |_, this| Ok(this.0.z));
      fields.add_field_method_get("w", |_, this| Ok(this.0.w));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_method("length", |_, this, ()| Ok(this.0.length()));
      methods.add_method("length_squared", |_, this, ()| Ok(this.0.length_squared()));

      methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }

  /// A lightweight [`LuaUserData`] wrapper for [`Quat`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaQuat(Quat);

  impl LuaUserData for LuaQuat {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("x", |_, this| Ok(this.0.x));
      fields.add_field_method_get("y", |_, this| Ok(this.0.y));
      fields.add_field_method_get("z", |_, this| Ok(this.0.z));
      fields.add_field_method_get("w", |_, this| Ok(this.0.w));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_method("length", |_, this, ()| Ok(this.0.length()));
      methods.add_method("length_squared", |_, this, ()| Ok(this.0.length_squared()));

      methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }

  /// A lightweight [`LuaUserData`] wrapper for [`Color`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaColor(Color);

  impl LuaUserData for LuaColor {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("r", |_, this| Ok(this.0.r));
      fields.add_field_method_get("g", |_, this| Ok(this.0.g));
      fields.add_field_method_get("b", |_, this| Ok(this.0.b));
      fields.add_field_method_get("a", |_, this| Ok(this.0.a));

      fields.add_field_method_set("r", |_, this, value: f32| {
        this.0.r = value;
        Ok(())
      });
      fields.add_field_method_set("g", |_, this, value: f32| {
        this.0.g = value;
        Ok(())
      });
      fields.add_field_method_set("b", |_, this, value: f32| {
        this.0.b = value;
        Ok(())
      });
      fields.add_field_method_set("a", |_, this, value: f32| {
        this.0.a = value;
        Ok(())
      });
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }

  /// A lightweight [`LuaUserData`] wrapper for [`Color32`].
  #[repr(transparent)]
  #[derive(Debug, Copy, Clone)]
  struct LuaColor32(Color32);

  impl LuaUserData for LuaColor32 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
      fields.add_field_method_get("r", |_, this| Ok(this.0.r));
      fields.add_field_method_get("g", |_, this| Ok(this.0.g));
      fields.add_field_method_get("b", |_, this| Ok(this.0.b));
      fields.add_field_method_get("a", |_, this| Ok(this.0.a));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
      methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{:?}", this.0)));
    }
  }
}
