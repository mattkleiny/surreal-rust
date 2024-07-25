use std::io::{BufRead, Seek, Write};

use crate::{Compressor, Decompressor, FileSystemError, ToVirtualPath};

/// Represents an error that occurred while reading or writing to a stream.
#[derive(Debug)]
pub enum StreamError {
  EndOfStream,
  InvalidData,
  GeneralFailure,
}

/// Allows a type to be imported from a VFS stream.
pub trait FromStream: Sized {
  type Error: From<StreamError> = StreamError;

  /// Imports the type from a path.
  fn from_path(path: impl ToVirtualPath) -> Result<Self, Self::Error> {
    let path = path.to_virtual_path();
    let mut stream = path.open_input_stream().map_err(|_| StreamError::GeneralFailure)?;

    Self::from_stream(&mut stream)
  }

  /// Imports the type from a byte array.
  fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
    let mut stream = std::io::Cursor::new(bytes);

    Self::from_stream(&mut stream)
  }

  /// Imports the type from a stream.
  fn from_stream(stream: &mut dyn InputStream) -> Result<Self, Self::Error>;
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

  /// Exports the type to a byte array.
  fn to_bytes(&self) -> Result<Vec<u8>, Self::Error> {
    let mut stream = std::io::Cursor::new(Vec::new());

    self.to_stream(&mut stream)?;

    Ok(stream.into_inner())
  }

  /// Exports the type to a stream.
  fn to_stream(&self, stream: &mut dyn OutputStream) -> Result<(), Self::Error>;
}

