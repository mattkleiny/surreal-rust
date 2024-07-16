use super::*;

/// Represents an error that occurred while serializing or deserializing.
#[derive(Debug)]
pub enum SerializationError {
  FailedToSerialize,
  FailedToDeserialize,
}

/// Allows serialization to different types implicitly.
///
/// Implementors of this trait will gain access to basic
/// serialization formats for free via convenience methods.
pub trait Serializable: ::serde::Serialize + Sized {
  /// Serializes the object to a byte array.
  #[cfg(feature = "binary")]
  fn to_binary(&self) -> Result<Vec<u8>, SerializationError> {
    Ok(binary::serialize(self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to a binary file.
  #[cfg(feature = "binary")]
  fn to_binary_file(&self, path: impl ToVirtualPath) -> Result<(), SerializationError> {
    let mut stream = path
      .to_virtual_path()
      .open_output_stream()
      .map_err(|_| SerializationError::FailedToSerialize)?;

    self.to_binary_stream(&mut stream)
  }

  /// Serializes the type to a binary stream.
  #[cfg(feature = "binary")]
  fn to_binary_stream(&self, stream: &mut dyn super::OutputStream) -> Result<(), SerializationError> {
    Ok(binary::serialize_into(stream, self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to a `json` string.
  #[cfg(feature = "json")]
  fn to_json(&self) -> Result<String, SerializationError> {
    Ok(json::to_string(self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to disk in `json` format.
  #[cfg(feature = "json")]
  fn to_json_file(&self, path: impl ToVirtualPath) -> Result<(), SerializationError> {
    let mut stream = path
      .to_virtual_path()
      .open_output_stream()
      .map_err(|_| SerializationError::FailedToSerialize)?;

    self.to_json_stream(&mut stream)
  }

  /// Serializes the type to stream in `json` format.
  #[cfg(feature = "json")]
  fn to_json_stream(&self, stream: &mut dyn super::OutputStream) -> Result<(), SerializationError> {
    Ok(json::to_writer(stream, self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to a `ron` string.
  #[cfg(feature = "ron")]
  fn to_ron(&self) -> Result<String, SerializationError> {
    Ok(ron::to_string(self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to disk in `ron` format.
  #[cfg(feature = "ron")]
  fn to_ron_file(&self, path: impl ToVirtualPath) -> Result<(), SerializationError> {
    let mut stream = path
      .to_virtual_path()
      .open_output_stream()
      .map_err(|_| SerializationError::FailedToSerialize)?;

    let string = ron::to_string(self).map_err(|_| SerializationError::FailedToSerialize)?;

    stream
      .write_all(string.as_bytes())
      .map_err(|_| SerializationError::FailedToSerialize)?;

    Ok(())
  }

  /// Serializes the type to stream in `ron` format.
  #[cfg(feature = "ron")]
  fn to_ron_stream(&self, stream: &mut dyn super::OutputStream) -> Result<(), SerializationError> {
    Ok(ron::ser::to_writer(stream, self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to a `toml` string.
  #[cfg(feature = "toml")]
  fn to_toml(&self) -> Result<String, SerializationError> {
    Ok(toml::to_string(self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to disk in `toml` format.
  #[cfg(feature = "toml")]
  fn to_toml_file(&self, path: impl ToVirtualPath) -> Result<(), SerializationError> {
    let mut stream = path
      .to_virtual_path()
      .open_output_stream()
      .map_err(|_| SerializationError::FailedToSerialize)?;

    let string = toml::to_string(self).map_err(|_| SerializationError::FailedToSerialize)?;

    stream
      .write_all(string.as_bytes())
      .map_err(|_| SerializationError::FailedToSerialize)?;

    Ok(())
  }

  /// Serializes the type to stream in `toml` format.
  #[cfg(feature = "toml")]
  fn to_toml_stream(&self, stream: &mut dyn super::OutputStream) -> Result<(), SerializationError> {
    let string = toml::to_string(self).map_err(|_| SerializationError::FailedToSerialize)?;

    stream
      .write_all(string.as_bytes())
      .map_err(|_| SerializationError::FailedToSerialize)?;

    Ok(())
  }

  /// Serializes the type to a `yaml` string.
  #[cfg(feature = "yaml")]
  fn to_yaml(&self) -> Result<String, SerializationError> {
    Ok(yaml::to_string(self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to disk in `yaml` format.
  #[cfg(feature = "yaml")]
  fn to_yaml_file(&self, path: impl ToVirtualPath) -> Result<(), SerializationError> {
    let mut stream = path
      .to_virtual_path()
      .open_output_stream()
      .map_err(|_| SerializationError::FailedToSerialize)?;

    Ok(yaml::to_writer(&mut stream, self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to stream in `yaml` format.
  #[cfg(feature = "yaml")]
  fn to_yaml_stream(&self, stream: &mut dyn super::OutputStream) -> Result<(), SerializationError> {
    Ok(yaml::to_writer(stream, self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to a `xml` string.
  #[cfg(feature = "xml")]
  fn to_xml(&self) -> Result<String, SerializationError> {
    Ok(xml::to_string(self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to disk in `xml` format.
  #[cfg(feature = "xml")]
  fn to_xml_file(&self, path: impl ToVirtualPath) -> Result<(), SerializationError> {
    let mut stream = path
      .to_virtual_path()
      .open_output_stream()
      .map_err(|_| SerializationError::FailedToSerialize)?;

    Ok(xml::to_writer(&mut stream, self).map_err(|_| SerializationError::FailedToSerialize)?)
  }

  /// Serializes the type to stream in `xml` format.
  #[cfg(feature = "xml")]
  fn to_xml_stream(&self, stream: &mut dyn super::OutputStream) -> Result<(), SerializationError> {
    Ok(xml::to_writer(stream, self).map_err(|_| SerializationError::FailedToSerialize)?)
  }
}

/// Allows deserialization from different types implicitly.
///
/// Implementors of this trait will gain access to basic
/// deserialization formats for free via convenience methods.
pub trait Deserializable: for<'de> ::serde::Deserialize<'de> + Sized {
  /// Deserializes the object from a byte array.
  #[cfg(feature = "binary")]
  fn from_binary(raw: &[u8]) -> Result<Self, SerializationError> {
    Ok(binary::deserialize(raw).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes the type from a binary file.
  #[cfg(feature = "binary")]
  fn from_binary_file(path: impl ToVirtualPath) -> Result<Self, SerializationError> {
    let mut stream = path
      .to_virtual_path()
      .open_input_stream()
      .map_err(|_| SerializationError::FailedToDeserialize)?;

    Ok(binary::deserialize_from(&mut stream).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes the object from a binary stream.
  #[cfg(feature = "binary")]
  fn from_binary_stream(reader: &mut dyn super::InputStream) -> Result<Self, SerializationError> {
    Ok(binary::deserialize_from(reader).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `json` string.
  #[cfg(feature = "json")]
  fn from_json(raw: &str) -> Result<Self, SerializationError> {
    Ok(json::from_str(raw).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `json` file.
  #[cfg(feature = "json")]
  fn from_json_file(path: impl ToVirtualPath) -> Result<Self, SerializationError> {
    let mut stream = path
      .to_virtual_path()
      .open_input_stream()
      .map_err(|_| SerializationError::FailedToDeserialize)?;

    Ok(json::from_reader(&mut stream).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `json` stream.
  #[cfg(feature = "json")]
  fn from_json_stream(reader: &mut dyn super::InputStream) -> Result<Self, SerializationError> {
    Ok(json::from_reader(reader).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `ron` string.
  #[cfg(feature = "ron")]
  fn from_ron(raw: &str) -> Result<Self, SerializationError> {
    Ok(ron::from_str(raw).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `ron` file.
  #[cfg(feature = "ron")]
  fn from_ron_file(path: impl ToVirtualPath) -> Result<Self, SerializationError> {
    let stream = path
      .to_virtual_path()
      .open_input_stream()
      .map_err(|_| SerializationError::FailedToDeserialize)?;

    let string = stream
      .to_string()
      .map_err(|_| SerializationError::FailedToDeserialize)?;

    Ok(ron::from_str(&string).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `ron` stream.
  #[cfg(feature = "ron")]
  fn from_ron_stream(stream: &mut dyn super::InputStream) -> Result<Self, SerializationError> {
    let string = stream
      .to_string()
      .map_err(|_| SerializationError::FailedToDeserialize)?;

    Ok(ron::from_str(&string).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `toml` string.
  #[cfg(feature = "toml")]
  fn from_toml(raw: &str) -> Result<Self, SerializationError> {
    Ok(toml::from_str(raw).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `toml` file.
  #[cfg(feature = "toml")]
  fn from_toml_file(path: impl ToVirtualPath) -> Result<Self, SerializationError> {
    let stream = path
      .to_virtual_path()
      .open_input_stream()
      .map_err(|_| SerializationError::FailedToDeserialize)?;

    let string = stream
      .to_string()
      .map_err(|_| SerializationError::FailedToDeserialize)?;

    Ok(toml::from_str(&string).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `toml` stream.
  #[cfg(feature = "toml")]
  fn from_toml_stream(stream: &mut dyn super::InputStream) -> Result<Self, SerializationError> {
    let string = stream
      .to_string()
      .map_err(|_| SerializationError::FailedToDeserialize)?;

    Ok(toml::from_str(&string).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `yaml` string.
  #[cfg(feature = "yaml")]
  fn from_yaml(raw: &str) -> Result<Self, SerializationError> {
    Ok(yaml::from_str(raw).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `yaml` file.
  #[cfg(feature = "yaml")]
  fn from_yaml_file(path: impl ToVirtualPath) -> Result<Self, SerializationError> {
    let mut stream = path
      .to_virtual_path()
      .open_input_stream()
      .map_err(|_| SerializationError::FailedToDeserialize)?;

    Ok(yaml::from_reader(&mut stream).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `yaml` stream.
  #[cfg(feature = "yaml")]
  fn from_yaml_stream(stream: &mut dyn super::InputStream) -> Result<Self, SerializationError> {
    Ok(yaml::from_reader(stream).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `xml` string.
  #[cfg(feature = "xml")]
  fn from_xml(raw: &str) -> Result<Self, SerializationError> {
    Ok(xml::from_str(raw).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `xml` file.
  #[cfg(feature = "xml")]
  fn from_xml_file(path: impl ToVirtualPath) -> Result<Self, SerializationError> {
    let mut stream = path
      .to_virtual_path()
      .open_input_stream()
      .map_err(|_| SerializationError::FailedToDeserialize)?;

    Ok(xml::from_reader(&mut stream).map_err(|_| SerializationError::FailedToDeserialize)?)
  }

  /// Deserializes from the given `xml` stream.
  #[cfg(feature = "xml")]
  fn from_xml_stream(stream: &mut dyn super::InputStream) -> Result<Self, SerializationError> {
    xml::from_reader(stream).map_err(|_| SerializationError::FailedToDeserialize)
  }
}

/// Blanket implementation of all [`Serializable`] types.
impl<T> Serializable for T where T: ::serde::Serialize {}

/// Blanket implementation of all [`Deserializable`] types.
impl<T> Deserializable for T where T: for<'de> ::serde::Deserialize<'de> {}
