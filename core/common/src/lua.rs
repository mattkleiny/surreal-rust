//! Directly integrated Lua support for the engine.
//!
//! Lua is a pretty solid scripting language that is easy to embed and use, so
//! let's make it a first class citizen.

pub use mlua::prelude::*;

use crate::{
  Callable, Callback, CallbackError, Color, Color32, FromVariant, Pointer, Quat, ToStringName, ToVariant,
  ToVirtualPath, Variant, Vec2, Vec3, Vec4,
};

/// A Lua scripting engine.
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
    // build a closure that can be called from Lua
    let function_name = name.to_string_name(); // pool string names

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
      Variant::Callable(callable) => {
        // create a Lua function that calls the callable
        // TODO: clean this up?
        let function = lua.create_function(move |lua, args: LuaMultiValue| {
          let args = args
            .into_iter()
            .map(|value| Variant::from_lua(value, lua))
            .collect::<LuaResult<Vec<_>>>()?;

          let result = callable
            .call(&args)
            .map_err(|error| LuaError::RuntimeError(format!("An error occurred calling a function, {:?}", error)))?;

          Ok(result.into_lua(lua)?)
        })?;

        LuaValue::Function(function)
      }
      Variant::UserData(value) => LuaValue::LightUserData(LuaLightUserData(value.into_void())),
    })
  }
}

/// Allows a [`Variant`] to be converted from Lua state.
impl<'lua> FromLua<'lua> for Variant {
  fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
    Ok(match value {
      LuaValue::Nil => Variant::Null,
      LuaValue::Boolean(value) => Variant::Bool(value),
      LuaValue::Integer(value) => Variant::I64(value),
      LuaValue::Number(value) => Variant::F64(value),
      LuaValue::String(value) => Variant::String(value.to_str()?.to_string()),
      LuaValue::Table(value) => Variant::UserData(Pointer::new(value.into_owned())),
      LuaValue::Function(function) => {
        // create a callable that calls the Lua function
        let function = function.into_owned();
        let callable = Callable::new(move |args| {
          function
            .call(args)
            .map_err(|error| CallbackError::ExecutionError(error.to_string()))
        });

        Variant::Callable(callable)
      }
      LuaValue::LightUserData(value) => Variant::UserData(Pointer::from_raw_mut(value.0)),
      LuaValue::UserData(value) => match () {
        _ if value.is::<LuaVec2>() => Variant::Vec2(value.borrow::<LuaVec2>()?.0),
        _ if value.is::<LuaVec3>() => Variant::Vec3(value.borrow::<LuaVec3>()?.0),
        _ if value.is::<LuaVec4>() => Variant::Vec4(value.borrow::<LuaVec4>()?.0),
        _ if value.is::<LuaQuat>() => Variant::Quat(value.borrow::<LuaQuat>()?.0),
        _ if value.is::<LuaColor>() => Variant::Color(value.borrow::<LuaColor>()?.0),
        _ if value.is::<LuaColor32>() => Variant::Color32(value.borrow::<LuaColor32>()?.0),
        _ => return Err(LuaError::RuntimeError("Unrecognized user data".to_string())),
      },
      _ => Err(LuaError::RuntimeError("Unsupported Lua value".to_string()))?,
    })
  }
}

/// Extension methods for [`LuaTable`] to work with [`Variant`]s.
pub trait VariantTableExt<'lua> {
  fn get_as<R: FromVariant>(&self, key: impl IntoLua<'lua>) -> LuaResult<R>;
  fn set_as<R: ToVariant>(&self, key: impl IntoLua<'lua>, value: R) -> LuaResult<()>;
}

impl<'lua> VariantTableExt<'lua> for LuaTable<'lua> {
  #[inline]
  fn get_as<R: FromVariant>(&self, key: impl IntoLua<'lua>) -> LuaResult<R> {
    let variant = self.get(key);

    variant.and_then(|value| R::from_variant(value).map_err(|_| LuaError::UserDataTypeMismatch))
  }

  #[inline]
  fn set_as<R: ToVariant>(&self, key: impl IntoLua<'lua>, value: R) -> LuaResult<()> {
    let variant = value.to_variant();

    self.set(key, variant)
  }
}

/// A helper for adding get/set methods for a field.
macro_rules! impl_get_set {
  ($fields:expr, $name:tt) => {
    $fields.add_field_method_get(stringify!($name), |_, this| Ok(this.0.$name));
    $fields.add_field_method_set(stringify!($name), |_, this, value| {
      this.0.$name = value;
      Ok(())
    });
  };
}

/// A helper for adding a named method.
macro_rules! impl_method {
  ($methods:expr, $name:tt) => {
    $methods.add_method(stringify!($name), |_, this, ()| Ok(this.0.$name()));
  };
}

/// A lightweight [`LuaUserData`] wrapper for [`Vec2`].
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
struct LuaVec2(Vec2);

impl LuaUserData for LuaVec2 {
  fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    impl_get_set!(fields, x);
    impl_get_set!(fields, y);
  }

  fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("length", |_, this, ()| Ok(this.0.length()));
    methods.add_method("length_squared", |_, this, ()| Ok(this.0.length_squared()));

    methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{}", this.0)));
  }
}

/// A lightweight [`LuaUserData`] wrapper for [`Vec3`].
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
struct LuaVec3(Vec3);

impl LuaUserData for LuaVec3 {
  fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    impl_get_set!(fields, x);
    impl_get_set!(fields, y);
    impl_get_set!(fields, z);
  }

  fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
    impl_method!(methods, length);
    impl_method!(methods, length_squared);

    methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{}", this.0)));
  }
}

/// A lightweight [`LuaUserData`] wrapper for [`Vec4`].
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
struct LuaVec4(Vec4);

impl LuaUserData for LuaVec4 {
  fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    impl_get_set!(fields, x);
    impl_get_set!(fields, y);
    impl_get_set!(fields, z);
    impl_get_set!(fields, w);
  }

  fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
    impl_method!(methods, length);
    impl_method!(methods, length_squared);

    methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{}", this.0)));
  }
}

/// A lightweight [`LuaUserData`] wrapper for [`Quat`].
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
struct LuaQuat(Quat);

impl LuaUserData for LuaQuat {
  fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    impl_get_set!(fields, x);
    impl_get_set!(fields, y);
    impl_get_set!(fields, z);
    impl_get_set!(fields, w);
  }

  fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
    impl_method!(methods, length);
    impl_method!(methods, length_squared);

    methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{}", this.0)));
  }
}

/// A lightweight [`LuaUserData`] wrapper for [`Color`].
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
struct LuaColor(Color);

impl LuaUserData for LuaColor {
  fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    impl_get_set!(fields, r);
    impl_get_set!(fields, g);
    impl_get_set!(fields, b);
    impl_get_set!(fields, a);
  }

  fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{}", this.0)));
  }
}

/// A lightweight [`LuaUserData`] wrapper for [`Color32`].
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
struct LuaColor32(Color32);

impl LuaUserData for LuaColor32 {
  fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    impl_get_set!(fields, r);
    impl_get_set!(fields, g);
    impl_get_set!(fields, b);
    impl_get_set!(fields, a);
  }

  fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| Ok(format!("{}", this.0)));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_table_coercion_through_pointer() {
    let lua = Lua::new();
    let table = lua.globals();

    table.set("Key", "Hello, world!").unwrap();

    let pointer_a = Pointer::new(table);
    let pointer_b: Pointer<LuaTable> = unsafe { pointer_a.cast_unchecked() };

    let value: String = pointer_b.get("Key").unwrap();

    assert_eq!(value, "Hello, world!");

    pointer_b.delete();
  }
}
