//! Undo/Redo support for the Surreal editor.
//!
//! This module provides the [`UndoManager`] type, which is responsible for managing the
//! undo/redo stack for the editor.

use std::ops::Deref;

/// A handler for undo/redo actions in the editor.
#[derive(Default)]
pub struct UndoManager {}

/// A scope wrapper for a type that can be rolled backwards/forwards via [`EditorCommand`]s,
/// for use in undo/redo scenarios.
pub struct UndoScope<T> {
  pub value: T,
}

impl<T> UndoScope<T> {
  /// Creates a new [`UndoScope`] for the given value.
  pub fn new(value: T) -> Self {
    Self { value }
  }
}

impl<T> Deref for UndoScope<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

/// A command that can be undone/redone in the editor.
pub trait EditorCommand<T> {
  /// Redoes the command/rolls forward.
  fn redo(&self, value: &mut T) -> surreal::Result<()>;

  /// Undoes the command/rolls backward.
  fn undo(&self, value: &mut T) -> surreal::Result<()>;
}
