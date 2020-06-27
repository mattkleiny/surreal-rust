use crate::RID;

#[derive(Debug, Eq, PartialEq)]
pub struct Texture {
  id: RID,
  flags: TextureFlags,
}

#[repr(u8)]
#[derive(BitFlags, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFlags {
  Clamp = 1 << 0,
}