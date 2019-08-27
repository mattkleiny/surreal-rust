//! A simple in-game console.

use crate::collections::RingBuffer;

use super::*;

/// Interprets commands from an in-game console.
pub trait Interpreter {
  fn evaluate<C: AsRef<str>>(&mut self, command: C) -> Result<()>;
}

/// An interactive in-game console with log history.
pub struct InGameConsole<I: Interpreter> {
  /// The command interpreter to use when executing commands.
  interpreter: I,
  /// A history of the log events observed by the console.
  history: RingBuffer<String>,
}

impl<I: Interpreter> InGameConsole<I> {
  /// Creates a new console with the given log retention capacity.
  pub fn new(interpreter: I, capacity: usize) -> Self {
    Self {
      interpreter,
      history: RingBuffer::new(capacity),
    }
  }

  /// Evaluates the given command in the console.
  pub fn evaluate<C: AsRef<str>>(&mut self, command: C) {
    // TODO: handle these commands
    match self.interpreter.evaluate(command) {
      Ok(_result) => {}
      Err(error) => self.history.append(format!("{}", error))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct SimpleInterpreter;

  impl Interpreter for SimpleInterpreter {
    fn evaluate<C: AsRef<str>>(&mut self, _command: C) -> Result<()> {
      Err("it no worky".to_string())
    }
  }

  #[test]
  fn it_should_evaluate_commands() {
    let mut console = InGameConsole::new(SimpleInterpreter, 100);

    console.evaluate("print 'Hello, World!'");
  }
}