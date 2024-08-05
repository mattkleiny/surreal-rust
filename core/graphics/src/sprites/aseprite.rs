//! A utility for loading and parsing Aseprite files.

use common::{Color32, FromStream, InputStream, StreamError, Zlib};

/// An error that can occur when loading an Aseprite file.
#[derive(Debug)]
pub enum AsepriteError {
  EndOfStream,
  InvalidEncoding,
  InvalidMagicNumber,
}

/// An Aseprite file.
#[derive(Debug)]
pub struct AsepriteFile {
  pub header: AsepriteHeader,
  pub frames: Vec<AsepriteFrame>,
}

/// The header of an Aseprite file.
#[derive(Debug)]
pub struct AsepriteHeader {
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
/// A single frame of an Aseprite file.
#[derive(Debug)]
pub struct AsepriteFrame {
  pub length: u32,
  pub chunk_count: u32,
  pub duration_ms: u16,
  pub chunks: Vec<AsepriteChunk>,
}

/// Possible chunks that can be found in an Aseprite file.
#[derive(Debug)]
pub enum AsepriteChunk {
  /// A layer chunk - contains information about a single layer.
  Layer {
    flags: u16,
    layer_type: LayerType,
    child_level: u16,
    default_width: u16,
    default_height: u16,
    blend_mode: LayerBlendMode,
    opacity: u8,
    name: String,
  },
  /// A cel chunk - contains the pixel data for a single frame.
  Cel {
    layer_index: u16,
    offset_x: i16,
    offset_y: i16,
    opacity: u8,
    cel_type: CelType,
    width: u16,
    height: u16,
    pixels: Vec<CelPixel>,
  },
  /// A linked cel chunk that points to another cel's pixel data.
  LinkedCel {
    layer_index: u16,
    offset_x: i16,
    offset_y: i16,
    opacity: u8,
    cel_type: CelType,
    linked_index: u16,
  },
  /// A cel extra chunk - contains additional information about a cel.
  CelExtra {
    flags: u32,
    precise_x: f64,
    precise_y: f64,
    width: f64,
    height: f64,
  },
  /// A frame tags chunk - contains information about tags in the animation.
  Tags { count: u16, tags: Vec<AsepriteTag> },
  /// A palette chunk - contains information about the color palette.
  Palette {
    palette_size: u32,
    first_color: u32,
    last_color: u32,
    colors: Vec<AsepritePaletteEntry>,
  },
}

/// A tag in an Aseprite file.
#[derive(Debug)]
pub struct AsepriteTag {
  pub from_frame: u16,
  pub to_frame: u16,
  pub loop_type: LoopType,
  pub loop_count: u16,
  pub color: Color32,
  pub name: String,
}

/// An entry in an Aseprite palette.
#[derive(Debug)]
pub struct AsepritePaletteEntry {
  pub flags: u16,
  pub color: Color32,
  pub name: Option<String>,
}

/// Possible types of chunks that can be found in an Aseprite file.
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum ChunkType {
  Layer = 0x2004,
  Cel = 0x2005,
  CelExtra = 0x2006,
  Tags = 0x2018,
  Palette = 0x2019,
  Other(u16),
}

/// Possible types of cels that can be found in an Aseprite file.
#[derive(Debug, Clone, Copy)]
pub enum CelType {
  Raw,
  Linked,
  Compressed,
}

/// A single pixel in a cel.
#[derive(Debug, Clone, Copy)]
pub enum CelPixel {
  Rgba(Color32),
  Mono { index: u8 },
  Indexed { index: u8 },
}

/// Possible color depths for an Aseprite file.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ColorDepth {
  Rgba = 32,
  Grayscale = 16,
  Indexed = 8,
}

/// Possible loop modes for an animation.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LoopType {
  Forward,
  Reverse,
  PingPong,
}

/// Possible types of layers in an Aseprite file.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LayerType {
  Normal,
  Folder,
}

/// Possible blend modes for a layer.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LayerBlendMode {
  Normal,
  Multiply,
  Screen,
  Overlay,
  Darken,
  Lighten,
  ColorDodge,
  ColorBurn,
  HardLight,
  SoftLight,
  Difference,
  Exclusion,
  Hue,
  Saturation,
  Color,
  Luminosity,
}

impl FromStream for AsepriteFile {
  type Error = AsepriteError;

