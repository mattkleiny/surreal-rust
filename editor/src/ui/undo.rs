//! Undo/Redo support for the Surreal editor.
//!
//! This module provides the [`UndoManager`] type, which is responsible for
//! managing the undo/redo.

use std::ops::Deref;

/// A handler for undo/redo actions in the editor.
#[derive(Default)]
pub struct UndoManager {}

/// A scope wrapper for a type that can be rolled backwards/forwards in response to user actions.
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
