//! Helpers for building up user-facing errors and diagnostic messages.

use std::fmt::Display;

/// Represents an error that the user can fix.
#[derive(Debug)]
pub struct UserError {
  message: String,
  parts: Vec<UserErrorPart>,
}

/// Represents a user-facing error message.
#[derive(Debug)]
enum UserErrorPart {
  Reason(String),
  Hint(String),
}

impl UserError {
  /// Creates a new user-facing error message.
  pub fn new(message: impl AsRef<str>) -> Self {
    Self {
      message: message.as_ref().to_owned(),
      parts: Vec::new(),
    }
  }

  /// Adds a reason to the error message.
  pub fn reason(mut self, reason: impl AsRef<str>) -> Self {
    self.parts.push(UserErrorPart::Reason(reason.as_ref().to_owned()));
    self
  }

  /// Adds a reason to the error message.
  pub fn error(mut self, reason: impl std::error::Error) -> Self {
    self.parts.push(UserErrorPart::Reason(reason.to_string()));
    self
  }

  /// Adds a hint to the error message.
  pub fn hint(mut self, hint: impl AsRef<str>) -> Self {
    self.parts.push(UserErrorPart::Hint(hint.as_ref().to_owned()));
    self
  }
}

impl Display for UserError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Error: {}", self.message)?;

    for part in &self.parts {
      match part {
        UserErrorPart::Reason(reason) => writeln!(f, "  - {}", reason)?,
        UserErrorPart::Hint(hint) => writeln!(f, "Try: {}", hint)?,
      }
    }

    Ok(())
  }
}

/// Allows converting any error into a [`UserError`].
impl<E: std::error::Error> From<E> for UserError {
  fn from(error: E) -> Self {
    Self::new(error.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_error_creation() {
    let error = UserError::new("Failed to do the thing")
      .reason("The thing failed because of reason 1")
      .reason("The thing failed because of reason 2")
      .hint("Try doing the thing again");

    println!("{:#}", error);
  }
}