  fn from_stream(stream: &mut dyn InputStream) -> Result<Self, Self::Error> {
    let header = AsepriteHeader::from_stream(stream)?;
    let mut frames = Vec::with_capacity(header.frame_count as usize);

    for _ in 0..header.frame_count {
      frames.push(AsepriteFrame::from_stream(&header, stream)?);
    }

    Ok(Self { header, frames })
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

impl AsepriteFrame {
  /// Parses an [`AsepriteFrame`] from the given stream.
  pub fn from_stream(header: &AsepriteHeader, stream: &mut dyn InputStream) -> Result<Self, AsepriteError> {
    let length = stream.read_u32()?;
    let magic_number = stream.read_u16()?;

    if magic_number != 0xF1FA {
      return Err(AsepriteError::InvalidMagicNumber);
    }

    let mut chunk_count = stream.read_u16()? as u32;
    let duration_ms = stream.read_u16()?;

    stream.skip_bytes(2)?;

    // unpack the chunk count
    let new_chunk_count = stream.read_u32()?;
    if new_chunk_count != 0 {
      chunk_count = new_chunk_count;
    }

    // decode each chunk
    let mut chunks = Vec::with_capacity(chunk_count as usize);

    for _ in 0..chunk_count {
      if let Some(chunk) = AsepriteChunk::from_stream(stream, header)? {
        chunks.push(chunk);
      }
    }

    Ok(Self {
      length,
      chunk_count,
      duration_ms,
      chunks,
    })
  }
}

impl AsepriteChunk {
  /// Parses an [`AsepriteChunk`] from the given stream.
  pub fn from_stream(
    stream: &mut dyn InputStream,
    header: &AsepriteHeader,
  ) -> Result<Option<AsepriteChunk>, AsepriteError> {
    const HEADER_SIZE: u32 = 6;

    let length = stream.read_u32()?;
    let chunk_type = stream.read_u16()?.into();

    match chunk_type {
      ChunkType::Layer => {
        let flags = stream.read_u16()?;
        let layer_type = stream.read_u16()?.into();
        let child_level = stream.read_u16()?;
        let default_width = stream.read_u16()?;
        let default_height = stream.read_u16()?;
        let blend_mode = stream.read_u16()?.into();
        let opacity = stream.read_u8()?;

        stream.skip_bytes(3)?;

        let name = stream.read_string()?;

        Ok(Some(AsepriteChunk::Layer {
          flags,
          layer_type,
          child_level,
          default_width,
          default_height,
          blend_mode,
          opacity,
          name,
        }))
      }
      ChunkType::Cel => {
        const CEL_HEADER_SIZE: u32 = 9 + 7;

        let layer_index = stream.read_u16()?;
        let offset_x = stream.read_i16()?;
        let offset_y = stream.read_i16()?;
        let opacity = stream.read_u8()?;
        let cel_type = stream.read_u16()?.into();

        stream.skip_bytes(7)?;

        match cel_type {
          CelType::Raw => {
            let width = stream.read_u16()?;
            let height = stream.read_u16()?;
            let size = width * height;
            let raw_pixels = stream.read_bytes(size as usize)?;
            let pixels = CelPixel::decode(&raw_pixels, header.color_depth);

            Ok(Some(AsepriteChunk::Cel {
              layer_index,
              offset_x,
              offset_y,
              opacity,
              cel_type,
              width,
              height,
              pixels,
            }))
          }
          CelType::Compressed => {
            let width = stream.read_u16()?;
            let height = stream.read_u16()?;
            let size = length - HEADER_SIZE - CEL_HEADER_SIZE - 4;
            let raw_pixels = stream.read_decompress(size as usize, &Zlib)?;
            let pixels = CelPixel::decode(&raw_pixels, header.color_depth);

            Ok(Some(AsepriteChunk::Cel {
              layer_index,
              offset_x,
              offset_y,
              opacity,
              cel_type,
              width,
              height,
              pixels,
            }))
          }
          CelType::Linked => {
            let linked_index = stream.read_u16()?;

            Ok(Some(AsepriteChunk::LinkedCel {
              layer_index,
              offset_x,
              offset_y,
              opacity,
              cel_type,
              linked_index,
            }))
          }
        }
      }
      ChunkType::CelExtra => {
        let flags = stream.read_u32()?;
        let precise_x = stream.read_f64()?;
        let precise_y = stream.read_f64()?;
        let width = stream.read_f64()?;
        let height = stream.read_f64()?;

        Ok(Some(AsepriteChunk::CelExtra {
          flags,
          precise_x,
          precise_y,
          width,
          height,
        }))
      }
      ChunkType::Tags => {
        let count = stream.read_u16()?;
        let mut tags = Vec::with_capacity(count as usize);

        stream.skip_bytes(8)?;

        for _ in 0..count {
          let from_frame = stream.read_u16()?;
          let to_frame = stream.read_u16()?;
          let loop_type = stream.read_u8()?.into();
          let loop_count = stream.read_u16()?;

          stream.skip_bytes(6)?;

          let color = Color32::from_packed(stream.read_u32()?);
          let name = stream.read_string()?;

          tags.push(AsepriteTag {
            from_frame,
            to_frame,
            loop_type,
            loop_count,
            color,
            name,
          });
        }

        Ok(Some(AsepriteChunk::Tags { count, tags }))
      }
      ChunkType::Palette => {
        let palette_size = stream.read_u32()?;
        let first_color = stream.read_u32()?;
        let last_color = stream.read_u32()?;

        stream.skip_bytes(8)?;

        let mut colors = Vec::with_capacity(palette_size as usize);

        for _ in 0..palette_size {
          let flags = stream.read_u16()?;
          let color = Color32::from_packed(stream.read_u32()?);

          let name = if flags & 1 != 0 {
            Some(stream.read_string()?)
          } else {
            None
          };

          colors.push(AsepritePaletteEntry { flags, color, name });
        }

        Ok(Some(AsepriteChunk::Palette {
          palette_size,
          first_color,
          last_color,
          colors,
        }))
      }
      _ => {
        // if the chunk is not recognized, skip it
        stream.skip_bytes(length as usize - HEADER_SIZE as usize)?;
        Ok(None)
      }
    }
  }
}

impl CelPixel {
  /// Decodes a slice of bytes into a list of [`CelPixel`]s.
  pub fn decode(data: &[u8], color_depth: ColorDepth) -> Vec<Self> {
    match color_depth {
      ColorDepth::Rgba => {
        let mut pixels = Vec::with_capacity(data.len() / 4);

        for i in (0..data.len()).step_by(4) {
          let r = data[i];
          let g = data[i + 1];
          let b = data[i + 2];
          let a = data[i + 3];

          pixels.push(CelPixel::Rgba(Color32::rgba(r, g, b, a)));
        }

        pixels
      }
      ColorDepth::Grayscale => {
        let mut pixels = Vec::with_capacity(data.len() / 2);

        for i in (0..data.len()).step_by(2) {
          pixels.push(CelPixel::Mono { index: data[i] });
        }

        pixels
      }
      ColorDepth::Indexed => {
        let mut pixels = Vec::with_capacity(data.len());

        for index in data {
          pixels.push(CelPixel::Indexed { index: *index });
        }

        pixels
      }
    }
  }
}

impl From<u16> for ChunkType {
  fn from(value: u16) -> Self {
    match value {
      0x2004 => Self::Layer,
      0x2005 => Self::Cel,
      0x2006 => Self::CelExtra,
      0x2018 => Self::Tags,
      0x2019 => Self::Palette,
      _ => Self::Other(value),
    }
  }
}

impl From<u16> for LayerType {
  fn from(value: u16) -> Self {
    match value {
      0 => Self::Normal,
      1 => Self::Folder,
      _ => panic!("Invalid layer type: {}", value),
    }
  }
}

impl From<u8> for LoopType {
  fn from(value: u8) -> Self {
    match value {
      0 => Self::Forward,
      1 => Self::Reverse,
      2 => Self::PingPong,
      _ => panic!("Invalid loop type: {}", value),
    }
  }
}

impl From<u16> for LayerBlendMode {
  fn from(value: u16) -> Self {
    match value {
      0 => Self::Normal,
      1 => Self::Multiply,
      2 => Self::Screen,
      3 => Self::Overlay,
      4 => Self::Darken,
      5 => Self::Lighten,
      6 => Self::ColorDodge,
      7 => Self::ColorBurn,
      8 => Self::HardLight,
      9 => Self::SoftLight,
      10 => Self::Difference,
      11 => Self::Exclusion,
      12 => Self::Hue,
      13 => Self::Saturation,
      14 => Self::Color,
      15 => Self::Luminosity,
      _ => panic!("Invalid blend mode: {}", value),
    }
  }
}

impl From<u16> for CelType {
  fn from(value: u16) -> Self {
    match value {
      0 => Self::Raw,
      1 => Self::Linked,
      2 => Self::Compressed,
      _ => panic!("Invalid cel type: {}", value),
    }
  }
}

impl From<u16> for ColorDepth {
  fn from(value: u16) -> Self {
    match value {
      32 => Self::Rgba,
      16 => Self::Grayscale,
      8 => Self::Indexed,
      _ => panic!("Invalid color depth: {}", value),
    }
  }
}

impl From<StreamError> for AsepriteError {
  #[inline(always)]
  fn from(_: StreamError) -> Self {
    Self::InvalidEncoding
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_load_a_simple_aseprite_file() {
    let file = AsepriteFile::from_path(&"assets/test.ase").unwrap();

    println!("{:#?}", file);

    assert_eq!(file.header.width, 16);
    assert_eq!(file.header.height, 16);
  }
}
