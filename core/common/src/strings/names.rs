use std::{
  fmt::{Debug, Display},
  ptr::NonNull,
  sync::RwLock,
};

use crate::{Arena, Singleton};

crate::impl_arena_index!(pub(crate) StringId, "Identifies a string in a string pool.");

/// A trait for objects that can be converted to a [`StringName`].
pub trait ToStringName {
  /// Converts the value to a [`StringName`].
  fn to_string_name(&self) -> StringName;
}

/// Represents an interned string that can be cheaply passed around the engine.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, Hash)]
pub struct StringName {
  id: StringId,
}

/// An internal global pool of strings.
#[derive(Default, Singleton)]
struct StringNamePool {
  strings_by_id: RwLock<Arena<StringId, NonNull<str>>>,
}

impl StringName {
  /// Creates a new string name from a string reference.
  pub fn new(value: &str) -> Self {
    Self {
      id: unsafe { intern_string(value) },
    }
  }

  /// Creates a new string name from a string ID.
  pub(crate) fn from_id(id: StringId) -> Self {
    Self { id }
  }

  /// Returns the ID of the string.
  pub(crate) fn id(&self) -> StringId {
    self.id
  }
}

/// Converts a string reference to a string name.
impl From<&str> for StringName {
  #[inline]
  fn from(value: &str) -> Self {
    Self::new(value)
  }
}

/// Converts a string to a string name.
impl From<String> for StringName {
  #[inline]
  fn from(value: String) -> Self {
    Self::new(&value)
  }
}

/// Converts a string name to an owned string.
impl From<StringName> for String {
  #[inline]
  fn from(value: StringName) -> Self {
    value.to_string()
  }
}

/// Allows a string named to be interpreted as a string reference.
impl AsRef<str> for StringName {
  fn as_ref(&self) -> &str {
    lookup_string(self.id).expect("String name not found in pool")
  }
}

/// Allows a string reference to be converted to a string name.
impl<R: AsRef<str>> ToStringName for R {
  fn to_string_name(&self) -> StringName {
    StringName::new(self.as_ref())
  }
}

/// Compares two string names.
impl PartialEq<StringName> for StringName {
  #[inline(always)]
  fn eq(&self, other: &StringName) -> bool {
    self.id == other.id
  }
}

/// Compares a string reference with a string name.
impl PartialEq<StringName> for &str {
  fn eq(&self, other: &StringName) -> bool {
    if let Some(value) = lookup_string(other.id) {
      value == *self
    } else {
      false
    }
  }
}

/// Compares a string reference with a string name.
impl PartialEq<&str> for StringName {
  fn eq(&self, other: &&str) -> bool {
    if let Some(value) = lookup_string(self.id) {
      value == *other
    } else {
      false
    }
  }
}

/// Compares a string with a string name.
impl PartialEq<String> for StringName {
  fn eq(&self, other: &String) -> bool {
    if let Some(value) = lookup_string(self.id) {
      value == other
    } else {
      false
    }
  }
}

impl Debug for StringName {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(value) = lookup_string(self.id) {
      write!(formatter, "{:?}", value)
    } else {
      write!(formatter, "StringName({:?})", self.id)
    }
  }
}

/// Pretty-prints a string name.
impl Display for StringName {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Some(value) = lookup_string(self.id) {
      write!(formatter, "{}", value)
    } else {
      write!(formatter, "")
    }
  }
}

/// Interns the given string and returns its ID.
///
/// If the string is already interned, its reference count is incremented.
/// Otherwise, it is inserted into the pool.
unsafe fn intern_string(value: &str) -> StringId {
  let entries = StringNamePool::instance().strings_by_id.read().unwrap();

  for (id, entry) in entries.enumerate() {
    if entry.as_ref() == value {
      return id;
    }
  }

  // we need to drop the read lock before we can write to the map
  drop(entries);

  let mut entries = StringNamePool::instance().strings_by_id.write().unwrap();
  let raw = value.to_owned().leak(); // leak the string to make it static

  entries.insert(NonNull::new(raw).unwrap())
}

/// Looks up the string with the given ID.
///
/// If the string is not interned, returns `None`.
fn lookup_string(id: StringId) -> Option<&'static str> {
  let entries = unsafe { StringNamePool::instance().strings_by_id.read().unwrap() };

  if let Some(entry) = entries.get(id) {
    Some(unsafe { entry.as_ref() })
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_string_name_should_intern_similar_strings() {
    unsafe {
      let id1 = intern_string("test");
      let id2 = intern_string("test");
      let id3 = intern_string("test2");

      assert_eq!(id1, id2);
      assert_ne!(id1, id3);
    }
  }

  #[test]
  fn test_string_name_should_convert_from_reference() {
    let name1 = StringName::new("test");
    let name2 = StringName::new("test");

    assert_eq!(name1, name2);
  }
}
