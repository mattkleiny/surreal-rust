use std::io::{BufRead, Seek, Write};

use crate::{BlockableFuture, Compressor, Decompressor, FileSystemError, Task, ToVirtualPath};

/// Represents an error that occurred while working with a stream.
#[derive(Debug)]
pub enum StreamError {
  EndOfStream,
  InvalidData,
  GeneralFailure,
}

impl std::fmt::Display for StreamError {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(formatter, "{}", match self {
      Self::EndOfStream => "End of stream",
      Self::InvalidData => "Invalid data",
      Self::GeneralFailure => "General failure",
    })
  }
}

impl std::error::Error for StreamError {}

/// Allows a type to be imported from a VFS stream.
pub trait FromStream: Sized {
  type Error: From<StreamError> = StreamError;

  /// Imports the type from a path.
  fn from_path(path: impl ToVirtualPath) -> Result<Self, Self::Error> {
    let path = path.to_virtual_path();
    let mut stream = path.open_input_stream().map_err(|_| StreamError::GeneralFailure)?;

    Self::from_stream(&mut stream)
  }

  /// Imports the type from a path asynchronously.
  async fn from_path_async(path: impl ToVirtualPath) -> Result<Self, Self::Error> {
    let path = path.to_virtual_path();
    let mut stream = path.open_input_stream().map_err(|_| StreamError::GeneralFailure)?;

    Self::from_stream_async(&mut stream).await
  }

  /// Imports the type from a byte array.
  fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
    let mut cursor = std::io::Cursor::new(bytes);

    Self::from_stream(&mut cursor)
  }

  /// Imports the type from a byte array asynchronously.
  async fn from_bytes_async(bytes: &[u8]) -> Result<Self, Self::Error> {
    let mut cursor = std::io::Cursor::new(bytes);

    Self::from_stream_async(&mut cursor).await
  }

  /// Imports the type from a stream.
  fn from_stream(stream: &mut dyn InputStream) -> Result<Self, Self::Error> {
    Self::from_stream_async(stream).block()
  }

  /// Imports the type from a stream asynchronously.
  async fn from_stream_async(stream: &mut dyn InputStream) -> Result<Self, Self::Error>;
}

/// Allows a type to be exported to a VFS stream.
pub trait ToStream: Sized {
  type Error: From<StreamError> = StreamError;

  /// Exports the type to a path.
  fn to_path(&self, path: impl ToVirtualPath) -> Result<(), Self::Error> {
    let path = path.to_virtual_path();
    let mut stream = path.open_output_stream().map_err(|_| StreamError::GeneralFailure)?;

    self.to_stream(&mut stream)
  }

  /// Exports the type to a path asynchronously.
  async fn to_path_async(&self, path: impl ToVirtualPath) -> Result<(), Self::Error> {
    let path = path.to_virtual_path();
    let mut stream = path.open_output_stream().map_err(|_| StreamError::GeneralFailure)?;

    self.to_stream_async(&mut stream).await
  }

  /// Exports the type to a byte array.
  fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
    let mut cursor = std::io::Cursor::new(Vec::new());

    self.to_stream(&mut cursor)?;

    Ok(cursor.into_inner())
  }

  /// Exports the type to a byte array asynchronously.
  async fn to_bytes_async(&self) -> Result<Vec<u8>, Self::Error> {
    let mut cursor = std::io::Cursor::new(Vec::new());

    self.to_stream_async(&mut cursor).await?;

    Ok(cursor.into_inner())
  }

  /// Exports the type to a stream.
  fn to_stream(&self, stream: &mut dyn OutputStream) -> Result<(), Self::Error>;

  /// Exports the type to a stream asynchronously.
  async fn to_stream_async(&self, stream: &mut dyn OutputStream) -> Result<(), Self::Error> {
    self.to_stream(stream)
  }
}

impl From<std::io::Error> for StreamError {
  #[inline]
  fn from(_: std::io::Error) -> Self {
    Self::EndOfStream
  }
}

impl From<std::string::ParseError> for StreamError {
  #[inline]
  fn from(_: std::string::ParseError) -> Self {
    Self::InvalidData
  }
}

impl From<std::num::ParseIntError> for StreamError {
  #[inline]
  fn from(_: std::num::ParseIntError) -> Self {
    Self::InvalidData
  }
}

impl From<std::string::FromUtf8Error> for StreamError {
  #[inline]
  fn from(_: std::string::FromUtf8Error) -> Self {
    Self::EndOfStream
  }
}

