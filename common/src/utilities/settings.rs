//! Abstracts over reading/writing settings from/to various sources.

#[cfg(target_os = "windows")]
pub use windows::*;

/// Represents key for a setting of type T.
pub struct SettingKey<T> {
  name: &'static str,
  phantom: std::marker::PhantomData<T>,
}

impl<T> SettingKey<T> {
  /// Creates a new setting key with the given name and default value.
  pub const fn new(name: &'static str) -> Self {
    Self {
      name,
      phantom: std::marker::PhantomData,
    }
  }
}

#[cfg(target_os = "windows")]
mod windows {
  //! Allows reading/writing settings from the Windows registry.
  use winreg::{
    enums::HKEY_CURRENT_USER,
    types::{FromRegValue, ToRegValue},
  };

  use super::*;

  /// A settings provider that uses the Windows registry.
  pub struct RegistrySettings {
    root: winreg::RegKey,
  }

  impl Default for RegistrySettings {
    fn default() -> Self {
      Self::open("Surreal")
    }
  }

  impl RegistrySettings {
    /// Opens the given registry key.
    pub fn open(key: &str) -> Self {
      Self {
        root: winreg::RegKey::predef(HKEY_CURRENT_USER)
          .open_subkey("Software")
          .expect("Failed to open registry key")
          .create_subkey(key)
          .expect("Failed to open registry key")
          .0,
      }
    }

    /// Gets the value of the given setting.
    pub fn get<T, K>(&self, key: K) -> Option<T>
    where
      T: FromRegValue,
      K: Into<SettingKey<T>>,
    {
      let key = key.into();
      let value = self.root.get_value(key.name).ok()?;

      Some(value)
    }

    /// Sets the value of the given setting.
    pub fn set<T, K>(&mut self, key: K, value: T)
    where
      T: ToRegValue,
      K: Into<SettingKey<T>>,
    {
      let key = key.into();

      self.root.set_value(key.name, &value).unwrap();
    }
  }
}
