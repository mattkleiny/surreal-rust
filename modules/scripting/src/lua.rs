use common::ToStringName;
use mlua::Lua;

use super::*;

/// The Lua [`ScriptLanguage`] implementation.
pub struct LuaLanguage;

impl ScriptProvider for LuaLanguage {
  fn name(&self) -> StringName {
    "Lua".to_string_name()
  }

  fn file_extension(&self) -> StringName {
    "lua".to_string_name()
  }
}

impl ScriptLanguage for LuaLanguage {
  type Script = LuaScript;

  fn compile_code(&self, code: &str) -> common::Result<Self::Script> {
    Ok(LuaScript::from_code(code))
  }
}

/// A script in the Lua scripting language.
pub struct LuaScript {
  lua: Lua,
  code: String,
}

impl LuaScript {
  /// Creates a new [`LuaScript`] from the given raw Lua code.
  pub fn from_code(code: &str) -> Self {
    Self {
      lua: Lua::new(),
      code: code.to_string(),
    }
  }
}

impl Script for LuaScript {
  fn execute(&mut self) -> common::Result<()> {
    let chunk = self.lua.load(&self.code);

    Ok(chunk.exec()?)
  }

  fn call(&mut self, _name: &str, _arguments: &[Variant]) -> common::Result<Variant> {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_execute_simple_script() {
    let mut script = LuaLanguage.compile_code("print(\"Hello, world!\")").unwrap();

    script.execute().unwrap();
  }
}