impl From<FileSystemError> for StreamError {
  #[inline]
  fn from(_: FileSystemError) -> Self {
    Self::GeneralFailure
  }
}

/// Implements a specialized read method for the given type.
macro_rules! impl_read {
  ($name:tt, $buffer_size:expr, $result:ty) => {
    #[inline]
    fn $name(&mut self) -> Result<$result, StreamError> {
      let mut buffer = [0; $buffer_size];

      self.read_exact(&mut buffer)?;

      Ok(<$result>::from_le_bytes(buffer))
    }
  };
}

/// A stream for reading from some [`VirtualPath`].
pub trait InputStream: Seek + BufRead {
  impl_read!(read_u8, 1, u8);
  impl_read!(read_u16, 2, u16);
  impl_read!(read_u32, 4, u32);
  impl_read!(read_u64, 8, u64);
  impl_read!(read_u128, 16, u128);
  impl_read!(read_usize, size_of::<usize>(), usize);
  impl_read!(read_i8, 1, i8);
  impl_read!(read_i16, 2, i16);
  impl_read!(read_i32, 4, i32);
  impl_read!(read_i64, 8, i64);
  impl_read!(read_i128, 16, i128);
  impl_read!(read_isize, size_of::<isize>(), isize);
  impl_read!(read_f32, 4, f32);
  impl_read!(read_f64, 8, f64);

  /// Reads a single character from the stream.
  fn read_char(&mut self) -> Result<char, StreamError> {
    self.read_u8().map(|value| value as char)
  }

  /// Reads a buffer from the stream.
  fn read_bytes(&mut self, amount: usize) -> Result<Vec<u8>, StreamError> {
    let mut buffer = vec![0; amount];

    self.read_exact(&mut buffer)?;

    Ok(buffer)
  }

  /// Reads a compressed buffer from the stream and decompresses it.
  fn read_decompress(&mut self, length: usize, algorithm: &dyn Decompressor) -> Result<Vec<u8>, StreamError> {
    let compressed = self.read_bytes(length)?;

    Ok(algorithm.decompress(&compressed)?)
  }

  /// Reads a string from the stream.
  fn read_string(&mut self) -> Result<String, StreamError> {
    let length = self.read_u16()? as usize;
    let mut buffer = vec![0; length];

    self.read_exact(&mut buffer)?;

    Ok(String::from_utf8(buffer)?)
  }

  /// Reads a line from the stream.
  fn read_string_line(&mut self) -> Result<String, StreamError> {
    let mut buffer = Vec::new();

    self.read_until(b'\n', &mut buffer)?;

    if buffer.last() == Some(&b'\n') {
      buffer.pop();
    }

    if buffer.last() == Some(&b'\r') {
      buffer.pop();
    }

    Ok(String::from_utf8(buffer)?)
  }

  /// Converts the stream into a buffer.
  fn to_buffer(self) -> Result<Vec<u8>, StreamError>;

  /// Converts the stream into a buffer asynchronously.
  fn to_buffer_async(self) -> Task<Result<Vec<u8>, StreamError>>;

  /// Converts the stream into a string.
  fn to_string(self) -> Result<String, StreamError>;

  /// Converts the stream into a buffer asynchronously.
  fn to_string_async(self) -> Task<Result<String, StreamError>>;

  /// Skips the given amount of bytes in the stream.
  fn skip_bytes(&mut self, amount: usize) -> Result<(), StreamError> {
    self.consume(amount);

    Ok(())
  }

  /// Skips any whitespace characters in the stream.
  fn skip_whitespace(&mut self) -> Result<(), StreamError> {
    loop {
      match self.fill_buf()?.first() {
        Some(byte) if byte.is_ascii_whitespace() => {
          self.consume(1);
        }
        _ => break,
      }
    }

    Ok(())
  }
}

/// Blanket implementation of [`InputStream`].
impl<T: BufRead + Seek> InputStream for T {
  fn to_buffer(mut self) -> Result<Vec<u8>, StreamError> {
    let mut buffer = Vec::new();

    self.read_to_end(&mut buffer)?;

    Ok(buffer)
  }

  fn to_buffer_async(self) -> Task<Result<Vec<u8>, StreamError>> {
    Task::from_result(self.to_buffer())
  }

  fn to_string(mut self) -> Result<String, StreamError> {
    let mut buffer = String::new();

    self.read_to_string(&mut buffer)?;

    Ok(buffer)
  }

  fn to_string_async(self) -> Task<Result<String, StreamError>> {
    Task::from_result(self.to_string())
  }
}

