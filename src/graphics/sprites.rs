//! Sprite management and rendering.
//!
//! Sprites are very common in projects, so we've a simple dedicated batch to support them.

use crate::maths::{vec2, Vector2};

use super::*;

/// The default number of sprites to allocate in a new batch.
const DEFAULT_SPRITE_COUNT: usize = 1024;

/// The maximum number of textures that can be bound in a single batch operation.
const TEXTURE_POOL_SIZE: usize = 32;

/// A fast and lightweight sprite batch renderer.
///
/// This batch pre-allocates an array of vertices and indices and re-uses them for as many
/// sprites as possible.
///
/// Batching is possible over 1 material; however up to 32 unique texture sources can be used
/// for that single batch operation. It's expected that the associated shader program is capable
/// of supporting multiple textures per operation.
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
/// Encodes a unique `texture_id` representing which of the bound texture units is
/// relevant for this sprite. This is used to avoid unnecessary flushes in the batch
/// when rendering sprites from multiple texture sources simultaneously.
#[repr(C)]
#[derive(Clone, Debug)]
struct SpriteVertex {
  pub position: Vector2<f32>,
  pub uv: Vector2<f32>,
  pub color: Color32,
  pub texture_id: u32,
}

impl Vertex for SpriteVertex {
  #[rustfmt::skip]
  const DESCRIPTORS: &'static [VertexDescriptor] = &[
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 4, kind: VertexKind::U8, should_normalize: true },
    VertexDescriptor { count: 1, kind: VertexKind::U32, should_normalize: false },
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
    let mut mesh = Mesh::new(graphics, BufferUsage::Dynamic);

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

    let position = options.position;
    let size = vec2(
      (options.scale.x * region.size.x as f32) * 0.5,
      (options.scale.y * region.size.y as f32) * 0.5,
    );
    let uv = region.calculate_uv();
    let texture_id = self.allocate_texture(&region.texture) as u32;

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
    let material = &mut self.material;
    if material.is_none() {
      return;
    }
    let material = material.as_mut().unwrap();

    let vertex_count = self.vertices.len();
    let sprite_count = vertex_count / 4;
    let index_count = sprite_count * 6;
    let mesh = &mut self.mesh;

    self.textures.bind(material);

    mesh.with_buffers(|vertices, _| {
      vertices.write_data(&self.vertices);
    });

    mesh.draw_sub(material, PrimitiveTopology::Triangles, vertex_count, index_count);

    self.vertices.clear();
    self.textures.clear();
  }

  /// Allocates a texture slot id for the given texture in the batch.
  fn allocate_texture(&mut self, texture: &Texture) -> u8 {
    match self.textures.allocate(&texture) {
      Some(slot) => slot,
      None => {
        // flush if we've reached texture capacity
        self.flush();
        self.textures.allocate(&texture).unwrap()
      }
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

/// Retains a pool of textures in unique texture slot indices to
/// allow multiple textures per batch.
///
/// Internally the pool will take care of allocating the slots as used.
/// If the pool is full, the batch will be flushed and the pool will be reset.
#[derive(Default)]
struct TexturePool {
  slots: [Option<Texture>; TEXTURE_POOL_SIZE],
}

impl TexturePool {
  /// Allocates a new texture from the pool, if possible.
  ///
  /// If the texture has already been allocated, returns it's slot index.
  /// Otherwise, the texture will be emplaced for use this frame.
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

  /// Binds all active texture in the pool to the given material.
  pub fn bind(&mut self, material: &mut Material) {
    let mut textures = smallvec::SmallVec::<[&Texture; TEXTURE_POOL_SIZE]>::new();

    for texture in self.slots.iter() {
      if let Some(texture) = texture {
        textures.push(texture);
      }
    }

    material.set_texture_array("u_textures", &textures, None);
  }

  /// Clears the pool of all textures.
  pub fn clear(&mut self) {
    self.slots.fill(None);
  }
}
