use std::any::{Any, TypeId};
use std::collections::HashMap;

/// An any-map is a map that can contain a single per unique type.
/// 
/// This is a variant of `AnyMultiMap` that supports single values per key.
pub struct AnyMap {
  entries: HashMap<TypeId, Box<dyn Any>>,
}

impl AnyMap {
  /// Creates a new any-map.
  pub fn new() -> Self {
    Self {
      entries: HashMap::new(),
    }
  }

  /// Is the map empty?
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  /// The number of entries in the map.
  pub fn len(&self) -> usize {
    self.entries.len()
  }

  /// Inserts a value into the map.
  pub fn insert<T: Any>(&mut self, value: T) {
    self.entries.insert(TypeId::of::<T>(), Box::new(value));
  }

  /// Retrieves an existing item from the map or creates it anew.
  pub fn get_or_create<T: Any + Default>(&mut self) -> &mut T {
    self
      .entries
      .entry(TypeId::of::<T>())
      .or_insert_with(|| Box::new(T::default()))
      .downcast_mut()
      .unwrap() // not possible (i hope)
  }

  /// Mutably accesses a value from the map.
  pub fn get<T: Any>(&self) -> Option<&T> {
    self
      .entries
      .get(&TypeId::of::<T>())
      .and_then(|any| any.downcast_ref())
  }

  /// Mutably accesses a given value from the map.
  pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
    self
      .entries
      .get_mut(&TypeId::of::<T>())
      .and_then(|any| any.downcast_mut())
  }

  /// Removes a value from the map.
  pub fn remove<T: Any>(&mut self) {
    self.entries.remove(&TypeId::of::<T>());
  }

  /// Clears the map.
  pub fn clear(&mut self) {
    self.entries.clear();
  }
}

/// An any-multi-map is a map that can contain multiple values per unique type.
/// 
/// This is a variant of `AnyMap` that supports multiple values per key.
pub struct AnyMultiMap {
  entries: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl AnyMultiMap {
  /// Creates a new any-multi-map.
  pub fn new() -> Self {
    Self {
      entries: HashMap::new(),
    }
  }

  /// Is the map empty?
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  /// The number of entries in the map.
  pub fn len(&self) -> usize {
    self.entries.len()
  }

  /// Inserts a value into the map.
  pub fn insert<T: Any>(&mut self, value: T) {
    self
      .entries
      .entry(TypeId::of::<T>())
      .or_insert_with(Vec::new)
      .push(Box::new(value));
  }

  /// Removes a value from the map.
  pub fn remove<T: Any>(&mut self) {
    self.entries.remove(&TypeId::of::<T>());
  }

  /// Clears the map.
  pub fn clear(&mut self) {
    self.entries.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn anymap_should_support_basic_read_and_write() {
    let mut map = AnyMap::new();

    map.insert(42usize);
    map.insert("Hello, World");

    assert_eq!(map.get::<usize>(), Some(&42));
    assert_eq!(map.get::<&'static str>(), Some(&"Hello, World"));
    assert!(map.get::<bool>().is_none());
  }

  #[test]
  fn anymultimap_should_support_basic_read_and_write() {
    let mut map = AnyMultiMap::new();

    map.insert(42usize);
    map.insert("Hello, World");
  }
}
