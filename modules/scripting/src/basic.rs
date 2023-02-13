//! BASIC language support for Surreal

use crate::{Compiler, Lexer, Parser};

struct BasicLexer<'a> {
  source: &'a str,
  position: usize,
}

impl<'a> Lexer for BasicLexer<'a> {
  type Token = Token;

  fn tokenize(&mut self) -> Result<Self::Token, crate::ParserError> {
    todo!()
  }
}

struct BasicParser<'a> {
  source: &'a str,
}

impl<'a> Parser for BasicParser<'a> {
  type Token = Token;
  type Expression = Expression;

  fn parse(&mut self, lexer: &mut impl Lexer<Token = Self::Token>) -> Result<Self::Expression, crate::ParserError> {
    match lexer.tokenize()? {
      Token::Literal(_) => todo!(),
      Token::Variable(_) => todo!(),
      Token::Add => todo!(),
      Token::Subtract => todo!(),
      Token::Multiply => todo!(),
      Token::Divide => todo!(),
      Token::Negate => todo!(),
      Token::OpenParen => todo!(),
      Token::CloseParen => todo!(),
    }
  }
}

struct BasicCompiler {}

impl Compiler for BasicCompiler {
  type Expression = Expression;

  fn compile(&mut self, _expression: Self::Expression) -> Result<crate::Program, crate::ParserError> {
    todo!()
  }
}

#[derive(Debug)]
enum Token {
  Literal(String),
  Variable(String),
  Add,
  Subtract,
  Multiply,
  Divide,
  Negate,
  OpenParen,
  CloseParen,
}

#[derive(Debug)]
enum Expression {
  Literal(Literal),
  Variable(String),
  BinaryOperation(BinaryOperation),
  UnaryOperation(UnaryOperation),
}

#[derive(Debug)]
enum Literal {
  Integer(i64),
  Float(f64),
  String(String),
}

#[derive(Debug)]
enum BinaryOperation {
  Add,
  Subtract,
  Multiply,
  Divide,
}

#[derive(Debug)]
enum UnaryOperation {
  Negate,
}
