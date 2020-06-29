use crate::assets::{Asset, AssetContext, LoadableAsset, Path};
use crate::graphics::{Color, Image};
use crate::maths::Vector2;

/// Represents a 2d texture.
#[derive(Debug, Eq, PartialEq)]
pub struct Texture {
  flags: TextureFlags,
}

impl Texture {
  pub fn width(&self) -> usize { unimplemented!() }
  pub fn height(&self) -> usize { unimplemented!() }

  /// Accesses the pixels of the `Texture`.
  pub fn pixels(&self) -> &[Color] {
    unimplemented!()
  }

  /// Mutably accesses the pixels of the `Texture`.
  pub fn pixels_mut(&mut self) -> &mut [Color] {
    unimplemented!()
  }
}

/// Represents a sub-region of a `Texture`.
pub struct TextureRegion {
  pub texture: Asset<Texture>,
  pub offset: Vector2<f32>,
  pub size: Vector2<f32>,
}

/// Flags for texture creation.
#[repr(u8)]
#[derive(BitFlags, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextureFlags {
  Clamp = 1 << 0,
}

impl LoadableAsset for Texture {
  fn load(path: impl AsRef<Path>, context: &mut impl AssetContext) -> Self {
    // TODO: actually implement me
    let image = Image::load(path, context);

    Texture {
      flags: TextureFlags::Clamp
    }
  }
}