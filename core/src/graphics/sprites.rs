//! Sprite management and rendering.
//!
//! Sprites are very common in projects, so this is a dedicated batch to support.

use super::*;
use crate as surreal;
use crate::{
  diagnostics,
  maths::{vec2, Mat2, Vec2},
};

/// The default number of sprites to allocate in a new batch.
const DEFAULT_SPRITE_COUNT: usize = 1024;

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
  vertices: Vec<SpriteVertex>,
  last_texture: Option<Texture>,
}

/// Options for drawing a sprite.
pub struct SpriteOptions {
  pub position: Vec2,
  pub rotation: f32,
  pub scale: Vec2,
  pub color: Color32,
}

impl Default for SpriteOptions {
  fn default() -> Self {
    Self {
      position: Vec2::ZERO,
      rotation: 0.,
      scale: Vec2::ONE,
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
  pub position: Vec2,
  pub uv: Vec2,
  pub color: Color32,
}

impl Vertex for SpriteVertex {
  #[rustfmt::skip]
  const DESCRIPTORS: &'static [VertexDescriptor] = &[
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 4, kind: VertexKind::U8, should_normalize: true },
  ];
}

impl SpriteBatch {
  /// Constructs a new [`SpriteBatch`] with a default capacity.
  pub fn new(graphics: &GraphicsServer) -> Self {
    Self::with_capacity(graphics, DEFAULT_SPRITE_COUNT)
  }

  /// Creates a new [`SpriteBatch`] with the given expected sprite capacity.
  ///
  /// This will pre-allocate buffers to minimize reallocation costs.
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
      vertices,
      material: None,
      last_texture: None,
    }
  }

  /// Starts a new batch run with the given `Material`.
  #[diagnostics::profiling]
  pub fn begin(&mut self, material: &Material) {
    self.material = Some(material.clone());
    self.vertices.clear();
  }

  /// Draws a line of text to the batch with the given options
  #[diagnostics::profiling]
  pub fn draw_text(&mut self, font: &dyn Font, text: &str, options: &SpriteOptions) {
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
            rotation: 0.,
            scale: options.scale,
            color: options.color,
          },
        );

        position.x += glyph.size.x as f32 * options.scale.x;
      }
    }
  }

  /// Draws a single sprite texture to the batch with the given options.
  #[diagnostics::profiling]
  pub fn draw_sprite<'a>(&mut self, region: &'a TextureRegion, options: &SpriteOptions) {
    // flush if we've reached capacity
    if self.vertices.len() + 4 >= self.vertices.capacity() {
      self.flush();
    }

    if let Some(texture) = &self.last_texture {
      if texture.handle() != region.texture.handle() {
        self.flush();
        self.last_texture = Some(region.texture.clone());
      }
    } else if self.last_texture.is_none() {
      self.last_texture = Some(region.texture.clone());
    }

    let scale = vec2(region.size.x as f32 * options.scale.x, region.size.y as f32 * options.scale.y);
    let angle = options.rotation;
    let translation = options.position;

    // prepare vertex transform
    let transform = Mat2::from_scale_angle(scale, angle);

    let uv = region.calculate_uv();

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
  #[diagnostics::profiling]
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

    if let Some(texture) = &self.last_texture {
      material.set_texture("u_texture", texture, None);
    }

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
