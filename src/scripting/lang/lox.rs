//! Scripting language implementation for the 'Lox' language.
//!
//! Based on the work from the excellent book, 'Crafting Interpreters'.

use super::*;

/// A scripting language based on 'Lox' from the 'Crafting Interpreters' series.
///
/// This is a simple language that offers basic functionality for the engine.
/// It's simple to integrate and serves as a first example of more complex scripting.
struct LoxLanguage {}

impl ScriptLanguage for LoxLanguage {
  fn compile(&self, code: &str) -> crate::Result<BytecodeChunk> {
    let _tokens = parser::tokenize(code)?;

    todo!()
  }
}

mod parser {
  //! Parsing and tokenization for the Lox language.

  use super::*;

  use std::collections::HashMap;

  /// Represents a token in the Lox language.
  #[derive(Debug)]
  pub enum Token {
    // single characters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // one or two characters
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // literals
    Identifier(String),
    String(String),
    Number(f64),
    Keyword(Keyword),
  }

  /// Keywords supported by Lox.
  #[derive(Copy, Clone, Debug)]
  pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
  }

  lazy_static::lazy_static! {
    /// A look-up table of keywords.
    static ref KEYWORDS: HashMap<&'static str, Keyword> = {
      let mut m = HashMap::new();
      m.insert("and", Keyword::And);
      m.insert("class", Keyword::Class);
      m.insert("else", Keyword::Else);
      m.insert("false", Keyword::False);
      m.insert("for", Keyword::For);
      m.insert("fun", Keyword::Fun);
      m.insert("if", Keyword::If);
      m.insert("nil", Keyword::Nil);
      m.insert("or", Keyword::Or);
      m.insert("print", Keyword::Print);
      m.insert("return", Keyword::Return);
      m.insert("super", Keyword::Super);
      m.insert("this", Keyword::This);
      m.insert("true", Keyword::True);
      m.insert("var", Keyword::Var);
      m.insert("while", Keyword::While);
      m
    };
  }

  /// Tokenizes the given string into a list of `Token`.
  pub fn tokenize(code: &str) -> crate::Result<Vec<(Token, TokenPos)>> {
    let mut tokens = Vec::new();
    let token_pos = TokenPos { line: 1, column: 1 }; // TODO: track me
    let mut characters = code.chars().peekable();

    macro_rules! emit {
      // emits a new token into the output
      ($token:expr) => {
        tokens.push(($token, token_pos))
      };
      // emits one of two new token into the output depending on the peek character
      ($token1:expr, $token2:expr, $peek:expr) => {
        if let Some($peek) = characters.peek() {
          characters.next(); // consume peeked character
          emit!($token2);
        } else {
          emit!($token1);
        }
      };
    }

    // TODO: better column/line tracking?
    while let Some(character) = characters.next() {
      match character {
        // single characters
        '(' => emit!(Token::LeftParen),
        ')' => emit!(Token::RightParen),
        '{' => emit!(Token::LeftBrace),
        '}' => emit!(Token::RightBrace),
        ';' => emit!(Token::SemiColon),
        ',' => emit!(Token::Comma),
        '.' => emit!(Token::Dot),
        '-' => emit!(Token::Minus),
        '+' => emit!(Token::Plus),
        '*' => emit!(Token::Star),

        // one or two characters
        '!' => emit!(Token::Bang, Token::BangEqual, '='),
        '=' => emit!(Token::Equal, Token::EqualEqual, '='),
        '<' => emit!(Token::Less, Token::LessEqual, '='),
        '>' => emit!(Token::Greater, Token::GreaterEqual, '='),

        // potential comments
        '/' => {
          if let Some('/') = characters.peek() {
            characters.next(); // consume 'comments'

            while let Some(character) = characters.next() {
              if character == '\n' {
                break;
              }
            }
          } else {
            emit!(Token::Slash);
          }
        }

        // strings
        '"' => {
          let mut string = String::new();

          while let Some(character) = characters.next() {
            if character != '"' {
              string.push(character);
            } else {
              break;
            }
          }

          emit!(Token::String(string));
        }

        // numbers
        '0'..='9' => {
          let mut number = String::new();

          number.push(character);

          while let Some(character) = characters.next() {
            if character.is_ascii_digit() {
              number.push(character);
            } else if character == '.' {
              if let Some(next) = characters.peek() {
                if next.is_ascii_digit() {
                  number.push(character);
                  continue;
                }
              }
              break;
            } else {
              break;
            }
          }

          emit!(Token::Number(number.parse()?));
        }

        // identifiers and keywords
        'a'..='z' | 'A'..='Z' | '_' => {
          let mut identifier = String::new();

          identifier.push(character);

          while let Some(character) = characters.next() {
            if character.is_ascii_alphanumeric() || character == '_' {
              identifier.push(character);
            } else {
              break;
            }
          }

          if let Some(keyword) = KEYWORDS.get(identifier.as_str()) {
            emit!(Token::Keyword(*keyword));
          } else {
            emit!(Token::Identifier(identifier));
          }
        }

        ' ' | '\t' | '\n' => {} // ignore whitespace

        _ => anyhow::bail!("An unexpected token was encountered {:}", character),
      }
    }

    Ok(tokens)
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn it_should_tokenize_simple_strings() {
      println!("{:?}", tokenize("(){};.+-").unwrap());
      println!("{:?}", tokenize("\"12345678\"").unwrap());
      println!("{:?}", tokenize("123456789").unwrap());
      println!("{:?}", tokenize("3.14159").unwrap());
      println!("{:?}", tokenize("gorgonzola").unwrap());
      println!("{:?}", tokenize("forest").unwrap());
      println!("{:?}", tokenize("for").unwrap());
    }
  }
}
