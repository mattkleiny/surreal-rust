//! Common compression/Decompression support.

use std::io::{Read, Write};

/// A trait for compressing data.
pub trait Compressor {
  /// Compresses the given data.
  fn compress(&self, data: &[u8]) -> Result<Vec<u8>, std::io::Error>;
}

/// A trait for decompressing data.
pub trait Decompressor {
  /// Decompresses the given data.
  fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, std::io::Error>;
}

/// The DEFLATE compression algorithm.
pub struct Deflate;

impl Compressor for Deflate {
  fn compress(&self, data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());

    encoder.write_all(data)?;

    encoder.finish()
  }
}

impl Decompressor for Deflate {
  fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut decoder = flate2::read::DeflateDecoder::new(data);
    let mut decompressed = Vec::new();

    decoder.read_to_end(&mut decompressed)?;

    Ok(decompressed)
  }
}

/// The Zlib compression algorithm.
pub struct Zlib;

impl Compressor for Zlib {
  fn compress(&self, data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());

    encoder.write_all(data)?;

    encoder.finish()
  }
}

impl Decompressor for Zlib {
  fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut decoder = flate2::read::ZlibDecoder::new(data);
    let mut decompressed = Vec::new();

    decoder.read_to_end(&mut decompressed)?;

    Ok(decompressed)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_compress_and_decompress_using_deflate() {
    let bytes = b"AAAAAABBBBBBCCCCCDDDDDEEEEE";

    let compressed = Deflate.compress(bytes).unwrap();
    let decompressed = Deflate.decompress(&compressed).unwrap();

    assert_eq!(bytes, decompressed.as_slice());
  }

  #[test]
  fn it_should_compress_and_decompress_using_zlib() {
    let bytes = b"AAAAAABBBBBBCCCCCDDDDDEEEEE";

    let compressed = Zlib.compress(bytes).unwrap();
    let decompressed = Zlib.decompress(&compressed).unwrap();

    assert_eq!(bytes, decompressed.as_slice());
  }
}
