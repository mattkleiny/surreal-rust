//! Directly integrated Lua support for the engine.
//!
//! Lua is a pretty solid scripting language that is easy to embed and use, so
//! let's make it a first class citizen.

pub use mlua::prelude::*;

use crate::{
  Callable, Callback, CallbackError, Color, Color32, FromVariant, Pointer, Quat, ToVariant, ToVirtualPath, Variant,
  Vec2, Vec3, Vec4,
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
    let engine = Self { lua: Lua::new() };

    // load the standard library
    engine.lua.load_from_std_lib(LuaStdLib::MATH)?;
    engine.lua.load_from_std_lib(LuaStdLib::STRING)?;

    {
      // configure common globals
      let globals = engine.globals();

      globals.set_callback("vec2", Vec2::new)?;
      globals.set_callback("vec3", Vec3::new)?;
      globals.set_callback("vec4", Vec4::new)?;
      globals.set_callback("quat", Quat::from_xyzw)?;
      globals.set_callback("rgb", Color::rgb)?;
      globals.set_callback("rgba", Color::rgba)?;
    }

    Ok(engine)
  }

  /// Gets the underlying Lua state.
  pub fn lua(&self) -> &Lua {
    &self.lua
  }

  /// Loads the given script and executes it.
  pub fn load_exec(&self, path: impl ToVirtualPath) -> LuaResult<()> {
    let script = path
      .to_virtual_path()
      .read_all_text()
      .map_err(|_| LuaError::RuntimeError("Unable to load script".to_string()))?;

    self.exec(&script)
  }

  /// Loads the given script and evaluates it.
  pub fn load_eval<R: for<'lua> FromLua<'lua>>(&self, path: impl ToVirtualPath) -> LuaResult<R> {
    let script = path
      .to_virtual_path()
      .read_all_text()
      .map_err(|_| LuaError::RuntimeError("Unable to load script".to_string()))?;

    self.eval(&script)
  }

  /// Evaluates the given script.
  pub fn eval<R: for<'lua> FromLua<'lua>>(&self, script: &str) -> LuaResult<R> {
    self.lua.load(script).eval()
  }

  /// Executes the given script.
  pub fn exec(&self, script: &str) -> LuaResult<()> {
    self.lua.load(script).exec()
  }

  /// Gets the global table from the Lua state.
  pub fn globals(&self) -> LuaTable {
    self.lua.globals()
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
        let function = lua.create_function(move |lua, args: LuaMultiValue| {
          let args = args
            .into_iter()
            .map(|value| Variant::from_lua(value, lua))
            .collect::<LuaResult<Vec<_>>>()?;

          let result = callable
            .call(&args)
            .map_err(|error| LuaError::RuntimeError(error.to_string()))?;

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
            .call(VariantArray(args.to_vec()))
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

/// Wraps many [`Variant`]s to be passed as arguments to a Lua function.
///
/// This is necessary because Lua does not support variadic arguments.
struct VariantArray(Vec<Variant>);

impl<'lua> IntoLuaMulti<'lua> for VariantArray {
  fn into_lua_multi(self, lua: &'lua Lua) -> LuaResult<LuaMultiValue<'lua>> {
    self.0.into_iter().map(|value| value.into_lua(lua)).collect()
  }
}

/// Extension methods for [`LuaTable`] to work with [`Variant`]s.
pub trait VariantTableExt<'lua> {
  fn get_variant<R: FromVariant>(&self, key: impl IntoLua<'lua>) -> LuaResult<R>;
  fn set_variant<R: ToVariant>(&self, key: impl IntoLua<'lua>, value: R) -> LuaResult<()>;

  /// Calls a [`Callable`] in the table.
  fn call_callback(&self, key: impl IntoLua<'lua>, args: &[Variant]) -> LuaResult<Variant> {
    let callable: Callable = self.get_variant(key)?;

    callable.call(args).map_err(|it| LuaError::RuntimeError(it.to_string()))
  }

  /// Sets a [`Callback`] in the table by allocating a [`Callable`].
  fn set_callback<R>(&self, key: impl IntoLua<'lua>, callback: impl Callback<R> + 'static) -> LuaResult<()> {
    let callable = Callable::from_callback(callback);

    self.set_variant(key, callable)
  }
}

impl<'lua> VariantTableExt<'lua> for LuaTable<'lua> {
  #[inline]
  fn get_variant<R: FromVariant>(&self, key: impl IntoLua<'lua>) -> LuaResult<R> {
    let variant = self.get(key);

    variant.and_then(|value| R::from_variant(value).map_err(|_| LuaError::UserDataTypeMismatch))
  }

  #[inline]
  fn set_variant<R: ToVariant>(&self, key: impl IntoLua<'lua>, value: R) -> LuaResult<()> {
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
    use LuaMetaMethod::*;

    methods.add_method("length", |_, this, ()| Ok(this.0.length()));
    methods.add_method("length_squared", |_, this, ()| Ok(this.0.length_squared()));

    methods.add_meta_method(ToString, |_, this, ()| Ok(format!("{}", this.0)));
    methods.add_meta_method(Add, |_, this, other: LuaVec2| Ok(LuaVec2(this.0 + other.0)));
    methods.add_meta_method(Sub, |_, this, other: LuaVec2| Ok(LuaVec2(this.0 - other.0)));
    methods.add_meta_method(Mul, |_, this, other: f32| Ok(LuaVec2(this.0 * other)));
    methods.add_meta_method(Div, |_, this, other: f32| Ok(LuaVec2(this.0 / other)));
  }
}

impl<'lua> FromLua<'lua> for LuaVec2 {
  fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
    match value {
      LuaValue::Table(value) => {
        let x = value.get("x")?;
        let y = value.get("y")?;

        Ok(LuaVec2(Vec2::new(x, y)))
      }
      LuaValue::UserData(value) => {
        let value = value.borrow::<LuaVec2>()?;

        Ok(*value)
      }
      _ => Err(LuaError::RuntimeError("Unsupported Lua value".to_string())),
    }
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
    use LuaMetaMethod::*;

    impl_method!(methods, length);
    impl_method!(methods, length_squared);

    methods.add_meta_method(ToString, |_, this, ()| Ok(format!("{}", this.0)));
    methods.add_meta_method(Add, |_, this, other: LuaVec3| Ok(LuaVec3(this.0 + other.0)));
    methods.add_meta_method(Sub, |_, this, other: LuaVec3| Ok(LuaVec3(this.0 - other.0)));
    methods.add_meta_method(Mul, |_, this, other: f32| Ok(LuaVec3(this.0 * other)));
    methods.add_meta_method(Div, |_, this, other: f32| Ok(LuaVec3(this.0 / other)));
  }
}

impl<'lua> FromLua<'lua> for LuaVec3 {
  fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
    match value {
      LuaValue::Table(value) => {
        let x = value.get("x")?;
        let y = value.get("y")?;
        let z = value.get("z")?;

        Ok(LuaVec3(Vec3::new(x, y, z)))
      }
      LuaValue::UserData(value) => {
        let value = value.borrow::<LuaVec3>()?;

        Ok(*value)
      }
      _ => Err(LuaError::RuntimeError("Unsupported Lua value".to_string())),
    }
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
    use LuaMetaMethod::*;

    impl_method!(methods, length);
    impl_method!(methods, length_squared);

    methods.add_meta_method(ToString, |_, this, ()| Ok(format!("{}", this.0)));
    methods.add_meta_method(Add, |_, this, other: LuaVec4| Ok(LuaVec4(this.0 + other.0)));
    methods.add_meta_method(Sub, |_, this, other: LuaVec4| Ok(LuaVec4(this.0 - other.0)));
    methods.add_meta_method(Mul, |_, this, other: f32| Ok(LuaVec4(this.0 * other)));
    methods.add_meta_method(Div, |_, this, other: f32| Ok(LuaVec4(this.0 / other)));
  }
}

impl<'lua> FromLua<'lua> for LuaVec4 {
  fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
    match value {
      LuaValue::Table(value) => {
        let x = value.get("x")?;
        let y = value.get("y")?;
        let z = value.get("z")?;
        let w = value.get("w")?;

        Ok(LuaVec4(Vec4::new(x, y, z, w)))
      }
      LuaValue::UserData(value) => {
        let value = value.borrow::<LuaVec4>()?;

        Ok(*value)
      }
      _ => Err(LuaError::RuntimeError("Unsupported Lua value".to_string())),
    }
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
    use LuaMetaMethod::*;

    impl_method!(methods, length);
    impl_method!(methods, length_squared);

    methods.add_meta_method(ToString, |_, this, ()| Ok(format!("{}", this.0)));
    methods.add_meta_method(Add, |_, this, other: LuaQuat| Ok(LuaQuat(this.0 + other.0)));
    methods.add_meta_method(Sub, |_, this, other: LuaQuat| Ok(LuaQuat(this.0 - other.0)));
    methods.add_meta_method(Mul, |_, this, other: f32| Ok(LuaQuat(this.0 * other)));
    methods.add_meta_method(Div, |_, this, other: f32| Ok(LuaQuat(this.0 / other)));
  }
}

impl<'lua> FromLua<'lua> for LuaQuat {
  fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
    match value {
      LuaValue::Table(value) => {
        let x = value.get("x")?;
        let y = value.get("y")?;
        let z = value.get("z")?;
        let w = value.get("w")?;

        Ok(LuaQuat(Quat::from_xyzw(x, y, z, w)))
      }
      LuaValue::UserData(value) => {
        let value = value.borrow::<LuaQuat>()?;

        Ok(*value)
      }
      _ => Err(LuaError::RuntimeError("Unsupported Lua value".to_string())),
    }
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
    use LuaMetaMethod::*;

    methods.add_meta_method(ToString, |_, this, ()| Ok(format!("{}", this.0)));
    methods.add_meta_method(Add, |_, this, other: LuaColor| Ok(LuaColor(this.0 + other.0)));
    methods.add_meta_method(Sub, |_, this, other: LuaColor| Ok(LuaColor(this.0 - other.0)));
    methods.add_meta_method(Mul, |_, this, other: f32| Ok(LuaColor(this.0 * other)));
    methods.add_meta_method(Div, |_, this, other: f32| Ok(LuaColor(this.0 / other)));
  }
}

