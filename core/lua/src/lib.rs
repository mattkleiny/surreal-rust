//! A custom-baked Lua wrapper with support.

use mlua_sys::*;

#[derive(Debug)]
pub enum LuaError {
  FailedToExecute,
}

/// A Lua interpreter.
pub struct LuaInterpreter {
  state: *mut lua_State,
}

impl Default for LuaInterpreter {
  fn default() -> Self {
    Self::new()
  }
}

impl LuaInterpreter {
  /// Creates a new Lua interpreter.
  pub fn new() -> Self {
    unsafe {
      let lua = luaL_newstate();

      luaopen_base(lua);
      luaopen_math(lua);
      luaopen_string(lua);
      luaopen_table(lua);

      Self { state: lua }
    }
  }

  /// Executes the given Lua code.
  pub fn execute(&mut self, code: impl AsRef<str>) -> Result<(), LuaError> {
    unsafe {
      let code = std::ffi::CString::new(code.as_ref()).unwrap();

      if luaL_loadstring(self.state, code.as_ptr()) != LUA_OK {
        return Err(LuaError::FailedToExecute);
      }

      if lua_pcall(self.state, 0, 0, 0) != LUA_OK {
        return Err(LuaError::FailedToExecute);
      }

      lua_pushcfunction(self.state, example);

      lua_pop(self.state, lua_gettop(self.state));
    }

    Ok(())
  }
}

impl Drop for LuaInterpreter {
  fn drop(&mut self) {
    unsafe {
      lua_close(self.state);
    }
  }
}

unsafe extern "C-unwind" fn example(state: *mut lua_State) -> i32 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_lua_interpreter_execution() {
    let mut interpreter = LuaInterpreter::default();

    let code = r#"
      print("Hello, World!")
    "#;

    interpreter.execute(code).unwrap();
  }
}
