use std::sync::RwLock;

use crate::collections::{Arena, ArenaIndex};

/// Creates a new, opaque resource id type.
///
/// The type is implicitly convertible to and from [`u64`], [`u32`], and
/// [`ArenaIndex`], and can be used as a key in [`ResourceStorage`].
#[macro_export]
macro_rules! impl_rid {
  ($name:ident) => {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct $name($crate::collections::ArenaIndex);

    impl $name {
      pub const NONE: Self = Self($crate::collections::ArenaIndex::NONE);
    }

    impl From<u32> for $name {
      #[inline]
      fn from(id: u32) -> Self {
        let id: u64 = id.into();
        Self((id.into()))
      }
    }

    impl From<$name> for u32 {
      #[inline]
      fn from(id: $name) -> Self {
        let id: u64 = id.into();
        id as u32
      }
    }

    impl From<u64> for $name {
      #[inline]
      fn from(id: u64) -> Self {
        Self(id.into())
      }
    }

    impl From<$name> for u64 {
      #[inline]
      fn from(id: $name) -> Self {
        id.0.into()
      }
    }

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

    impl $crate::maths::FromRandom for $name {
      #[inline]
      fn from_random(random: &mut $crate::maths::Random) -> Self {
        Self::from(random.next_u64())
      }
    }
  };
}

/// Thread-safe storage for [`ResourceId`] [`K`] to some internal data structure
/// [`V`].
///
/// This allows for opaque decoupling of user-facing resource IDs and internal
/// data structures.
pub struct ResourceArena<K, V> {
  entries: RwLock<Arena<V>>,
  _key: std::marker::PhantomData<K>,
}

impl<K, V> Default for ResourceArena<K, V> {
  fn default() -> Self {
    Self {
      entries: RwLock::new(Arena::new()),
      _key: std::marker::PhantomData,
    }
  }
}

impl<K: Into<ArenaIndex> + From<ArenaIndex>, V> ResourceArena<K, V> {
  /// Creates a new [`V`] in the storage with the given factory method.
  pub fn create(&self, factory: impl Fn() -> V) -> K {
    self.insert(factory())
  }

  /// Reads the [`V`] associated with the given key.
  pub fn read<R>(&self, key: K, body: impl FnMut(&V) -> R) -> Option<R> {
    let entries = self.entries.read().unwrap();

    entries.get(key.into()).map(body)
  }

  /// Reads all entries in the storage.
  pub fn read_all(&self, body: impl FnMut(&V)) {
    let entries = self.entries.read().unwrap();

    entries.iter().map(|(_, v)| v).for_each(body);
  }

  /// Writes the [`V`] associated with the given key.
  pub fn write<R>(&self, key: K, body: impl FnMut(&mut V) -> R) -> Option<R> {
    let mut entries = self.entries.write().unwrap();

    entries.get_mut(key.into()).map(body)
  }

  /// Writes all entries in the storage.
  pub fn write_all(&self, body: impl FnMut(&mut V)) {
    let mut entries = self.entries.write().unwrap();

    entries.iter_mut().map(|(_, v)| v).for_each(body);
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

  /// Clears all entries from storage.
  pub fn clear(&self) {
    let mut entries = self.entries.write().unwrap();

    entries.clear();
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
    let storage = ResourceArena::<TestId, TestResource>::default();

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