impl<'lua> FromLua<'lua> for LuaColor {
  fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
    match value {
      LuaValue::Table(value) => {
        let r = value.get("r")?;
        let g = value.get("g")?;
        let b = value.get("b")?;
        let a = value.get("a").unwrap_or(1.0);

        Ok(LuaColor(Color::rgba(r, g, b, a)))
      }
      LuaValue::UserData(value) => {
        let value = value.borrow::<LuaColor>()?;

        Ok(*value)
      }
      _ => Err(LuaError::RuntimeError("Unsupported Lua value".to_string())),
    }
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
    use LuaMetaMethod::*;

    methods.add_meta_method(ToString, |_, this, ()| Ok(format!("{}", this.0)));
    methods.add_meta_method(Add, |_, this, other: LuaColor32| Ok(LuaColor32(this.0 + other.0)));
    methods.add_meta_method(Sub, |_, this, other: LuaColor32| Ok(LuaColor32(this.0 - other.0)));
  }
}

impl<'lua> FromLua<'lua> for LuaColor32 {
  fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
    match value {
      LuaValue::Table(value) => {
        let r = value.get("r")?;
        let g = value.get("g")?;
        let b = value.get("b")?;
        let a = value.get("a").unwrap_or(255);

        Ok(LuaColor32(Color32::rgba(r, g, b, a)))
      }
      LuaValue::UserData(value) => {
        let value = value.borrow::<LuaColor32>()?;

        Ok(*value)
      }
      _ => Err(LuaError::RuntimeError("Unsupported Lua value".to_string())),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_table_coercion_through_pointer() {
    let lua = Lua::new();
    let globals = lua.globals();

    globals.set("Key", "Hello, world!").unwrap();

    let pointer_a = Pointer::new(globals);
    let pointer_b: Pointer<LuaTable> = unsafe { pointer_a.cast_unchecked() };

    let value: String = pointer_b.get("Key").unwrap();

    assert_eq!(value, "Hello, world!");

    pointer_b.delete();
  }

  #[test]
  fn test_basic_wrapper_operations_for_vec2() {
    let lua = LuaScriptEngine::new().unwrap();
    let globals = lua.globals();

    globals.set_callback("vec2", Vec2::new).unwrap();

    let script = r#"
      local a = vec2(1, 2)
      local b = vec2(3, 4)
      local c = a + b
      local d = a * 2
      local e = a / 2
      local f = a:length()
      local g = a:length_squared()
      local h = tostring(a)
      
      assert(c.x == 4 and c.y == 6)
      assert(d.x == 2 and d.y == 4)
    "#;

    lua.exec(script).unwrap();
  }

  #[test]
  fn test_basic_wrapper_operations_for_vec3() {
    let lua = LuaScriptEngine::new().unwrap();
    let globals = lua.globals();

    globals.set_callback("vec3", Vec3::new).unwrap();

    let script = r#"
      local a = vec3(1, 2, 3)
      local b = vec3(4, 5, 6)
      local c = a + b
      local d = a * 2
      local e = a / 2
      local f = a:length()
      local g = a:length_squared()
      local h = tostring(a)
      
      assert(c.x == 5 and c.y == 7 and c.z == 9)
      assert(d.x == 2 and d.y == 4 and d.z == 6)
    "#;

    lua.exec(script).unwrap();
  }

  #[test]
  fn test_basic_wrapper_operations_for_vec4() {
    let lua = LuaScriptEngine::new().unwrap();
    let globals = lua.globals();

    globals.set_callback("vec4", Vec4::new).unwrap();

    let script = r#"
      local a = vec4(1, 2, 3, 4)
      local b = vec4(5, 6, 7, 8)
      local c = a + b
      local d = a * 2
      local e = a / 2
      local f = a:length()
      local g = a:length_squared()
      local h = tostring(a)
      
      assert(c.x == 6 and c.y == 8 and c.z == 10 and c.w == 12)
      assert(d.x == 2 and d.y == 4 and d.z == 6 and d.w == 8)
    "#;

    lua.exec(script).unwrap();
  }

  #[test]
  fn test_basic_wrapper_operations_for_quat() {
    let lua = LuaScriptEngine::new().unwrap();
    let globals = lua.globals();

    globals.set_callback("quat", Quat::from_xyzw).unwrap();

    let script = r#"
      local a = quat(1, 2, 3, 4)
      local b = quat(5, 6, 7, 8)
      local c = a + b
      local d = a * 2
      local e = a / 2
      local f = a:length()
      local g = a:length_squared()
      local h = tostring(a)
      
      assert(c.x == 6 and c.y == 8 and c.z == 10 and c.w == 12)
      assert(d.x == 2 and d.y == 4 and d.z == 6 and d.w == 8)
    "#;

    lua.exec(script).unwrap();
  }

  #[test]
  fn test_basic_wrapper_operations_for_color() {
    let lua = LuaScriptEngine::new().unwrap();
    let globals = lua.globals();

    globals.set_callback("color", Color::rgba).unwrap();

    let script = r#"
      local a = color(1, 2, 3, 4)
      local b = color(5, 6, 7, 8)
      local c = a + b
      local d = a * 2
      local e = a / 2
      local f = tostring(a)
      
      assert(c.r == 6 and c.g == 8 and c.b == 10 and c.a == 12)
      assert(d.r == 2 and d.g == 4 and d.b == 6 and d.a == 8)
    "#;

    lua.exec(script).unwrap();
  }

  #[test]
  fn test_basic_wrapper_operations_for_color32() {
    let lua = LuaScriptEngine::new().unwrap();
    let globals = lua.globals();

    globals.set_callback("color32", Color32::rgba).unwrap();

    let script = r#"
      local a = color32(1, 2, 3, 4)
      local b = color32(5, 6, 7, 8)
      local c = a + b
      
      assert(c.r == 6 and c.g == 8 and c.b == 10 and c.a == 12)
    "#;

    lua.exec(script).unwrap();
  }

  #[test]
  fn test_basic_call_from_lua() {
    let lua = LuaScriptEngine::new().unwrap();
    let globals = lua.globals();

    globals.set_callback("add", |a: i32, b: i32| a + b).unwrap();

    let script = r#"
      local a = add(1, 2)
      
      assert(a == 3)
    "#;

    lua.exec(script).unwrap();
  }

  #[test]
  fn test_basic_call_from_rust() {
    let lua = LuaScriptEngine::new().unwrap();

    let script = r#"
      function add(a, b)
        return a + b
      end
    "#;

    lua.exec(script).unwrap();

    let globals = lua.globals();

    let args = [Variant::I64(3), Variant::I64(4)];
    let result = globals.call_callback("add", &args).unwrap();

    assert_eq!(result, Variant::I64(7));
  }
}
