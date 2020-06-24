//! Sprite batching and rendering.

/// A utility that is capable of batching sprite representations and rendering
/// them concurrently.
///
/// This implementation abstracts over the notion of a sprite, so that it can
/// vary based on your implementation. We basically only store vertex positions,
/// UVs, and other mesh data in the batch.
pub struct SpriteBatch {
  vertices: Vec<f32>,
  indices: Vec<u16>,
  uvs: Vec<f32>,
}

impl SpriteBatch {
  pub fn new() -> Self {
    Self {
      vertices: Vec::new(),
      indices: Vec::new(),
      uvs: Vec::new(),
    }
  }

  /// Pushes sprite geometry into the batch.
  pub fn push(&mut self, sprite: impl SpriteGeometry) {
    let vertices = sprite.emit_vertices();

    self.vertices.push(vertices[0]);
    self.vertices.push(vertices[1]);
    self.vertices.push(vertices[2]);
    self.vertices.push(vertices[3]);

    let uvs = sprite.emit_uvs();

    self.uvs.push(uvs[0]);
    self.uvs.push(uvs[1]);
    self.uvs.push(uvs[2]);
    self.uvs.push(uvs[3]);
  }

  /// Flushes the sprite batch to the given batch target.
  pub fn flush(&mut self, sink: &mut impl SpriteSink) {
    sink.flush_vertices(&self.vertices);
    sink.flush_indices(&self.indices);
    sink.flush_uvs(&self.uvs);
  }
}

/// Something that can be rendered via a `SpriteBatch` using geometric primitives.
pub trait SpriteGeometry {
  fn emit_vertices(&self) -> [f32; 4];
  fn emit_uvs(&self) -> [f32; 4];
}

/// Something that can accept sprite geometry and render it to the screen.
pub trait SpriteSink {
  fn flush_vertices(&mut self, vertices: &Vec<f32>);
  fn flush_indices(&mut self, indices: &Vec<u16>);
  fn flush_uvs(&mut self, uvs: &Vec<f32>);
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Sprite {}

  impl SpriteGeometry for Sprite {
    fn emit_vertices(&self) -> [f32; 4] {
      unimplemented!()
    }

    fn emit_uvs(&self) -> [f32; 4] {
      unimplemented!()
    }
  }

  #[test]
  fn it_should_build_a_simple_batch_of_sprite_quads() {
    let mut batch = SpriteBatch::new();

    batch.push(Sprite {});
    batch.push(Sprite {});
    batch.push(Sprite {});
    batch.push(Sprite {});
  }
}