impl From<std::io::Error> for StreamError {
  #[inline]
  fn from(_: std::io::Error) -> Self {
    Self::EndOfStream
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

/// A stream for reading from some [`VirtualPath`].
pub trait InputStream: Seek + BufRead {
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

  /// Skips the given amount of bytes in the stream.
  fn skip_bytes(&mut self, amount: usize) -> Result<(), StreamError> {
    self.consume(amount);

    Ok(())
  }

  /// Reads a compressed buffer from the stream and decompresses it.
  fn read_decompress(&mut self, length: usize, algorithm: &dyn Decompressor) -> Result<Vec<u8>, StreamError> {
    let compressed = self.read_bytes(length)?;

    Ok(algorithm.decompress(&compressed)?)
  }

  fn read_u8(&mut self) -> Result<u8, StreamError>;
  fn read_char(&mut self) -> Result<char, StreamError>;
  fn read_u16(&mut self) -> Result<u16, StreamError>;
  fn read_u32(&mut self) -> Result<u32, StreamError>;
  fn read_u64(&mut self) -> Result<u64, StreamError>;
  fn read_u128(&mut self) -> Result<u128, StreamError>;
  fn read_usize(&mut self) -> Result<usize, StreamError>;
  fn read_i8(&mut self) -> Result<i8, StreamError>;
  fn read_i16(&mut self) -> Result<i16, StreamError>;
  fn read_i32(&mut self) -> Result<i32, StreamError>;
  fn read_i64(&mut self) -> Result<i64, StreamError>;
  fn read_i128(&mut self) -> Result<i128, StreamError>;
  fn read_isize(&mut self) -> Result<isize, StreamError>;
  fn read_f32(&mut self) -> Result<f32, StreamError>;
  fn read_f64(&mut self) -> Result<f64, StreamError>;
  fn read_string(&mut self) -> Result<String, StreamError>;
  fn read_bytes(&mut self, amount: usize) -> Result<Vec<u8>, StreamError>;
  fn to_buffer(self) -> Result<Vec<u8>, StreamError>;
  fn to_string(self) -> Result<String, StreamError>;
}

macro_rules! impl_read {
  ($self:expr, $buffer_size:expr, $result:ty) => {{
    let mut buffer = [0; $buffer_size];

    $self.read_exact(&mut buffer)?;

    Ok(<$result>::from_le_bytes(buffer))
  }};
}

/// Blanket implementation of [`InputStream`].
impl<T: BufRead + Seek> InputStream for T {
  #[inline]
  fn read_u8(&mut self) -> Result<u8, StreamError> {
    impl_read!(self, 1, u8)
  }

  #[inline]
  fn read_char(&mut self) -> Result<char, StreamError> {
    impl_read!(self, 1, u8).map(|value| value as char)
  }

  #[inline]
  fn read_u16(&mut self) -> Result<u16, StreamError> {
    impl_read!(self, 2, u16)
  }

  #[inline]
  fn read_u32(&mut self) -> Result<u32, StreamError> {
    impl_read!(self, 4, u32)
  }

  #[inline]
  fn read_u64(&mut self) -> Result<u64, StreamError> {
    impl_read!(self, 8, u64)
  }

  #[inline]
  fn read_u128(&mut self) -> Result<u128, StreamError> {
    impl_read!(self, 16, u128)
  }

  #[inline]
  fn read_usize(&mut self) -> Result<usize, StreamError> {
    impl_read!(self, size_of::<usize>(), usize)
  }

  #[inline]
  fn read_i8(&mut self) -> Result<i8, StreamError> {
    impl_read!(self, 1, i8)
  }

  #[inline]
  fn read_i16(&mut self) -> Result<i16, StreamError> {
    impl_read!(self, 2, i16)
  }

  #[inline]
  fn read_i32(&mut self) -> Result<i32, StreamError> {
    impl_read!(self, 4, i32)
  }

  #[inline]
  fn read_i64(&mut self) -> Result<i64, StreamError> {
    impl_read!(self, 8, i64)
  }

  #[inline]
  fn read_i128(&mut self) -> Result<i128, StreamError> {
    impl_read!(self, 16, i128)
  }

  #[inline]
  fn read_isize(&mut self) -> Result<isize, StreamError> {
    impl_read!(self, size_of::<isize>(), isize)
  }

  #[inline]
  fn read_f32(&mut self) -> Result<f32, StreamError> {
    impl_read!(self, 4, f32)
  }

  #[inline]
  fn read_f64(&mut self) -> Result<f64, StreamError> {
    impl_read!(self, 8, f64)
  }

  fn read_string(&mut self) -> Result<String, StreamError> {
    let length = self.read_u16()? as usize;
    let mut buffer = vec![0; length];

    self.read_exact(&mut buffer)?;

    Ok(String::from_utf8(buffer)?)
  }

  fn read_bytes(&mut self, amount: usize) -> Result<Vec<u8>, StreamError> {
    let mut buffer = vec![0; amount];

    self.read_exact(&mut buffer)?;

    Ok(buffer)
  }

  fn to_buffer(mut self) -> Result<Vec<u8>, StreamError> {
    let mut buffer = Vec::new();

    self.read_to_end(&mut buffer)?;

    Ok(buffer)
  }

  fn to_string(mut self) -> Result<String, StreamError> {
    let mut buffer = String::new();

    self.read_to_string(&mut buffer)?;

    Ok(buffer)
  }
}

/// A stream for writing to some [`VirtualPath`].
pub trait OutputStream: Seek + Write {
  /// Writes a compressed buffer to the stream.
  fn write_compress(&mut self, data: &[u8], algorithm: &dyn Compressor) -> Result<(), StreamError> {
    let compressed = algorithm.compress(data)?;

    self.write_bytes(&compressed)?;

    Ok(())
  }

  fn write_u8(&mut self, value: u8) -> Result<(), StreamError>;
  fn write_u16(&mut self, value: u16) -> Result<(), StreamError>;
  fn write_u32(&mut self, value: u32) -> Result<(), StreamError>;
  fn write_u64(&mut self, value: u64) -> Result<(), StreamError>;
  fn write_u128(&mut self, value: u128) -> Result<(), StreamError>;
  fn write_usize(&mut self, value: usize) -> Result<(), StreamError>;
  fn write_i8(&mut self, value: i8) -> Result<(), StreamError>;
  fn write_i16(&mut self, value: i16) -> Result<(), StreamError>;
  fn write_i32(&mut self, value: i32) -> Result<(), StreamError>;
  fn write_i64(&mut self, value: i64) -> Result<(), StreamError>;
  fn write_i128(&mut self, value: i128) -> Result<(), StreamError>;
  fn write_isize(&mut self, value: isize) -> Result<(), StreamError>;
  fn write_f32(&mut self, value: f32) -> Result<(), StreamError>;
  fn write_f64(&mut self, value: f64) -> Result<(), StreamError>;
  fn write_string(&mut self, value: &str) -> Result<(), StreamError>;
  fn write_bytes(&mut self, value: &[u8]) -> Result<(), StreamError>;
}

macro_rules! impl_write {
  ($self:expr, $type:ty, $value:expr) => {{
    let buffer = <$type>::to_le_bytes($value);

    $self.write_all(&buffer)?;

    Ok(())
  }};
}

/// Blanket implementation of [`OutputStream`].
impl<T: Write + Seek> OutputStream for T {
  #[inline]
  fn write_u8(&mut self, value: u8) -> Result<(), StreamError> {
    impl_write!(self, u8, value)
  }

  #[inline]
  fn write_u16(&mut self, value: u16) -> Result<(), StreamError> {
    impl_write!(self, u16, value)
  }

  #[inline]
  fn write_u32(&mut self, value: u32) -> Result<(), StreamError> {
    impl_write!(self, u32, value)
  }

  #[inline]
  fn write_u64(&mut self, value: u64) -> Result<(), StreamError> {
    impl_write!(self, u64, value)
  }

  #[inline]
  fn write_u128(&mut self, value: u128) -> Result<(), StreamError> {
    impl_write!(self, u128, value)
  }

  #[inline]
  fn write_usize(&mut self, value: usize) -> Result<(), StreamError> {
    impl_write!(self, usize, value)
  }

  #[inline]
  fn write_i8(&mut self, value: i8) -> Result<(), StreamError> {
    impl_write!(self, i8, value)
  }

  #[inline]
  fn write_i16(&mut self, value: i16) -> Result<(), StreamError> {
    impl_write!(self, i16, value)
  }

  #[inline]
  fn write_i32(&mut self, value: i32) -> Result<(), StreamError> {
    impl_write!(self, i32, value)
  }

  #[inline]
  fn write_i64(&mut self, value: i64) -> Result<(), StreamError> {
    impl_write!(self, i64, value)
  }

  #[inline]
  fn write_i128(&mut self, value: i128) -> Result<(), StreamError> {
    impl_write!(self, i128, value)
  }

  #[inline]
  fn write_isize(&mut self, value: isize) -> Result<(), StreamError> {
    impl_write!(self, isize, value)
  }

  #[inline]
  fn write_f32(&mut self, value: f32) -> Result<(), StreamError> {
    impl_write!(self, f32, value)
  }

  #[inline]
  fn write_f64(&mut self, value: f64) -> Result<(), StreamError> {
    impl_write!(self, f64, value)
  }

  fn write_string(&mut self, value: &str) -> Result<(), StreamError> {
    self.write_u16(value.len() as u16)?;
    self.write_bytes(value.as_bytes())?;

    Ok(())
  }

  fn write_bytes(&mut self, value: &[u8]) -> Result<(), StreamError> {
    self.write_all(value)?;

    Ok(())
  }
}

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
  fn it_should_compress_and_decompress() {
    let mut cursor = std::io::Cursor::new(vec![0x00; 64]);

    cursor.write_compress(b"Hello, world!", &crate::Deflate).unwrap();
    cursor.set_position(0);

    let decompressed = cursor.read_decompress(64, &crate::Deflate).unwrap();

    assert_eq!(decompressed, b"Hello, world!");
  }
}