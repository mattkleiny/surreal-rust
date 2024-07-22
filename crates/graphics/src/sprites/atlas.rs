//! A sprite atlas utility

use common::{Color32, UVec2};

use super::*;

/// An atlas of sprites.
pub struct SpriteAtlas {
  region: TextureRegion,
  size: UVec2,
}

impl SpriteAtlas {
  /// Creates a new sprite atlas from a texture region.
  pub fn from_region(region: TextureRegion, size: UVec2) -> Self {
    Self { region, size }
  }

  /// Creates a new sprite atlas from a texture.
  pub fn from_texture(texture: &Texture, size: UVec2) -> Self {
    Self::from_region(texture.to_region(), size)
  }

  /// Gets the cell at the given coordinates.
  pub fn cell_at(&self, x: u32, y: u32) -> Option<TextureRegion> {
    if x >= self.region.size.x / self.size.x || y >= self.region.size.y / self.size.y {
      return None;
    }

    Some(self.region.slice(x, y, self.size.x, self.size.y))
  }

  /// Gets the cell at the given coordinates without bounds checking.
  ///
  /// # Safety
  /// This function is unsafe because it does not perform bounds checking, make
  /// sure that the coordinates are within the atlas.
  pub unsafe fn cell_at_unchecked(&self, x: u32, y: u32) -> TextureRegion {
    self.region.slice(x, y, self.size.x, self.size.y)
  }
}

/// A builder for creating sprite atlases.
pub struct SpriteAtlasBuilder {
  cell_size: UVec2,
  pixels: Vec<Color32>,
}

impl SpriteAtlasBuilder {
  /// Creates a new sprite atlas builder from a texture.
  pub fn new(cell_size: UVec2) -> Self {
    Self {
      cell_size,
      pixels: Vec::new(),
    }
  }

  /// Builds a texture.
  pub fn to_texture(self) -> Texture {
    todo!()
  }

  /// Builds the sprite atlas.
  pub fn to_sprite_atlas(self) -> SpriteAtlas {
    todo!()
  }
}

/// Calculates the nearest power of 2 for the given value.
fn nearest_power_of_2(value: u32) -> u32 {
  let mut result = 1;

  while result < value {
    result *= 2;
  }

  result
}
