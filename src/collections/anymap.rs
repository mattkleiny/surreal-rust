use std::any::{Any, TypeId};
use std::collections::HashMap;

/// An any-map is a map that can contain a single per unique type.
///
/// Entries are provided by the `Any` trait and support a simple `TypeId`-based lookup.
pub struct AnyMap {
  entries: HashMap<TypeId, Box<dyn Any>>,
}

impl AnyMap {
  /// Creates a new any-map.
  pub fn new() -> Self {
    Self {
      entries: HashMap::new()
    }
  }

  /// Inserts a value into the map.
  pub fn insert<T>(&mut self, value: T) where T: Any {
    self.entries.insert(TypeId::of::<T>(), Box::new(value));
  }

  /// Retrieves an existing item from the map or creates it anew.
  pub fn get_or_create<T>(&mut self) -> &mut T where T: Any + Default {
    self.entries
      .entry(TypeId::of::<T>())
      .or_insert_with(|| Box::new(T::default()))
      .downcast_mut()
      .unwrap() // not possible (i hope)
  }

  /// Mutably accesses a value from the map.
  pub fn get<T>(&self) -> Option<&T> where T: Any {
    self.entries
      .get(&TypeId::of::<T>())
      .and_then(|any| any.downcast_ref())
  }

  /// Mutably accesses a given value from the map.
  pub fn get_mut<T>(&mut self) -> Option<&mut T> where T: Any {
    self.entries
      .get_mut(&TypeId::of::<T>())
      .and_then(|any| any.downcast_mut())
  }

  /// Removes a value from the map.
  pub fn remove<T>(&mut self) where T: Any {
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
}