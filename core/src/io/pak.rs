//! A binary packed file format for efficient asset streaming

use std::collections::HashMap;

use crate::io::{BinarySerializable, InputStream, OutputStream};
use crate::utilities::Size;

/// A packed file.
pub struct PackedFile<'a> {
  header: PackedFileHeader,
  stream: &'a mut dyn InputStream,
}

impl<'a> PackedFile<'a> {
  /// Loads a [`PackedFile`] from the given [`InputStream`].
  pub fn load(stream: &'a mut dyn InputStream) -> crate::Result<Self> {
    let header = PackedFileHeader::read_from(stream)?;

    Ok(Self { header, stream })
  }
}

/// Header data for a packed file.
#[derive(Clone, Debug)]
pub struct PackedFileHeader {
  pub size: Size,
  pub entries: Vec<PackedFileEntry>,
}

impl BinarySerializable for PackedFileHeader {
  fn read_from(stream: &mut dyn InputStream) -> crate::Result<Self> {
    let size = Size::from(stream.read_u32()?);
    let entry_count = stream.read_u32()?;
    let mut entries = Vec::with_capacity(entry_count as usize);

    for _ in 0..entry_count {
      entries.push(PackedFileEntry::read_from(stream)?);
    }

    Ok(Self { size, entries })
  }

  fn write_to(&self, stream: &mut dyn OutputStream) -> crate::Result<()> {
    stream.write_u32(self.size.as_bytes() as u32)?;
    stream.write_u32(self.entries.len() as u32)?;

    for entry in &self.entries {
      entry.write_to(stream)?;
    }

    Ok(())
  }
}

/// A packed file entry.
#[derive(Clone, Debug)]
pub struct PackedFileEntry {
  pub name: String,
  pub data_size: Size,
  pub data_offset: Size,
  pub metadata: HashMap<String, String>,
  pub compression: Compression,
}

impl BinarySerializable for PackedFileEntry {
  fn read_from(stream: &mut dyn InputStream) -> crate::Result<Self> {
    let name = stream.read_string()?;
    let data_size = Size::from(stream.read_u32()?);
    let data_offset = Size::from(stream.read_u32()?);
    let metadata_count = stream.read_u32()?;
    let mut metadata = HashMap::with_capacity(metadata_count as usize);

    for _ in 0..metadata_count {
      let key = stream.read_string()?;
      let value = stream.read_string()?;

      metadata.insert(key, value);
    }

    Ok(Self {
      name,
      data_size,
      data_offset,
      metadata,
      compression: Compression::None,
    })
  }

  fn write_to(&self, stream: &mut dyn OutputStream) -> crate::Result<()> {
    stream.write_string(&self.name)?;
    stream.write_u32(self.data_size.as_bytes() as u32)?;
    stream.write_u32(self.data_offset.as_bytes() as u32)?;
    stream.write_u32(self.metadata.len() as u32)?;

    for (key, value) in &self.metadata {
      stream.write_string(key)?;
      stream.write_string(value)?;
    }

    Ok(())
  }
}

/// Compression formats for packed file entries.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Compression {
  None,
  Gzip,
  Zlib,
  Brotli,
}

#[cfg(test)]
mod tests {
  use crate::io::VirtualPath;

  use super::*;

  // #[test]
  // fn packed_file_should_save_to_binary() {
  //   let header = PackedFileHeader {
  //     size: Size::from_kilobytes(2.5),
  //     entries: vec![
  //       PackedFileEntry {
  //         name: "Test 1".to_string(),
  //         data_size: Size::from_bytes(108),
  //         data_offset: Size::from_bytes(0),
  //         metadata: Default::default(),
  //         compression: Compression::None,
  //       },
  //       PackedFileEntry {
  //         name: "Test 2".to_string(),
  //         data_size: Size::from_kilobytes(0.8),
  //         data_offset: Size::from_bytes(108),
  //         metadata: Default::default(),
  //         compression: Compression::None,
  //       },
  //     ],
  //   };
  //
  //   let path = VirtualPath::parse("memory://test.pak");
  //   let mut stream = path.open_output_stream().unwrap();
  //
  //   header.write_to(&mut stream).unwrap();
  // }

  // #[test]
  // fn packed_file_should_load_from_binary() {
  //   let path = VirtualPath::parse("memory://test.pak");
  //   let mut stream = path.open_input_stream().unwrap();
  //
  //   let header = PackedFileHeader::read_from(&mut stream).unwrap();
  //
  //   println!("{:#?}", header);
  // }
}
