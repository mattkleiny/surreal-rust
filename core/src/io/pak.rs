//! A binary packed file format for efficient asset streaming

use std::collections::HashMap;

use crate::io::{BinarySerializable, InputStream, OutputStream};
use crate::utilities::Size;

/// Compression formats for packed file entries.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Compression {
  None,
}

/// A packed file.
pub struct PackedFile<'a> {
  header: Header,
  pub stream: &'a mut dyn InputStream,
}

impl<'a> PackedFile<'a> {
  /// Loads a [`PackedFile`] from the given [`InputStream`].
  pub fn load(stream: &'a mut dyn InputStream) -> crate::Result<Self> {
    let header = Header::read_from(stream)?;

    Ok(Self { header, stream })
  }
}

/// Header data for a packed file.
#[derive(Clone, Debug)]
struct Header {
  pub entries: Vec<Entry>,
}

impl Header {
  /// Computes the [`Size`] of the entry.
  pub fn size(&self) -> Size {
    self.entries.iter().map(|entry| entry.size).sum()
  }
}

impl BinarySerializable for Header {
  fn read_from(stream: &mut dyn InputStream) -> crate::Result<Self> {
    let entry_count = stream.read_u32()?;
    let mut entries = Vec::with_capacity(entry_count as usize);

    for _ in 0..entry_count {
      entries.push(Entry::read_from(stream)?);
    }

    Ok(Self { entries })
  }

  fn write_to(&self, stream: &mut dyn OutputStream) -> crate::Result<()> {
    stream.write_u32(self.entries.len() as u32)?;

    for entry in &self.entries {
      entry.write_to(stream)?;
    }

    Ok(())
  }
}

/// A packed file entry.
#[derive(Clone, Debug)]
struct Entry {
  pub name: String,
  pub size: Size,
  pub offset: Size,
  pub metadata: HashMap<String, String>,
  pub compression: Compression,
}

impl BinarySerializable for Entry {
  fn read_from(stream: &mut dyn InputStream) -> crate::Result<Self> {
    let name = stream.read_string()?;
    let size = Size::from(stream.read_u32()?);
    let offset = Size::from(stream.read_u32()?);
    let metadata_count = stream.read_u32()?;
    let mut metadata = HashMap::with_capacity(metadata_count as usize);

    for _ in 0..metadata_count {
      let key = stream.read_string()?;
      let value = stream.read_string()?;

      metadata.insert(key, value);
    }

    Ok(Self {
      name,
      size,
      offset,
      metadata,
      compression: Compression::None,
    })
  }

  fn write_to(&self, stream: &mut dyn OutputStream) -> crate::Result<()> {
    stream.write_string(&self.name)?;
    stream.write_u32(self.size.as_bytes() as u32)?;
    stream.write_u32(self.offset.as_bytes() as u32)?;
    stream.write_u32(self.metadata.len() as u32)?;

    for (key, value) in &self.metadata {
      stream.write_string(key)?;
      stream.write_string(value)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::io::VirtualPath;

  use super::*;

  #[test]
  fn packed_file_should_save_and_load_to_binary() {
    let header = Header {
      entries: vec![
        Entry {
          name: "Test 1".to_string(),
          size: Size::from_bytes(108),
          offset: Size::from_bytes(0),
          metadata: Default::default(),
          compression: Compression::None,
        },
        Entry {
          name: "Test 2".to_string(),
          size: Size::from_kilobytes(0.8),
          offset: Size::from_bytes(108),
          metadata: Default::default(),
          compression: Compression::None,
        },
      ],
    };

    let path = VirtualPath::parse("memory://test.pak");

    // write to file
    let mut stream = path.open_output_stream().unwrap();
    header.write_to(&mut stream).unwrap();

    // read from file
    let mut stream = path.open_input_stream().unwrap();
    let header = Header::read_from(&mut stream).unwrap();

    println!("{:#?}", header);
  }
}
