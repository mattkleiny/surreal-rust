use std::{
  borrow::Cow,
  fmt::{Debug, Display},
  sync::RwLock,
};

use macros::Singleton;

use crate::{Arena, ArenaIndex};

/// A custom string implementation for no_std targets.
#[cfg(feature = "no_std")]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct String {}

/// The default string implementation for std targets.
#[cfg(not(feature = "no_std"))]
pub type String = std::string::String;

/// Represents an interned string that can be used as a name.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct StringName(ArenaIndex);

/// A trait for objects that have a name.
pub trait ToStringName {
  /// Returns the name of this object as a string.
  fn to_string_name(&self) -> StringName;
}

impl ToStringName for String {
  fn to_string_name(&self) -> StringName {
    StringName::from(self.as_str())
  }
}

impl ToStringName for &str {
  fn to_string_name(&self) -> StringName {
    StringName::from(*self)
  }
}

impl From<&str> for StringName {
  fn from(value: &str) -> Self {
    let pool = StringNamePool::instance();

    StringName(pool.intern(value))
  }
}

impl<'a> From<Cow<'a, str>> for StringName {
  fn from(value: Cow<'a, str>) -> Self {
    let pool = StringNamePool::instance();

    StringName(pool.intern(&value))
  }
}

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

impl Drop for StringName {
  fn drop(&mut self) {
    let pool = StringNamePool::instance();

    pool.remove(self.0)
  }
}

/// An internal global pool of interned strings.
#[derive(Singleton)]
struct StringNamePool {
  strings_by_id: RwLock<Arena<StringPoolEntry>>,
}

/// An entry in the string pool.
struct StringPoolEntry {
  string: String,
  reference_count: usize,
}

impl Default for StringNamePool {
  fn default() -> Self {
    Self {
      strings_by_id: RwLock::new(Arena::default()),
    }
  }
}

impl StringNamePool {
  /// Looks up the string with the given ID.
  pub fn lookup(&self, id: ArenaIndex) -> Option<String> {
    let entries = self.strings_by_id.read().unwrap();

    entries.get(id).map(|entry| entry.string.clone())
  }

  /// Interns the given string and returns its ID.
  pub fn intern(&self, value: &str) -> ArenaIndex {
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
    self.strings_by_id.write().unwrap().insert(StringPoolEntry {
      string: value.to_owned(),
      reference_count: 1,
    })
  }

  /// Removes the string with the given ID.
  pub fn remove(&self, id: ArenaIndex) {
    let mut entries = self.strings_by_id.write().unwrap();

    if let Some(entry) = entries.get_mut(id) {
      entry.reference_count -= 1;

      if entry.reference_count == 0 {
        entries.remove(id);
      }
    }
  }
}

/// Implements owned and borrowed string conversions for a type.
#[macro_export]
macro_rules! impl_cow_string {
  ($type:ident) => {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct $type<'a>(std::borrow::Cow<'a, str>);

    impl<'a> From<&'a str> for $type<'a> {
      fn from(value: &'a str) -> Self {
        Self(std::borrow::Cow::Borrowed(value))
      }
    }

    impl<'a> From<String> for $type<'a> {
      fn from(value: String) -> Self {
        Self(std::borrow::Cow::Owned(value))
      }
    }
  };
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
