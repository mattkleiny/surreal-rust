//! Blackboard storage

use std::{any::Any, borrow::Cow};

use common::FastHashMap;

/// Represents a key for a blackboard entry.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BlackboardKey<T> {
  name: Cow<'static, str>,
  _marker: std::marker::PhantomData<T>,
}

impl<T> BlackboardKey<T> {
  /// Creates a new blackboard key.
  pub const fn new(name: &'static str) -> Self {
    Self {
      name: Cow::Borrowed(name),
      _marker: std::marker::PhantomData,
    }
  }
}

impl<T> Into<BlackboardKey<T>> for &'static str {
  fn into(self) -> BlackboardKey<T> {
    BlackboardKey::new(self)
  }
}

/// A blackboard is a storage for arbitrary data.
#[derive(Default)]
pub struct Blackboard {
  entries: FastHashMap<String, Box<dyn Any>>,
}

impl Blackboard {
  /// Creates a new blackboard.
  pub fn new() -> Self {
    Self {
      entries: FastHashMap::default(),
    }
  }

  /// Returns the number of entries in the blackboard.
  pub fn len(&self) -> usize {
    self.entries.len()
  }

  /// Is the blackboard empty?
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  /// Determines if the blackboard contains a value for the given key.
  pub fn contains<T, K>(&self, key: K) -> bool
  where
    T: Any,
    K: Into<BlackboardKey<T>>,
  {
    let key = key.into();
    let key = key.name.as_ref();

    self.entries.contains_key(key)
  }

  /// Inserts a value into the blackboard.
  pub fn insert<T, K>(&mut self, key: K, value: impl Any)
  where
    T: Any,
    K: Into<BlackboardKey<T>>,
  {
    let key = key.into();
    let key = key.name.to_string();

    self.entries.insert(key, Box::new(value));
  }

  /// Gets a value from the blackboard.
  pub fn get<T, K>(&self, key: K) -> Option<&T>
  where
    T: Any,
    K: Into<BlackboardKey<T>>,
  {
    let key = key.into();
    let key = key.name.as_ref();

    self.entries.get(key).and_then(|value| value.downcast_ref())
  }

  /// Gets a mutable value from the blackboard.
  pub fn get_mut<T, K>(&mut self, key: K) -> Option<&mut T>
  where
    T: Any,
    K: Into<BlackboardKey<T>>,
  {
    let key = key.into();
    let key = key.name.as_ref();

    self.entries.get_mut(key).and_then(|value| value.downcast_mut())
  }

  /// Removes a value from the blackboard.
  pub fn remove<T, K>(&mut self, key: K) -> Option<Box<T>>
  where
    T: Any,
    K: Into<BlackboardKey<T>>,
  {
    let key = key.into();
    let key = key.name.as_ref();

    self.entries.remove(key).and_then(|value| value.downcast().ok())
  }

  /// Clears the blackboard.
  pub fn clear(&mut self) {
    self.entries.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_KEY_1: BlackboardKey<i32> = BlackboardKey::new("foo");
  const TEST_KEY_2: BlackboardKey<&str> = BlackboardKey::new("bar");

  #[test]
  fn test_blackboard_read_write() {
    let mut blackboard = Blackboard::new();

    blackboard.insert(TEST_KEY_1, 42);
    blackboard.insert(TEST_KEY_2, "baz");

    assert_eq!(blackboard.get(TEST_KEY_1), Some(&42i32));
    assert_eq!(blackboard.get(TEST_KEY_2), Some(&"baz"));

    assert_eq!(blackboard.get_mut(TEST_KEY_1), Some(&mut 42i32));
    assert_eq!(blackboard.get_mut(TEST_KEY_2), Some(&mut "baz"));

    assert_eq!(blackboard.remove(TEST_KEY_1), Some(Box::new(42i32)));

    assert_eq!(blackboard.get(TEST_KEY_1), None);
  }
}
