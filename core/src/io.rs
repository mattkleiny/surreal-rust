//! Input/output utilities and virtual file system.

use serde::{Deserialize, Serialize};

pub use pak::*;
pub use vfs::*;

mod pak;
mod vfs;

/// Allows serialization to different types implicitly.
///
/// Implementors of this trait will gain access to basic
/// serialization formats for free via convenience methods.
pub trait Serializable: Serialize {
  /// Serializes the type to a `json` string.
  #[cfg(feature = "json")]
  fn to_json(&self) -> crate::Result<String> {
    Ok(json::to_string(self)?)
  }

  /// Serializes the type to a `ron` string.
  #[cfg(feature = "ron")]
  fn to_ron(&self) -> crate::Result<String> {
    Ok(ron::to_string(self)?)
  }

  /// Serializes the type to a `toml` string.
  #[cfg(feature = "toml")]
  fn to_toml(&self) -> crate::Result<String> {
    Ok(toml::to_string(self)?)
  }

  /// Serializes the type to a `yaml` string.
  #[cfg(feature = "yaml")]
  fn to_yaml(&self) -> crate::Result<String> {
    Ok(yaml::to_string(self)?)
  }

  /// Serializes the type to disk in `json` format.
  #[cfg(feature = "json")]
  fn save_to_json(&self, path: impl Into<VirtualPath>) -> crate::Result<()> {
    let mut stream = path.into().open_output_stream()?;

    Ok(json::to_writer(&mut stream, self)?)
  }

  /// Serializes the type to disk in `ron` format.
  #[cfg(feature = "ron")]
  fn save_to_ron(&self, path: impl Into<VirtualPath>) -> crate::Result<()> {
    let mut stream = path.into().open_output_stream()?;
    let string = ron::to_string(self)?;

    stream.write_all(string.as_bytes())?;

    Ok(())
  }

  /// Serializes the type to disk in `toml` format.
  #[cfg(feature = "toml")]
  fn save_to_toml(&self, path: impl Into<VirtualPath>) -> crate::Result<()> {
    let mut stream = path.into().open_output_stream()?;
    let string = toml::to_string(self)?;

    stream.write_all(string.as_bytes())?;

    Ok(())
  }

  /// Serializes the type to disk in `yaml` format.
  #[cfg(feature = "yaml")]
  fn save_to_yaml(&self, path: impl Into<VirtualPath>) -> crate::Result<()> {
    let mut stream = path.into().open_output_stream()?;

    Ok(yaml::to_writer(&mut stream, self)?)
  }
}

/// Blanket implementation of [`Serializable`] for any [`Serialize`]-able type.
impl<T> Serializable for T where T: Serialize {}

/// Allows deserialization from different types implicitly.
///
/// Implementors of this trait will gain access to basic
/// deserialization formats for free via convenience methods.
pub trait Deserializable: for<'de> Deserialize<'de> {
  /// Deserializes from the given `json` string.
  #[cfg(feature = "json")]
  fn from_json(json: &str) -> crate::Result<Self> {
    Ok(json::from_str(json)?)
  }

  /// Deserializes from the given `ron` string.
  #[cfg(feature = "ron")]
  fn from_ron(toml: &str) -> crate::Result<Self> {
    Ok(ron::from_str(toml)?)
  }

  /// Deserializes from the given `toml` string.
  #[cfg(feature = "toml")]
  fn from_toml(toml: &str) -> crate::Result<Self> {
    Ok(toml::from_str(toml)?)
  }

  /// Deserializes from the given `yaml` string.
  #[cfg(feature = "yaml")]
  fn from_yaml(yaml: &str) -> crate::Result<Self> {
    Ok(yaml::from_str(yaml)?)
  }

  /// Deserializes from the given `json` file.
  #[cfg(feature = "json")]
  fn load_from_json(path: impl Into<VirtualPath>) -> crate::Result<Self> {
    let mut stream = path.into().open_input_stream()?;

    Ok(json::from_reader(&mut stream)?)
  }

  /// Deserializes from the given `ron` file.
  #[cfg(feature = "ron")]
  fn load_from_ron(path: impl Into<VirtualPath>) -> crate::Result<Self> {
    let mut stream = path.into().open_input_stream()?;
    let mut string = String::new();

    stream.read_to_string(&mut string)?;

    Ok(ron::from_str(&string)?)
  }

  /// Deserializes from the given `toml` file.
  #[cfg(feature = "toml")]
  fn load_from_toml(path: impl Into<VirtualPath>) -> crate::Result<Self> {
    let mut stream = path.into().open_input_stream()?;
    let mut string = String::new();

    stream.read_to_string(&mut string)?;

    Ok(toml::from_str(&string)?)
  }

  /// Deserializes from the given `yaml` file.
  #[cfg(feature = "yaml")]
  fn load_from_yaml(path: impl Into<VirtualPath>) -> crate::Result<Self> {
    let mut stream = path.into().open_input_stream()?;

    Ok(yaml::from_reader(&mut stream)?)
  }
}

/// Blanket implementation of [`Deserializable`] for any [`Deserialize`]-able type.
impl<T> Deserializable for T where T: for<'de> Deserialize<'de> {}
