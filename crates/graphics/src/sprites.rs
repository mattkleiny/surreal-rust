//! Sprite management and rendering.

use common::{vec2, Angle, Color32, Mat2, Vec2};

use super::*;

/// The default number of sprites to allocate in a new batch.
const DEFAULT_SPRITE_COUNT: usize = 1024;

/// A region of a texture that can be used to draw a sprite.
pub trait Sprite {
  /// Returns the texture region for this sprite.
  fn to_region(&self) -> TextureRegion;
}

impl Sprite for Texture {
  #[inline]
  fn to_region(&self) -> TextureRegion {
    self.to_region()
  }
}

impl Sprite for TextureRegion {
  #[inline]
  fn to_region(&self) -> TextureRegion {
    self.clone()
  }
}

/// Options for drawing a sprite.
pub struct SpriteOptions {
  pub position: Vec2,
  pub rotation: Angle,
  pub scale: Vec2,
  pub color: Color32,
}

impl Default for SpriteOptions {
  fn default() -> Self {
    Self {
      position: Vec2::ZERO,
      rotation: Angle::ZERO,
      scale: Vec2::ONE,
      color: Color32::WHITE,
    }
  }
}

/// A fast and lightweight sprite batch renderer.
///
/// This batch pre-allocates an array of vertices and indices and re-uses them
/// for as many sprites as possible.
///
/// Batching is possible over 1 material and for sprites of the same texture.
/// If you need to render sprites from multiple textures, you should use a
/// [`MultiSpriteBatch`] instead, otherwise the batch will flush prior to
/// rendering the sprites from the new texture.
pub struct SpriteBatch {
  mesh: Mesh<SpriteVertex>,
  material: Option<Material>,
  vertices: Vec<SpriteVertex>,
  last_texture: Option<Texture>,
}

/// A specialized vertex for use in our sprite batch.
#[repr(C)]
#[derive(Clone, Debug, Vertex)]
struct SpriteVertex {
  #[vertex(2, F32)]
  pub position: Vec2,
  #[vertex(2, F32)]
  pub uv: Vec2,
  #[vertex(4, U8, normalize)]
  pub color: Color32,
}

impl SpriteBatch {
  /// Constructs a new [`SpriteBatch`] with a default capacity.
  pub fn new() -> Result<Self, MeshError> {
    Self::with_capacity(DEFAULT_SPRITE_COUNT)
  }

  /// Creates a new [`SpriteBatch`] with the given expected capacity.
  ///
  /// This will pre-allocate buffers to minimize reallocation costs.
  pub fn with_capacity(sprite_count: usize) -> Result<Self, MeshError> {
    // build standard quad indices ahead-of-time
    let vertices = Vec::with_capacity(sprite_count * 4);
    let indices = build_quad_indices(sprite_count);

    // create mesh, upload quad indices immediately
    let mut mesh = Mesh::new(BufferUsage::Dynamic)?;

    mesh.with_buffers(|_, buffer| {
      buffer.write_data(&indices);
    });

    Ok(Self {
      mesh,
      vertices,
      material: None,
      last_texture: None,
    })
  }

  /// Starts a new batch run with the given `Material`.
  pub fn begin(&mut self, material: &Material) {
    self.material = Some(material.clone());
    self.vertices.clear();
  }

  /// Draws a single sprite texture to the batch with the given options.
  pub fn draw_sprite(&mut self, sprite: &impl Sprite, options: &SpriteOptions) {
    // flush if we've reached capacity
    if self.vertices.len() + 4 >= self.vertices.capacity() {
      self.flush();
    }

    // flush if the texture has changed
    let region = sprite.to_region();
    if let Some(texture) = &self.last_texture {
      if texture.id() != region.texture.id() {
        self.flush();
        self.last_texture = Some(region.texture.clone());
      }
    } else if self.last_texture.is_none() {
      self.last_texture = Some(region.texture.clone());
    }

    let scale = vec2(
      region.size.x as f32 * options.scale.x,
      region.size.y as f32 * options.scale.y,
    );

    let angle = options.rotation;
    let translation = options.position;
    let transform = Mat2::from_scale_angle(scale, angle.into());
    let uv = region.calculate_uv();

    // add vertices
    self.vertices.push(SpriteVertex {
      position: translation + transform * vec2(-0.5, -0.5),
      color: options.color,
      uv: uv.top_left(),
    });

    self.vertices.push(SpriteVertex {
      position: translation + transform * vec2(-0.5, 0.5),
      color: options.color,
      uv: uv.bottom_left(),
    });

    self.vertices.push(SpriteVertex {
      position: translation + transform * vec2(0.5, 0.5),
      color: options.color,
      uv: uv.bottom_right(),
    });

    self.vertices.push(SpriteVertex {
      position: translation + transform * vec2(0.5, -0.5),
      color: options.color,
      uv: uv.top_right(),
    });
  }

  /// Flushes the batch to the GPU.
  pub fn flush(&mut self) {
    if self.vertices.is_empty() {
      return; // no vertices? no problem
    }

    // fetch the material out
    let material = &mut self.material;
    if material.is_none() {
      return;
    }
    let material = material.as_mut().unwrap();

    // prepare to draw
    let vertex_count = self.vertices.len();
    let sprite_count = vertex_count / 4;
    let index_count = sprite_count * 6;
    let mesh = &mut self.mesh;

    if let Some(texture) = &self.last_texture {
      material.set_texture("u_texture", texture, None);
    }

    // write vertices to mesh
    mesh.with_buffers(|vertices, _| {
      vertices.write_data(&self.vertices);
    });

    mesh.draw_sub(material, PrimitiveTopology::Triangles, vertex_count, index_count);

    self.vertices.clear();
  }
}

/// Fills a new buffer with standard quad indices.
fn build_quad_indices(sprite_count: usize) -> Vec<u32> {
  let mut indices = Vec::with_capacity(sprite_count * 6);
  let mut index = 0;

  for _ in 0..sprite_count {
    indices.push(index);
    indices.push(index + 1);
    indices.push(index + 2);
    indices.push(index + 2);
    indices.push(index + 3);
    indices.push(index);

    index += 4;
  }

  indices
}
