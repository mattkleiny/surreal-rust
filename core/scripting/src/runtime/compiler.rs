use crate::{lang::ast::*, runtime::Opcode};

/// An error that occurs when compiling.
#[derive(Debug)]
pub enum CompileError {}

/// Compiles a set of [`Statement`]s into [`Opcode`]s.
pub fn compile(statements: &[Statement]) -> Result<Vec<Opcode>, CompileError> {
  let mut compiler = Compiler::default();

  for statement in statements {
    compiler.compile_statement(statement)?;
  }

  Ok(compiler.instructions)
}

/// Context for the compiler.
#[derive(Default)]
struct Compiler {
  instructions: Vec<Opcode>,
}

impl Compiler {
  pub fn compile_statement(&mut self, statement: &Statement) -> Result<(), CompileError> {
    match statement {
      Statement::Expression(expression) => {
        // Compile the expression and push the result onto the stack.
        self.compile_expression(expression)?
      }
      Statement::Return(expression) => {
        self.compile_expression(expression)?;
        self.instructions.push(Opcode::Return);
      }
      _ => todo!(),
    }

    Ok(())
  }

  pub fn compile_expression(&mut self, expression: &Expression) -> Result<(), CompileError> {
    match expression {
      Expression::Literal(literal) => {
        let value = literal.clone();

        self.instructions.push(Opcode::Literal(value))
      }
      Expression::Binary(left, operator, right) => {
        self.compile_expression(left)?;
        self.compile_expression(right)?;

        self.instructions.push(Opcode::Binary(*operator));
      }
      Expression::Unary(operator, value) => {
        self.compile_expression(value)?;

        self.instructions.push(Opcode::Unary(*operator));
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use common::Variant;

  use super::*;

  #[test]
  fn test_compile_basic_program() {
    let statements = vec![Statement::Return(Expression::Binary(
      Box::new(Expression::Literal(Variant::I64(1))),
      BinaryOp::Add,
      Box::new(Expression::Literal(Variant::I64(2))),
    ))];

    let instructions = compile(&statements).unwrap();

    assert_eq!(instructions, vec![
      Opcode::Literal(Variant::I64(1)),
      Opcode::Literal(Variant::I64(2)),
      Opcode::Binary(BinaryOp::Add),
      Opcode::Return,
    ]);
  }
}
