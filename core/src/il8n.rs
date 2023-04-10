//! Internationalization (i18n) support.
//!
//! This module provides a simple way to localize your application text.

use crate::collections::FastHashMap;

/// A string that can be localized.
///
/// This is a wrapper around a string that can be localized. It is used to
/// ensure that all strings that need to be localized are marked as such.
///
/// The key is the key used to look up the localized string in the localization
/// database at runtime.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Il8nString {
  key: String,
}

impl std::fmt::Debug for Il8nString {
  fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(fmt, "\"{}\"", self.key)
  }
}

impl serde::Serialize for Il8nString {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    self.key.serialize(serializer)
  }
}

impl<'de> serde::Deserialize<'de> for Il8nString {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    Ok(Self {
      key: String::deserialize(deserializer)?,
    })
  }
}

impl From<&str> for Il8nString {
  fn from(value: &str) -> Self {
    Self {
      key: value.to_owned(),
    }
  }
}

/// A system that manages localization.
///
/// This system is responsible for managing localization. It is responsible for
/// loading the localization database and resolving strings at runtime.
///
/// The localization database is a simple key-value store. The key is the string
/// that is used in the code, and the value is the localized string.
#[derive(Default)]
pub struct Il8nManager {
  strings: FastHashMap<String, String>,
}

impl Il8nManager {
  /// Resolves a string to its localized version.
  pub fn resolve<'a>(&'a self, string: &'a Il8nString) -> &'a str {
    self.strings.get(&string.key).unwrap_or(&string.key)
  }

  /// Inserts a new string into the localization database.
  pub fn insert(&mut self, key: &str, value: &str) {
    self.strings.insert(key.to_owned(), value.to_owned());
  }
}
