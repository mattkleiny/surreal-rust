use super::VirtualPath;

/// Allows serialization to different types implicitly.
///
/// Implementors of this trait will gain access to basic
/// serialization formats for free via convenience methods.
pub trait Serializable: serde::Serialize + Sized {
  /// Serializes the object to a byte array.
  #[cfg(feature = "binary")]
  fn to_binary(&self) -> crate::Result<Vec<u8>> {
    Ok(binary::serialize(self)?)
  }

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

  /// Serializes the type to a `xml` string.
  #[cfg(feature = "xml")]
  fn to_xml(&self) -> crate::Result<String> {
    Ok(xml::to_string(self)?)
  }

  /// Serializes the type to a binary file.
  fn to_binary_file(&self, path: &VirtualPath) -> crate::Result<()> {
    let mut stream = path.open_output_stream()?;

    Ok(binary::serialize_into(&mut stream, self)?)
  }

  /// Serializes the type to disk in `json` format.
  #[cfg(feature = "json")]
  fn to_json_file(&self, path: impl Into<VirtualPath>) -> crate::Result<()> {
    let mut stream = path.into().open_output_stream()?;

    Ok(json::to_writer(&mut stream, self)?)
  }

  /// Serializes the type to disk in `xml` format.
  #[cfg(feature = "xml")]
  fn to_xml_file(&self, path: impl Into<VirtualPath>) -> crate::Result<()> {
    let mut stream = path.into().open_output_stream()?;

    Ok(xml::to_writer(&mut stream, self)?)
  }

  /// Serializes the type to disk in `ron` format.
  #[cfg(feature = "ron")]
  fn to_ron_file(&self, path: impl Into<VirtualPath>) -> crate::Result<()> {
    let mut stream = path.into().open_output_stream()?;
    let string = ron::to_string(self)?;

    stream.write_all(string.as_bytes())?;

    Ok(())
  }

  /// Serializes the type to disk in `toml` format.
  #[cfg(feature = "toml")]
  fn to_toml_file(&self, path: impl Into<VirtualPath>) -> crate::Result<()> {
    let mut stream = path.into().open_output_stream()?;
    let string = toml::to_string(self)?;

    stream.write_all(string.as_bytes())?;

    Ok(())
  }

  /// Serializes the type to disk in `yaml` format.
  #[cfg(feature = "yaml")]
  fn to_yaml_file(&self, path: impl Into<VirtualPath>) -> crate::Result<()> {
    let mut stream = path.into().open_output_stream()?;

    Ok(yaml::to_writer(&mut stream, self)?)
  }
}

/// Blanket implementation of all [`Serializable`] types.
impl<T> Serializable for T where T: serde::Serialize {}

/// Allows deserialization from different types implicitly.
///
/// Implementors of this trait will gain access to basic
/// deserialization formats for free via convenience methods.
pub trait Deserializable: for<'de> serde::Deserialize<'de> + Sized {
  /// Deserializes the object from a byte array.
  #[cfg(feature = "binary")]
  fn from_binary(data: &[u8]) -> crate::Result<Self> {
    Ok(binary::deserialize(data)?)
  }

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

  /// Deserializes from the given `xml` string.
  #[cfg(feature = "xml")]
  fn from_xml(yaml: &str) -> crate::Result<Self> {
    Ok(xml::from_str(yaml)?)
  }

  /// Deserializes the type from a binary file.
  #[cfg(feature = "binary")]
  fn from_binary_file(path: &VirtualPath) -> crate::Result<Self> {
    let mut stream = path.open_input_stream()?;

    Ok(binary::deserialize_from(&mut stream)?)
  }

  /// Deserializes from the given `json` file.
  #[cfg(feature = "json")]
  fn from_json_file(path: impl Into<VirtualPath>) -> crate::Result<Self> {
    let mut stream = path.into().open_input_stream()?;

    Ok(json::from_reader(&mut stream)?)
  }

  /// Deserializes from the given `ron` file.
  #[cfg(feature = "ron")]
  fn from_ron_file(path: impl Into<VirtualPath>) -> crate::Result<Self> {
    let mut stream = path.into().open_input_stream()?;
    let mut string = String::new();

    stream.read_to_string(&mut string)?;

    Ok(ron::from_str(&string)?)
  }

  /// Deserializes from the given `toml` file.
  #[cfg(feature = "toml")]
  fn from_toml_file(path: impl Into<VirtualPath>) -> crate::Result<Self> {
    let mut stream = path.into().open_input_stream()?;
    let mut string = String::new();

    stream.read_to_string(&mut string)?;

    Ok(toml::from_str(&string)?)
  }

  /// Deserializes from the given `yaml` file.
  #[cfg(feature = "yaml")]
  fn from_yaml_file(path: impl Into<VirtualPath>) -> crate::Result<Self> {
    let mut stream = path.into().open_input_stream()?;

    Ok(yaml::from_reader(&mut stream)?)
  }

  /// Deserializes from the given `xml` file.
  #[cfg(feature = "xml")]
  fn from_xml_file(path: impl Into<VirtualPath>) -> crate::Result<Self> {
    let mut stream = path.into().open_input_stream()?;

    Ok(xml::from_reader(&mut stream)?)
  }
}

/// Blanket implementation of all [`Deserializable`] types.
impl<T> Deserializable for T where T: for<'de> serde::Deserialize<'de> {}
