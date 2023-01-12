use std::sync::RwLock;

use surreal::collections::Arena;
use surreal::utilities::ResourceId;

/// Thread-safe storage for [`ResourceId`] [`K`] to some internal data structure [`V`].
///
/// This allows for opaque decoupling of user-facing resource IDs and internal data structures.
pub struct ResourceStorage<K, V> {
  entries: RwLock<Arena<V>>,
  _key: std::marker::PhantomData<K>,
}

impl<K: ResourceId, V> Default for ResourceStorage<K, V> {
  fn default() -> Self {
    Self {
      entries: RwLock::new(Arena::new()),
      _key: std::marker::PhantomData,
    }
  }
}

impl<K: ResourceId, V> ResourceStorage<K, V> {
  /// Creates a new [`V`] in the storage with the given factory method.
  pub fn create(&self, factory: impl Fn() -> V) -> K {
    self.insert(factory())
  }

  /// Reads the [`V`] associated with the given key.
  pub fn read<R>(&self, key: K, body: impl FnMut(&V) -> R) -> Option<R> {
    let entries = self.entries.read().unwrap();

    entries.get(key.into()).map(body)
  }

  /// Writes the [`V`] associated with the given key.
  pub fn write<R>(&self, key: K, body: impl FnMut(&mut V) -> R) -> Option<R> {
    let mut entries = self.entries.write().unwrap();

    entries.get_mut(key.into()).map(body)
  }

  /// Inserts a [`V`] into storage and returns it's [`K`].
  pub fn insert(&self, value: V) -> K {
    let mut entries = self.entries.write().unwrap();

    entries.insert(value).into()
  }

  /// Removes a [`V`] from storage.
  pub fn remove(&self, key: K) -> Option<V> {
    let mut entries = self.entries.write().unwrap();

    entries.remove(key.into())
  }
}
