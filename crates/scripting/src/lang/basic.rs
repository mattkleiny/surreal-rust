//! BASIC language support for Surreal

use super::*;

/// The BASIC scripting language
pub struct BASIC;

impl ScriptLanguage for BASIC {
  fn load(_path: impl ToVirtualPath) -> Result<Script, ScriptError> {
    todo!()
  }
}