/// Implements a specialized write method for the given type.
macro_rules! impl_write {
  ($name:tt, $type:ty) => {
    #[inline]
    fn $name(&mut self, value: $type) -> Result<(), StreamError> {
      let buffer = <$type>::to_le_bytes(value);

      self.write_all(&buffer)?;

      Ok(())
    }
  };
}

/// A stream for writing to some [`VirtualPath`].
pub trait OutputStream: Seek + Write {
  impl_write!(write_u8, u8);
  impl_write!(write_u16, u16);
  impl_write!(write_u32, u32);
  impl_write!(write_u64, u64);
  impl_write!(write_u128, u128);
  impl_write!(write_usize, usize);
  impl_write!(write_i8, i8);
  impl_write!(write_i16, i16);
  impl_write!(write_i32, i32);
  impl_write!(write_i64, i64);
  impl_write!(write_i128, i128);
  impl_write!(write_isize, isize);
  impl_write!(write_f32, f32);
  impl_write!(write_f64, f64);

  /// Writes a string to the stream.
  fn write_string(&mut self, value: &str) -> Result<(), StreamError> {
    self.write_u16(value.len() as u16)?;
    self.write_bytes(value.as_bytes())?;

    Ok(())
  }

  /// Writes a string to the stream asynchronously.
  fn write_string_async(&mut self, value: &str) -> Task<Result<(), StreamError>> {
    Task::from_result(self.write_string(value))
  }

  /// Writes a buffer to the stream.
  fn write_bytes(&mut self, value: &[u8]) -> Result<(), StreamError> {
    self.write_all(value)?;

    Ok(())
  }

  /// Writes a buffer to the stream asynchronously.
  fn write_bytes_async(&mut self, value: &[u8]) -> Task<Result<(), StreamError>> {
    Task::from_result(self.write_bytes(value))
  }

  /// Writes a compressed buffer to the stream.
  fn write_compress(&mut self, data: &[u8], compressor: &dyn Compressor) -> Result<(), StreamError> {
    let compressed = compressor.compress(data)?;

    self.write_bytes(&compressed)?;

    Ok(())
  }

  /// Writes a compressed buffer to the stream asynchronously.
  fn write_compressed_async(&mut self, data: &[u8], compressor: &dyn Compressor) -> Task<Result<(), StreamError>> {
    Task::from_result(self.write_compress(data, compressor))
  }
}

/// Blanket implementation of [`OutputStream`].
impl<T: Write + Seek> OutputStream for T {}

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! impl_read_write_test {
    ($name:ident, $read:tt, $write:tt, $value:expr) => {
      #[test]
      pub fn $name() {
        let buffer = vec![0x00; 64];
        let mut cursor = std::io::Cursor::new(buffer);

        cursor.$write($value).unwrap();
        cursor.set_position(0);

        assert_eq!(cursor.$read().unwrap(), $value);
      }
    };
  }

  impl_read_write_test!(it_should_read_write_u8, read_u8, write_u8, 0xFF);
  impl_read_write_test!(it_should_read_write_u16, read_u16, write_u16, 0xFFFF);
  impl_read_write_test!(it_should_read_write_u32, read_u32, write_u32, 0xFFFFFFFF);
  impl_read_write_test!(it_should_read_write_u64, read_u64, write_u64, 0xFFFFFFFFFFFFFFFF);
  impl_read_write_test!(it_should_read_write_usize, read_usize, write_usize, usize::MAX);
  impl_read_write_test!(it_should_read_write_i8, read_i8, write_i8, -1);
  impl_read_write_test!(it_should_read_write_i16, read_i16, write_i16, -1);
  impl_read_write_test!(it_should_read_write_i32, read_i32, write_i32, -1);
  impl_read_write_test!(it_should_read_write_i64, read_i64, write_i64, -1);
  impl_read_write_test!(it_should_read_write_isize, read_isize, write_isize, isize::MIN);
  impl_read_write_test!(it_should_read_write_f32, read_f32, write_f32, -1.0);
  impl_read_write_test!(it_should_read_write_f64, read_f64, write_f64, -1.0);
  impl_read_write_test!(it_should_read_write_string, read_string, write_string, "Hello, world!");

  #[test]
  fn test_basic_async_read_write() {
    let buffer = vec![0x00; 4];
    let mut cursor = std::io::Cursor::new(buffer);

    cursor.write_bytes_async(&[7, 3, 3, 7]).block().unwrap();
    cursor.set_position(0);

    let result = cursor.to_buffer_async().block().unwrap();

    assert_eq!(result, &[7, 3, 3, 7]);
  }
}
