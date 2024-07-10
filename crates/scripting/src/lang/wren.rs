//! Wren support for surreal

use super::*;

/// The Wren scripting language and virtual machine.
pub struct Wren;

impl ScriptLanguage for Wren {
  fn load(path: impl ToVirtualPath) -> Result<Script, ScriptError> {
    let path = path.to_virtual_path();
    let text = path.read_all_text().map_err(|_| ScriptError::ParseError)?;

    parser::parse(&text).map_err(|_| ScriptError::ParseError)?;

    todo!()
  }
}

mod parser {
  use super::*;

  pub fn parse(text: &str) -> Result<ast::Module, ()> {
    todo!()
  }
}
