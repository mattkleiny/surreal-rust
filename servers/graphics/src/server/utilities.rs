use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::RwLock;

use surreal::utilities::RID;

/// Thread-safe storage for [`RID`] [`K`] to some internal data structure [`V`].
///
/// This allows for opaque decoupling of user-facing resource IDs and internal data structures.
pub struct Storage<K, V> {
  next_id: AtomicU64,
  entries: RwLock<HashMap<K, V>>,
}

impl<K: RID, V> Default for Storage<K, V> {
  fn default() -> Self {
    Self {
      next_id: AtomicU64::new(1),
      entries: RwLock::new(HashMap::new()),
    }
  }
}

impl<K: RID, V> Storage<K, V> {
  /// Creates a new [`V`] in the storage with the given factory method.
  pub fn create(&self, factory: impl Fn(K) -> V) -> K {
    let next_id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    let key = K::from_u64(next_id);
    let value = factory(key);

    self.insert(key, value);

    key
  }

  /// Reads the [`V`] associated with the given key.
  pub fn read<R>(&self, key: K, body: impl Fn(&V) -> R) -> Option<R> {
    let entries = self.entries.read().unwrap();

    entries.get(&key).map(body)
  }

  /// Writes the [`V`] associated with the given key.
  pub fn write<R>(&self, key: K, body: impl Fn(&mut V) -> R) -> Option<R> {
    let mut entries = self.entries.write().unwrap();

    entries.get_mut(&key).map(body)
  }

  /// Inserts a [`V`] into storage.
  pub fn insert(&self, key: K, value: V) {
    let mut entries = self.entries.write().unwrap();

    entries.insert(key, value);
  }

  /// Removes a [`V`] from storage.
  pub fn remove(&self, key: K) -> Option<V> {
    let mut entries = self.entries.write().unwrap();

    entries.remove(&key)
  }
}
