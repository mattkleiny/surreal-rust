use crate::{lang::ast::*, runtime::Opcode};

/// An error that occurs when compiling.
#[derive(Debug)]
pub enum CompileError {}

/// Compiles a single expression into a sequence of opcodes
pub fn compile_expression(expression: &Expression) -> Result<Vec<Opcode>, CompileError> {
  let mut compiler = Compiler::default();

  compiler.compile_expression(expression)?;

  Ok(compiler.instructions)
}

/// Compiles a set of [`Statement`]s into [`Opcode`]s.
pub fn compile_statements(statements: &[Statement]) -> Result<Vec<Opcode>, CompileError> {
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
  /// Compiles a single statement into a sequence of opcodes.
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

  /// Compiles a single expression into a sequence of opcodes.
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

  macro_rules! compile_test {
    ($name:ident, $statements:expr, $expected:expr) => {
      #[test]
      fn $name() {
        let instructions = compile_statements($statements).unwrap();
        assert_eq!(instructions, $expected);
      }
    };
  }

  compile_test!(
    test_compile_basic_program,
    &vec![Statement::Return(Expression::Binary(
      Box::new(Expression::Literal(Variant::I64(1))),
      BinaryOp::Add,
      Box::new(Expression::Literal(Variant::I64(2))),
    ))],
    vec![
      Opcode::Literal(Variant::I64(1)),
      Opcode::Literal(Variant::I64(2)),
      Opcode::Binary(BinaryOp::Add),
      Opcode::Return,
    ]
  );

  compile_test!(
    test_compile_single_expression,
    &vec![Statement::Expression(Expression::Binary(
      Box::new(Expression::Literal(Variant::I64(3))),
      BinaryOp::Multiply,
      Box::new(Expression::Literal(Variant::I64(4))),
    ))],
    vec![
      Opcode::Literal(Variant::I64(3)),
      Opcode::Literal(Variant::I64(4)),
      Opcode::Binary(BinaryOp::Multiply),
    ]
  );
}
