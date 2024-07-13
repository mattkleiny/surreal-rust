//! LISP language support for Surreal

use super::*;

/// The LISP scripting language
pub struct Lisp;

impl ScriptLanguage for Lisp {
  fn load(_path: impl ToVirtualPath) -> Result<Script, ScriptError> {
    todo!()
  }
}
