//! GPU texture abstractions.


#[derive(Clone, Debug)]
pub enum TextureData {
  U8(Vec<u8>),
  U16(Vec<u16>),
}


#[derive(Copy, Clone, Debug)]
pub enum TextureFormat {
  RGB8,
  RGBA8,
}

impl TextureFormat {
  #[inline]
  pub fn channels(self) -> usize {
    match self {
      TextureFormat::RGB8 => 3,
      TextureFormat::RGBA8 => 4,
    }
  }
}
