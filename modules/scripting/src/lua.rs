use mlua::{Lua, StdLib};
use surreal::macros::Singleton;

use super::*;

/// The [`ScriptServer`] implementation for lua.
#[derive(Singleton)]
pub struct LuaScriptServer {
  _lua: Lua,
}

impl Default for LuaScriptServer {
  fn default() -> Self {
    Self::new().expect("Failed to create LuaScriptServer")
  }
}

impl LuaScriptServer {
  /// Creates a new instance of the [`LuaScriptServer`].
  pub fn new() -> surreal::Result<Self> {
    let server = Self {
      _lua: {
        // load standard library
        let lua = Lua::new();

        lua.load_from_std_lib(StdLib::MATH)?;
        lua.load_from_std_lib(StdLib::STRING)?;
        lua.load_from_std_lib(StdLib::TABLE)?;

        // create the 'Game' binding table
        lua.globals().set("Game", lua.create_table()?)?;

        lua
      },
    };

    Ok(server)
  }
}

#[allow(unused_variables)]
impl ScriptServer for LuaScriptServer {
  fn create_script(&self, prelude: &str) -> surreal::Result<ScriptId> {
    todo!()
  }

  fn delete_script(&self, script_id: ScriptId) -> surreal::Result<()> {
    todo!()
  }
}
