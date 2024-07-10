//! Wren support for surreal

use super::*;

/// The Wren scripting language.
pub struct Wren;

impl ScriptLanguage for Wren {
  fn load(path: impl ToVirtualPath) -> Result<Script, ScriptError> {
    let path = path.to_virtual_path();

    let text = path.read_all_text().map_err(|_| ScriptError::ParseError)?;
    let module = parser::parse(&text).map_err(|_| ScriptError::ParseError)?;

    Ok(Script { module })
  }
}

mod parser {
  use common::{StringSpan, ToStringSpan};

  use super::*;

  struct Token<'a> {
    span: StringSpan<'a>,
    kind: TokenKind,
  }

  enum TokenKind {
    Identifier,
    Keyword,
    Operator,
    Literal,
    Comment,
  }

  pub fn parse(text: &str) -> Result<ast::Module, ()> {
    let mut tokens = Vec::new();
    let mut start = 0;

    for (i, c) in text.chars().enumerate() {
      if c.is_alphabetic() {
        if start < i {
          tokens.push(Token {
            span: text.to_string_span().slice(start, i),
            kind: TokenKind::Identifier,
          });
        }

        start = i + 1;
      }
    }

    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load_wren_files_from_file_system() {
    let script = Script::from_path::<Wren>("tests/test.wren").unwrap();

    assert_eq!(script.module.name, "test");
  }
}
