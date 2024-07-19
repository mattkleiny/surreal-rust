//! A utility for loading and parsing Aseprite files.

use common::{FromStream, InputStream, StreamError};

/// An Aseprite file.
pub struct AsepriteFile {
  header: AsepriteHeader,
}

/// The header of an Aseprite file.
#[derive(Debug)]
struct AsepriteHeader {
  pub file_size: u32,
  pub frame_count: u16,
  pub width: u16,
  pub height: u16,
  pub color_depth: ColorDepth,
  pub flags: u32,
  pub speed: u16,
  pub transparent_color_index: u8,
  pub color_count: u16,
  pub pixel_width: u8,
  pub pixel_height: u8,
}

/// Possible color depths for an Aseprite file.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ColorDepth {
  Rgba = 32,
  Grayscale = 16,
  Indexed = 8,
}

/// An error that can occur when loading an Aseprite file.
#[derive(Debug)]
pub enum AsepriteError {
  EndOfStream,
  InvalidEncoding,
  InvalidMagicNumber,
}

impl From<u16> for ColorDepth {
  fn from(value: u16) -> Self {
    match value {
      32 => Self::Rgba,
      16 => Self::Grayscale,
      8 => Self::Indexed,
      _ => unreachable!(),
    }
  }
}

impl From<StreamError> for AsepriteError {
  fn from(_: StreamError) -> Self {
    Self::InvalidEncoding
  }
}

impl FromStream for AsepriteFile {
  type Error = AsepriteError;

  fn from_stream(stream: &mut dyn InputStream) -> Result<Self, Self::Error> {
    let header = AsepriteHeader::from_stream(stream)?;

    Ok(Self { header })
  }
}

impl AsepriteHeader {
  /// Reads an [`AsepriteHeader`] from the given stream.
  pub fn from_stream(stream: &mut dyn InputStream) -> Result<Self, AsepriteError> {
    // decode the header
    let file_size = stream.read_u32()?;
    let magic_number = stream.read_u16()?;

    if magic_number != 0xA5E0 {
      return Err(AsepriteError::InvalidMagicNumber);
    }

    let frame_count = stream.read_u16()?;
    let width = stream.read_u16()?;
    let height = stream.read_u16()?;
    let color_depth = stream.read_u16()?.into();
    let flags = stream.read_u32()?;
    let speed = stream.read_u16()?;

    stream.skip_bytes(8)?;

    let transparent_color_index = stream.read_u8()?;

    stream.skip_bytes(3)?;

    let color_count = stream.read_u16()?;
    let pixel_width = stream.read_u8()?;
    let pixel_height = stream.read_u8()?;

    stream.skip_bytes(92)?;

    Ok(AsepriteHeader {
      file_size,
      frame_count,
      width,
      height,
      color_depth,
      flags,
      speed,
      transparent_color_index,
      color_count,
      pixel_width,
      pixel_height,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_load_a_simple_aseprite_file() {
    let file = AsepriteFile::from_path("assets/test.ase").unwrap();

    assert_eq!(file.header.width, 16);
    assert_eq!(file.header.height, 16);
  }
}
