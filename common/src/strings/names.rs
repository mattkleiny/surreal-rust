use std::{
  fmt::{Debug, Display},
  sync::RwLock,
};

use crate::{Arena, Singleton};

crate::impl_arena_index!(StringId, "Identifies a string in a string pool.");

/// Represents an interned string that can be used as a name.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct StringName(StringId);

/// A trait for objects that have a [`StringName`].
pub trait ToStringName {
  /// Returns the name of this object as a string.
  fn to_string_name(&self) -> StringName;
}

/// Allows a string reference to be converted to a string name.
impl<R: AsRef<str>> ToStringName for R {
  fn to_string_name(&self) -> StringName {
    StringName::from(self.as_ref())
  }
}

/// Allows a string reference to be converted to a string name.
impl<R: AsRef<str>> From<R> for StringName {
  fn from(value: R) -> Self {
    let pool = StringNamePool::instance();

    StringName(pool.intern(value.as_ref()))
  }
}

/// Allows a string name to be compared to a string reference.
impl<R: AsRef<str>> PartialEq<R> for StringName {
  fn eq(&self, other: &R) -> bool {
    let pool = StringNamePool::instance();

    if let Some(value) = pool.lookup(self.0) {
      value == *other.as_ref()
    } else {
      false
    }
  }
}

impl PartialEq<StringName> for &str {
  #[inline]
  fn eq(&self, other: &StringName) -> bool {
    other == self
  }
}

/// Pretty-prints a string name.
impl Debug for StringName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let pool = StringNamePool::instance();

    if let Some(value) = pool.lookup(self.0) {
      write!(f, "{:?}", value)
    } else {
      write!(f, "StringName({:?})", self.0)
    }
  }
}

/// Pretty-prints a string name.
impl Display for StringName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let pool = StringNamePool::instance();

    if let Some(value) = pool.lookup(self.0) {
      write!(f, "{}", value)
    } else {
      write!(f, "")
    }
  }
}

#[cfg(feature = "serde")]
impl serde::Serialize for StringName {
  fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
    todo!()
  }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for StringName {
  fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
    todo!()
  }
}

/// An internal global pool of interned strings.
#[derive(Default, Singleton)]
struct StringNamePool {
  strings_by_id: RwLock<Arena<StringId, StringPoolEntry>>,
}

/// An entry in the string pool.
struct StringPoolEntry {
  string: String,
  reference_count: usize,
}

impl StringNamePool {
  /// Looks up the string with the given ID.
  ///
  /// If the string is not interned, returns `None`.
  pub fn lookup(&self, id: StringId) -> Option<String> {
    let entries = self.strings_by_id.read().unwrap();

    entries.get(id).map(|entry| entry.string.clone())
  }

  /// Interns the given string and returns its ID.
  ///
  /// If the string is already interned, its reference count is incremented.
  /// Otherwise, it is inserted into the pool.
  pub fn intern(&self, value: &str) -> StringId {
    // we need to manually scan the strings here because we optimize
    // for the case where the string is already interned
    let mut entries = self.strings_by_id.write().unwrap();

    for (id, entry) in entries.enumerate_mut() {
      if entry.string == value {
        entry.reference_count += 1;
        return id;
      }
    }

    // we need to drop the read lock before we can write to the map
    drop(entries);

    // insert the string into the map
    let mut entries = self.strings_by_id.write().unwrap();

    entries.insert(StringPoolEntry {
      string: value.to_owned(),
      reference_count: 1,
    })
  }

  /// Decrements the reference count for the given ID.
  ///
  /// If the reference count reaches zero, the string is removed from the pool.
  /// Otherwise, the reference count is decremented.
  #[allow(dead_code)] // TODO: consider if this should be used
  pub fn decrement(&self, id: StringId) {
    let mut entries = self.strings_by_id.write().unwrap();

    if let Some(entry) = entries.get_mut(id) {
      entry.reference_count -= 1;

      if entry.reference_count == 0 {
        entries.remove(id);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_string_name_should_intern_similar_strings() {
    let pool = StringNamePool::instance();

    let id1 = pool.intern("test");
    let id2 = pool.intern("test");
    let id3 = pool.intern("test2");

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
  }

  #[test]
  fn test_string_name_should_convert_from_reference() {
    let name1 = StringName::from("test");
    let name2 = StringName::from("test");

    assert_eq!(name1, name2);
  }
}
