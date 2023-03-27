//! Undo/Redo support for the Surreal editor.
//!
//! This module provides the [`UndoManager`] type, which is responsible for
//! managing the undo/redo.

use std::ops::{Deref, DerefMut};

/// A handler for undo/redo actions in the editor.
#[derive(Default)]
pub struct UndoManager {}

/// A scope wrapper for a type that can be rolled backwards/forwards in response
/// to user actions.
pub struct UndoScope<T> {
  pub value: T,
}

impl<T> UndoScope<T> {
  /// Creates a new [`UndoScope`] for the given value.
  pub fn new(value: T) -> Self {
    Self { value }
  }

  /// Undoes the most recent change (TODO: redo, too?)
  pub fn undo(&mut self) {
    todo!()
  }

  /// Starts a new [`DiffScope`] for the current value.
  pub fn modify(&mut self) -> DiffScope<T>
  where
    T: Diffable,
  {
    DiffScope::new(&mut self.value)
  }
}

impl<T> Deref for UndoScope<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

/// A utility for working with [`DiffSnapshot`]s and committing them to an
/// undo/redo stack.
pub struct DiffScope<'a, T> {
  _snapshot: DiffSnapshot,
  value: &'a mut T,
}

impl<'a, T: Diffable> DiffScope<'a, T> {
  /// Creates a new [`DiffScope`] for the given [`Diffable`] type.
  pub fn new(value: &'a mut T) -> Self {
    Self {
      _snapshot: value.snapshot(),
      value,
    }
  }

  /// Commits the changes made to the object back to the given [`UndoScope`].
  pub fn commit(self, _description: impl Into<String>) {
    todo!()
  }
}

impl<'a, T> Deref for DiffScope<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.value
  }
}

impl<'a, T> DerefMut for DiffScope<'a, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.value
  }
}

/// An object that can be diffed.
pub trait Diffable {
  /// Produces a [`DiffSnapshot`] for this object.
  fn snapshot(&self) -> DiffSnapshot;
}

/// A snapshot of a [`Diffable`] object's fields.
#[derive(Default)]
pub struct DiffSnapshot {}

impl DiffSnapshot {
  /// Adds a field to the snapshot.
  pub fn add_field(self, _name: impl Into<String>, _value: impl PartialEq) -> Self {
    self
  }

  /// Compares this [`DiffSnapshot`] to another [`DiffSnapshot`], yielding all
  /// differences.
  pub fn compare_to(&self, _other: &Self) -> Vec<DiffDelta> {
    Vec::with_capacity(0)
  }
}

/// Describes the difference between two [`DiffSnapshot`]s.
pub struct DiffDelta {}

#[cfg(test)]
mod tests {
  use serde::{Deserialize, Serialize};
  use surreal::graphics::Color;

  use super::*;

  #[derive(Serialize, Deserialize)]
  pub struct PointLight {
    pub color: Color,
    pub radius: f32,
  }

  impl Default for PointLight {
    fn default() -> Self {
      Self {
        color: Color::BLACK,
        radius: 1.0,
      }
    }
  }

  impl Diffable for PointLight {
    fn snapshot(&self) -> DiffSnapshot {
      DiffSnapshot::default()
        .add_field("color", self.color)
        .add_field("radius", self.radius)
    }
  }

  // #[test]
  // fn changes_should_be_staged_via_diff_scope() {
  //   let mut light = UndoScope::new(PointLight::default());
  //   let mut diff = light.modify();

  //   diff.color = Color::WHITE;
  //   diff.radius = 1.5;

  //   diff.commit("Changed light color and radius.");

  //   assert_eq!(light.color, Color::WHITE);
  //   assert_eq!(light.radius, 1.5);

  //   light.undo();

  //   assert_eq!(light.color, Color::BLACK);
  //   assert_eq!(light.radius, 1.0);
  // }
}
