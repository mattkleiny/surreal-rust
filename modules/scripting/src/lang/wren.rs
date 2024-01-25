use super::*;

/// The Wren [`ScriptLanguage`] implementation.
///
/// This is based on the Wren language by Bob Nystrom.
/// Find more information about Wren at https://wren.io/.
pub struct Wren;

impl ScriptLanguage for Wren {
  fn name(&self) -> &'static str {
    "Wren"
  }

  fn file_extensions(&self) -> &[&'static str] {
    &["wren"]
  }

  fn parse_code(&self, _code: &str) -> common::Result<ast::Module> {
    let _module = parser::parse(_code)?;

    todo!()
  }
}

mod parser {
  use super::*;

  /// Parses the given Wren code into a [`Module`].
  pub fn parse(_code: &str) -> common::Result<ast::Module> {
    todo!()
  }

  #[derive(Debug, PartialEq)]
  enum Token {}

  ast::impl_token_stream!(Token as TokenStream);
}
