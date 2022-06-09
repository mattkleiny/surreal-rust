//! Sprite management and rendering.
//!
//! Sprites are very common in projects, so we've a simple dedicated batch to support them.

use crate::maths::{vec2, Vector2};

use super::*;

/// The default number of sprites to allocate in a new batch.
const DEFAULT_SPRITE_COUNT: usize = 1024;

/// A fast and lightweight sprite batch renderer.
///
/// This batch pre-allocates an array of vertices and indices and re-uses them for as many
/// sprites as possible.
///
/// Batching is possible over 1 material and texture pair; each texture swap requires a flush
/// and so it's important to pre-sort sprites into batches by material and texture.
pub struct SpriteBatch {
  mesh: Mesh<SpriteVertex>,
  material: Option<Material>,
  textures: TexturePool,
  vertices: Vec<SpriteVertex>,
}

/// Options for drawing a sprite.
pub struct SpriteOptions {
  pub position: Vector2<f32>,
  pub scale: Vector2<f32>,
  pub color: Color32,
}

impl Default for SpriteOptions {
  fn default() -> Self {
    Self {
      position: Vector2::ZERO,
      scale: Vector2::ONE,
      color: Color32::WHITE,
    }
  }
}

/// A specialized vertex for use in our sprite batch.
///
/// Encodes a unique texture id representing which of the bound texture units is relevant for this sprite
#[derive(Copy, Clone, Debug)]
struct SpriteVertex {
  position: Vector2<f32>,
  uv: Vector2<f32>,
  color: Color32,
  texture_id: u8,
}

impl Vertex for SpriteVertex {
  #[rustfmt::skip]
  const DESCRIPTORS: &'static [VertexDescriptor] = &[
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 4, kind: VertexKind::U8, should_normalize: true },
    VertexDescriptor { count: 1, kind: VertexKind::U8, should_normalize: false },
  ];
}

impl SpriteBatch {
  /// Constructs a new [`SpriteBatch`].
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self::with_capacity(graphics, DEFAULT_SPRITE_COUNT)
  }

  /// Creates a new [`SpriteBatch`] with the given sprite capacity.
  pub fn with_capacity(graphics: &GraphicsServer, sprite_count: usize) -> Self {
    // build standard quad indices ahead-of-time
    let vertices = Vec::with_capacity(sprite_count * 4);
    let indices = build_quad_indices(sprite_count);

    // create mesh, upload quad indices immediately
    let mut mesh = Mesh::new(graphics, BufferUsage::Static);

    mesh.with_buffers(|_, buffer| {
      buffer.write_data(&indices);
    });

    Self {
      mesh,
      material: None,
      textures: TexturePool::default(),
      vertices,
    }
  }

  /// Starts a new batch run with the given `Material`.
  #[profiling::function]
  pub fn begin(&mut self, material: &Material) {
    self.material = Some(material.clone());
    self.vertices.clear();
    self.textures.clear();
  }

  /// Draws a line of text to the batch with the given options
  #[profiling::function]
  pub fn draw_text(&mut self, font: &impl Font, text: &str, options: &SpriteOptions) {
    let size = font.measure_size(text);
    let mut position = options.position;

    // TODO: fix centering when scale is applied
    position.x -= size.0 as f32 * options.scale.x / 2.;
    position.y -= size.1 as f32 * options.scale.y / 2.;

    for character in text.chars() {
      if let Some(glyph) = font.get_glyph(character) {
        self.draw_sprite(
          &glyph,
          &SpriteOptions {
            position,
            scale: options.scale,
            color: options.color,
          },
        );

        position.x += glyph.size.x as f32 * options.scale.x;
      }
    }
  }
  /// Draws a single sprite texture to the batch with the given options.
  #[profiling::function]
  pub fn draw_sprite<'a>(&mut self, region: &'a TextureRegion, options: &SpriteOptions) {
    // flush if we've reached capacity
    if self.vertices.len() + 4 >= self.vertices.capacity() {
      self.flush();
    }

    let texture_id = self.allocate_texture(&region.texture);

    let position = options.position;
    let size = vec2(
      (options.scale.x * region.size.x as f32) * 0.5,
      (options.scale.y * region.size.y as f32) * 0.5,
    );
    let uv = region.calculate_uv();

    self.vertices.push(SpriteVertex {
      position: position + vec2(-size.x, -size.y),
      color: options.color,
      uv: uv.top_left(),
      texture_id,
    });

    self.vertices.push(SpriteVertex {
      position: position + vec2(-size.x, size.y),
      color: options.color,
      uv: uv.bottom_left(),
      texture_id,
    });

    self.vertices.push(SpriteVertex {
      position: position + vec2(size.x, size.y),
      color: options.color,
      uv: uv.bottom_right(),
      texture_id,
    });

    self.vertices.push(SpriteVertex {
      position: position + vec2(size.x, -size.y),
      color: options.color,
      uv: uv.top_right(),
      texture_id,
    });
  }

  /// Flushes the batch to the GPU.
  #[profiling::function]
  pub fn flush(&mut self) {
    if self.vertices.is_empty() {
      return; // no vertices? no problem
    }

    // fetch the material out
    let Some(material) = &mut self.material else { return; };

    let vertex_count = self.vertices.len();
    let sprite_count = vertex_count / 4;
    let index_count = sprite_count * 6;

    self.textures.bind(material);

    self.mesh.with_buffers(|vertices, _| {
      vertices.write_data(&self.vertices);
    });

    let mesh = &self.mesh;

    mesh.draw_sub(material, PrimitiveTopology::Triangles, vertex_count, index_count);

    self.vertices.clear();
  }

  fn allocate_texture(&mut self, texture: &Texture) -> u8 {
    let slot = self.textures.allocate(&texture);

    // flush if the texture pool is full
    if slot.is_none() {
      self.flush();
      self.textures.allocate(&texture).unwrap()
    } else {
      slot.unwrap()
    }
  }
}

/// Fills a new buffer with standard quad indices.
fn build_quad_indices(sprite_count: usize) -> Vec<u32> {
  let mut indices = Vec::with_capacity(sprite_count * 6);
  let mut index = 0;

  for _ in 0..sprite_count {
    indices.push(index + 0);
    indices.push(index + 1);
    indices.push(index + 2);
    indices.push(index + 2);
    indices.push(index + 3);
    indices.push(index + 0);

    index += 4;
  }

  indices
}

/// Retains a pool of textures in unique slot indices to allow multiple textures per batch
#[derive(Default)]
struct TexturePool {
  slots: [Option<Texture>; 16],
}

impl TexturePool {
  /// Allocates a new texture from the pool, if possible.
  pub fn allocate(&mut self, texture: &Texture) -> Option<u8> {
    for (index, slot) in self.slots.iter_mut().enumerate() {
      match slot {
        Some(existing) if existing.handle() == texture.handle() => {
          return Some(index as u8);
        }
        None => {
          *slot = Some(texture.clone());
          return Some(index as u8);
        }
        _ => continue,
      }
    }

    None
  }

  /// Binds all active texture in the pool to the given material and then resets all slots.
  pub fn bind(&mut self, material: &mut Material) {
    for (index, texture) in self.slots.iter().enumerate() {
      if let Some(texture) = texture {
        material.set_uniform(&format!("u_texture[{}]", index), texture);
      }
    }

    self.clear();
  }

  /// Clears the pool of all textures.
  pub fn clear(&mut self) {
    self.slots.fill(None);
  }
}
