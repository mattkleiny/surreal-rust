use std::{
  any::{Any, TypeId},
  collections::HashMap,
  hash::RandomState,
};

/// An any-map is a hash map that can contain a single value per [`TypeId`].
///
/// Any-maps are useful for storing values of different types in a single
/// container. They are also useful for storing values of the same type, but
/// where the type is not known at compile time.
#[derive(Default)]
pub struct AnyMap<S = RandomState> {
  entries: HashMap<TypeId, Box<dyn Any>, S>,
}

impl AnyMap {
  /// Creates a new any-map.
  #[inline]
  pub fn new() -> Self {
    Self {
      entries: HashMap::default(),
    }
  }

  /// Is the map empty?
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  /// The number of entries in the map.
  #[inline]
  pub fn len(&self) -> usize {
    self.entries.len()
  }

  /// Inserts a value into the map.
  #[inline]
  pub fn insert<T: Any>(&mut self, value: T) {
    self.entries.insert(TypeId::of::<T>(), Box::new(value));
  }

  /// Retrieves an existing item from the map or creates it anew via
  /// [`Default::default`].
  pub fn get_or_default<T: Any + Default>(&mut self) -> &mut T {
    self
      .entries
      .entry(TypeId::of::<T>())
      .or_insert_with(|| Box::<T>::default())
      .downcast_mut()
      .unwrap() // not possible (i hope)
  }

  /// Mutably accesses a value from the map.
  pub fn get<T: Any>(&self) -> Option<&T> {
    self.entries.get(&TypeId::of::<T>()).and_then(|any| any.downcast_ref())
  }

  /// Mutably accesses a given value from the map.
  pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
    self
      .entries
      .get_mut(&TypeId::of::<T>())
      .and_then(|any| any.downcast_mut())
  }

  /// Removes a value from the map.
  #[inline]
  pub fn remove<T: Any>(&mut self) {
    self.entries.remove(&TypeId::of::<T>());
  }

  /// Clears the map.
  #[inline]
  pub fn clear(&mut self) {
    self.entries.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_should_support_basic_read_and_write() {
    let mut map = AnyMap::new();

    map.insert(42usize);
    map.insert("Hello, World");

    assert_eq!(map.get::<usize>(), Some(&42));
    assert_eq!(map.get::<&'static str>(), Some(&"Hello, World"));
    assert!(map.get::<bool>().is_none());
  }

  #[test]
  fn test_should_return_none_for_nonexistent_value() {
    let mut map = AnyMap::new();

    map.insert(42usize);

    assert!(map.get::<&'static str>().is_none());
    assert!(map.get::<bool>().is_none());
  }

  #[test]
  fn test_should_return_mutable_reference() {
    let mut map = AnyMap::new();

    map.insert(42usize);

    if let Some(value) = map.get_mut::<usize>() {
      *value = 100;
    }

    assert_eq!(map.get::<usize>(), Some(&100));
  }

  #[test]
  fn test_should_remove_value() {
    let mut map = AnyMap::new();

    map.insert(42usize);

    assert_eq!(map.get::<usize>(), Some(&42));

    map.remove::<usize>();

    assert!(map.get::<usize>().is_none());
  }

  #[test]
  fn test_should_clear_map() {
    let mut map = AnyMap::new();

    map.insert(42usize);
    map.insert("Hello, World");

    assert_eq!(map.len(), 2);

    map.clear();

    assert_eq!(map.len(), 0);
    assert!(map.get::<usize>().is_none());
    assert!(map.get::<&'static str>().is_none());
  }
}
