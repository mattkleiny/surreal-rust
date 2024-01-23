//! Sprite management and rendering.
//!
//! Sprites are very common in projects, so this is a dedicated batch to
//! support.

use common::{macros::Vertex, vec2, Angle, Mat2, Vec2};

use super::*;

/// The default number of sprites to allocate in a new batch.
const DEFAULT_SPRITE_COUNT: usize = 1024;

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

/// Similar to [`SpriteBatch`], but allows for multiple textures per batch.
///
/// This is useful for rendering sprites from multiple texture sources in a
/// single draw call, however it requires the underlying shader program to
/// support multiple textures.
pub struct MultiSpriteBatch {
  mesh: Mesh<MultiSpriteVertex>,
  material: Option<Material>,
  vertices: Vec<MultiSpriteVertex>,
  textures: TextureBindingSet,
}

/// A specialized vertex for use in our sprite batch.
///
/// Encodes the texture index in the vertex, for use in the shader.
/// This allows us to use a single vertex buffer for multiple textures.
#[repr(C)]
#[derive(Clone, Debug, Vertex)]
struct MultiSpriteVertex {
  #[vertex(2, F32)]
  pub position: Vec2,
  #[vertex(2, F32)]
  pub uv: Vec2,
  #[vertex(4, U8, normalize)]
  pub color: Color32,
  #[vertex(1, U8)]
  pub texture_id: u8,
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

impl SpriteBatch {
  /// Constructs a new [`SpriteBatch`] with a default capacity.
  pub fn new(graphics: &GraphicsEngine) -> common::Result<Self> {
    Self::with_capacity(graphics, DEFAULT_SPRITE_COUNT)
  }

  /// Creates a new [`SpriteBatch`] with the given expected capacity.
  ///
  /// This will pre-allocate buffers to minimize reallocation costs.
  pub fn with_capacity(graphics: &GraphicsEngine, sprite_count: usize) -> common::Result<Self> {
    // build standard quad indices ahead-of-time
    let vertices = Vec::with_capacity(sprite_count * 4);
    let indices = build_quad_indices(sprite_count);

    // create mesh, upload quad indices immediately
    let mut mesh = Mesh::new(graphics, BufferUsage::Dynamic)?;

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
  pub fn draw_sprite(&mut self, region: &TextureRegion, options: &SpriteOptions) {
    // flush if we've reached capacity
    if self.vertices.len() + 4 >= self.vertices.capacity() {
      self.flush();
    }

    // flush if the texture has changed
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

impl MultiSpriteBatch {
  /// Constructs a new [`MultiSpriteBatch`] with a default capacity.
  pub fn new(graphics: &GraphicsEngine) -> common::Result<Self> {
    Self::with_capacity(graphics, DEFAULT_SPRITE_COUNT)
  }

  /// Creates a new [`MultiSpriteBatch`] with the given expected capacity.
  ///
  /// This will pre-allocate buffers to minimize reallocation costs.
  pub fn with_capacity(graphics: &GraphicsEngine, sprite_count: usize) -> common::Result<Self> {
    // build standard quad indices ahead-of-time
    let vertices = Vec::with_capacity(sprite_count * 4);
    let indices = build_quad_indices(sprite_count);

    // create mesh, upload quad indices immediately
    let mut mesh = Mesh::new(graphics, BufferUsage::Dynamic)?;

    mesh.with_buffers(|_, buffer| {
      buffer.write_data(&indices);
    });

    Ok(Self {
      mesh,
      vertices,
      material: None,
      textures: TextureBindingSet::default(),
    })
  }

  /// Starts a new batch run with the given `Material`.
  pub fn begin(&mut self, material: &Material) {
    self.material = Some(material.clone());
    self.vertices.clear();
  }

  /// Draws a single sprite texture to the batch with the given options.
  pub fn draw_sprite(&mut self, region: &TextureRegion, options: &SpriteOptions) {
    // flush if we've reached capacity
    if self.vertices.len() + 4 >= self.vertices.capacity() {
      self.flush();
    }

    let mut texture_id = self.textures.allocate(&region.texture);
    if texture_id.is_none() {
      // we've run out of texture slots, flush and try again
      self.flush();
      texture_id = self.textures.allocate(&region.texture);
    }

    let texture_id = texture_id.expect("Failed to allocate texture slot");

    let scale = vec2(
      region.size.x as f32 * options.scale.x,
      region.size.y as f32 * options.scale.y,
    );

    let angle = options.rotation;
    let translation = options.position;
    let transform = Mat2::from_scale_angle(scale, angle.into());
    let uv = region.calculate_uv();

    // add vertices
    self.vertices.push(MultiSpriteVertex {
      position: translation + transform * vec2(-0.5, -0.5),
      color: options.color,
      uv: uv.top_left(),
      texture_id,
    });

    self.vertices.push(MultiSpriteVertex {
      position: translation + transform * vec2(-0.5, 0.5),
      color: options.color,
      uv: uv.bottom_left(),
      texture_id,
    });

    self.vertices.push(MultiSpriteVertex {
      position: translation + transform * vec2(0.5, 0.5),
      color: options.color,
      uv: uv.bottom_right(),
      texture_id,
    });

    self.vertices.push(MultiSpriteVertex {
      position: translation + transform * vec2(0.5, -0.5),
      color: options.color,
      uv: uv.top_right(),
      texture_id,
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

    for (_index, _texture_id) in self.textures.iter().enumerate() {
      // TODO: set all of the textures into an array?
    }

    // write vertices to mesh
    mesh.with_buffers(|vertices, _| {
      vertices.write_data(&self.vertices);
    });

    mesh.draw_sub(material, PrimitiveTopology::Triangles, vertex_count, index_count);

    self.vertices.clear();
    self.textures.clear();
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
