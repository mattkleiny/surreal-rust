//! Lua language support for Surreal

use super::*;

/// The Lua scripting language
pub struct Lua;

impl ScriptLanguage for Lua {
  fn load(_path: impl ToVirtualPath) -> Result<Script, ScriptError> {
    todo!()
  }
}
