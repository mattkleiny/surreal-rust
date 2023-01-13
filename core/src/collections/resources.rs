use std::sync::RwLock;

use crate::collections::{Arena, ArenaIndex};

/// Abstracts over resource IDs.
pub trait ResourceId: Copy + Eq + From<ArenaIndex> + Into<ArenaIndex> {}

/// Creates a new, opaque [`ResourceId`] type.
#[macro_export]
macro_rules! impl_rid {
  ($name:ident) => {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct $name($crate::collections::ArenaIndex);

    impl $crate::collections::ResourceId for $name {}

    impl From<$crate::collections::ArenaIndex> for $name {
      #[inline]
      fn from(id: $crate::collections::ArenaIndex) -> Self {
        Self(id)
      }
    }

    impl From<$name> for $crate::collections::ArenaIndex {
      #[inline]
      fn from(id: $name) -> Self {
        id.0
      }
    }
  };
}

/// Thread-safe storage for [`ResourceId`] [`K`] to some internal data structure
/// [`V`].
///
/// This allows for opaque decoupling of user-facing resource IDs and internal
/// data structures.
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

#[cfg(test)]
mod tests {
  use super::*;

  impl_rid!(TestId);

  pub struct TestResource {
    value: u16,
  }

  #[test]
  fn resource_storage_should_read_and_write_entries() {
    let storage = ResourceStorage::<TestId, TestResource>::default();

    let id1 = storage.insert(TestResource { value: 0 });
    let id2 = storage.insert(TestResource { value: 1 });

    storage.read(id1, |entry| {
      assert_eq!(entry.value, 0);
    });

    storage.write(id2, |entry| {
      entry.value = 2;
    });

    storage.read(id2, |entry| {
      assert_eq!(entry.value, 2);
    });
  }
}
