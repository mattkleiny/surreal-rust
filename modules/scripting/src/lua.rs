use mlua::{Lua, StdLib};
use surreal::collections::ResourceStorage;

use super::*;

/// The [`ScriptServer`] implementation for lua.
pub struct LuaScriptServer {
  _lua: Lua,
  scripts: ResourceStorage<ScriptId, LuaScript>,
}

/// Internal state for a script in the [`LuaScriptServer`].
struct LuaScript {}

impl LuaScriptServer {
  /// Creates a new instance of the [`LuaScriptServer`].
  pub fn new() -> surreal::Result<Self> {
    Ok(Self {
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
      scripts: ResourceStorage::default(),
    })
  }
}

#[allow(unused_variables)]
impl ScriptServer for LuaScriptServer {
  fn name(&self) -> &str {
    "Lua"
  }

  fn extensions(&self) -> &[&str] {
    &["lua"]
  }

  fn script_create(&self) -> surreal::Result<ScriptId> {
    Ok(self.scripts.insert(LuaScript {}))
  }

  fn script_load(&self, script_id: ScriptId, path: &VirtualPath) -> surreal::Result<()> {
    self.scripts.write(script_id, |script| {
      // TODO: load the script contents
    });

    Ok(())
  }

  fn script_execute(&self, script_id: ScriptId, method: &str, arguments: &[Variant]) -> surreal::Result<Variant> {
    let result = self.scripts.write(script_id, |script| {
      // TODO: execute the script with the given arguments and return the result

      Variant::Null
    });

    Ok(result.unwrap_or(Variant::Null))
  }

  fn script_delete(&self, script_id: ScriptId) -> surreal::Result<()> {
    self.scripts.remove(script_id);

    Ok(())
  }
}
