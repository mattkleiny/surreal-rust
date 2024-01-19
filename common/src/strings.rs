use std::{
  borrow::Cow,
  fmt::{Debug, Display},
  sync::{atomic::AtomicU32, RwLock},
};

use macros::Singleton;

use crate::collections::FastHashMap;

/// A custom string implementation for no_std targets.
#[cfg(feature = "no_std")]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct String {}

/// The default string implementation for std targets.
#[cfg(not(feature = "no_std"))]
pub type String = std::string::String;

/// Represents an interned string that can be used as a name.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StringName {
  id: u32,
}

impl Debug for StringName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(value) = StringNamePool::instance().lookup(self.id) {
      write!(f, "{:?}", value)
    } else {
      write!(f, "StringName({})", self.id)
    }
  }
}

impl Display for StringName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(value) = StringNamePool::instance().lookup(self.id) {
      write!(f, "{}", value)
    } else {
      write!(f, "")
    }
  }
}

impl From<&str> for StringName {
  fn from(value: &str) -> Self {
    let pool = StringNamePool::instance();

    StringName { id: pool.intern(value) }
  }
}

impl<'a> From<Cow<'a, str>> for StringName {
  fn from(value: Cow<'a, str>) -> Self {
    let pool = StringNamePool::instance();

    StringName {
      id: pool.intern(&value),
    }
  }
}

/// An internal global pool of interned strings.
#[derive(Singleton)]
struct StringNamePool {
  next_id: AtomicU32,
  strings_by_id: RwLock<FastHashMap<u32, String>>,
}

impl Default for StringNamePool {
  fn default() -> Self {
    Self {
      next_id: AtomicU32::new(1),
      strings_by_id: RwLock::new(FastHashMap::default()),
    }
  }
}

impl StringNamePool {
  /// Looks up the string with the given ID.
  pub fn lookup(&self, id: u32) -> Option<String> {
    let strings = self.strings_by_id.read().unwrap();

    if let Some(value) = strings.get(&id) {
      // TODO: remove this clone
      Some(value.clone())
    } else {
      None
    }
  }

  /// Interns the given string and returns its ID.
  pub fn intern(&self, value: &str) -> u32 {
    // we need to manually scan the strings here because we optimize
    // for the case where the string is already interned
    let strings = self.strings_by_id.read().unwrap();

    for (id, string) in strings.iter() {
      if string == value {
        return *id;
      }
    }

    // we need to drop the read lock before we can write to the map
    drop(strings);

    // insert the string into the map
    let mut strings = self.strings_by_id.write().unwrap();
    let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    strings.insert(id, value.to_owned());

    id
  }
}

/// Implements owned and borrowed string conversions for a type.
#[macro_export]
macro_rules! impl_cow_string {
  ($type:ident) => {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct $type<'a>(Cow<'a, str>);

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
    let name1: StringName = "test".into();
    let name2: StringName = "test".into();

    assert_eq!(name1, name2);
  }
}
