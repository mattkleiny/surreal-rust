use crate::assets::{Asset, AssetContext, LoadableAsset};
use crate::graphics::{Color, Graphics, GraphicsError, Texture, TextureRegion};
use crate::maths::{Sliceable, Vector2};
use crate::vfs::Path;

/// Represents a sprite that may be batched via a `SpriteBatch`.
pub trait Sprite {
  /// Gets the `TextureRegion` for this sprite.
  fn get_sprite_texture(&self) -> &TextureRegion;
}

/// An efficiently batched array of `Sprite`s.
#[derive(Clone, Debug)]
pub struct SpriteBatch {
  vertices: Vec<SpriteVertex>,
  indices: Vec<u16>,
}

impl SpriteBatch {
  pub fn new(graphics: &mut impl Graphics) -> Result<Self, GraphicsError> {
    Ok(SpriteBatch {
      vertices: Vec::new(),
      indices: Vec::new(),
    })
  }

  /// Pushes sprite geometry into the batch.
  pub fn push(&mut self, position: Vector2<f32>, rotation: f32, sprite: &impl Sprite) {}

  /// Flushes the sprite batch to the given batch target.
  pub fn flush(&mut self, graphics: &mut impl Graphics) {}
}

/// Vertex definition for our sprite.
#[derive(Clone, Debug)]
struct SpriteVertex {
  pub pos: Vector2<f32>,
  pub uv: Vector2<f32>,
  pub color: Color,
}

/// A sheet of multiple sprites from a single source.
pub struct SpriteSheet {
  texture: Asset<Texture>,
}

impl SpriteSheet {
  /// Gets a single sprite `TextureRegion` from the sprite sheet.
  pub fn get_sprite(&self, x: u32, y: u32) -> TextureRegion {
    let regions = self.texture.subdivide((16, 16));

    unimplemented!()
  }
}

impl LoadableAsset for SpriteSheet {
  fn load(path: &impl AsRef<Path>, context: &mut impl AssetContext) -> Self {
    SpriteSheet { texture: Asset::load(path, context) }
  }
}
