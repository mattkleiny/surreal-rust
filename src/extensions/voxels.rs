//! A simple toolkit for voxel-based tessellation and rendering.

use crate::graphics::Mesh;

/// Describes the behaviour of a single voxel in the engine.
///
/// A voxel is essentially translatable to/from some smaller ID representation for efficient packing.
pub trait Voxel {
  /// The underlying ID type for this voxel; usually some sort of int.
  type Id: Copy;

  fn from_id(id: Self::Id) -> Self;
  fn to_id(&self) -> Self::Id;
}

const CHUNK_WIDTH: usize = 16;
const CHUNK_HEIGHT: usize = 128;
const CHUNK_DEPTH: usize = 16;

/// A fixed-size chunk of voxels in the world.
///
/// Chunks are groups of voxels that can be efficiently manipulated in bulk.
pub struct Chunk<V> where V: Voxel {
  voxels: [V::Id; CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH],
}

impl<V> Chunk<V> where V: Voxel {
  /// Constructs a chunk using implemented default value for the voxel type.
  pub fn new() -> Self where V: Default {
    Self::with_default(V::default())
  }

  /// Constructs a chunk with the given default value for all voxels.
  pub fn with_default(default: V) -> Self {
    Self { voxels: [default.to_id(); CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH] }
  }

  pub fn width(&self) -> usize { CHUNK_WIDTH }
  pub fn height(&self) -> usize { CHUNK_HEIGHT }
  pub fn depth(&self) -> usize { CHUNK_DEPTH }

  /// Gets the voxel at the given (x, y, z) position.
  #[inline]
  pub fn get(&self, x: usize, y: usize, z: usize) -> V {
    V::from_id(self.get_raw(x, y, z))
  }

  /// Gets the voxel ID at the given (x, y, z) position.
  #[inline]
  pub fn get_raw(&self, x: usize, y: usize, z: usize) -> V::Id {
    self.voxels[Self::compute_index(x, y, z)]
  }

  /// Sets the voxel at the given (x, y, z) position.
  #[inline]
  pub fn set(&mut self, x: usize, y: usize, z: usize, voxel: V) {
    self.set_raw(x, y, z, voxel.to_id());
  }

  /// Sets the voxel ID at the given (x, y, z) position.
  #[inline]
  pub fn set_raw(&mut self, x: usize, y: usize, z: usize, id: V::Id) {
    self.voxels[Self::compute_index(x, y, z)] = id;
  }

  /// Accesses the voxel IDs as a slice.
  #[inline(always)]
  pub fn as_slice(&self) -> &[V::Id] {
    &self.voxels
  }

  /// Mutably accesses the voxel IDs as a slice.
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [V::Id] {
    &mut self.voxels
  }

  /// Blanket implementation for any chunk where the voxels are tessellatable.
  pub fn tessellate(&self, mesh: &mut impl Mesh<Vertex=V::Vertex, Index=V::Index>) where V: Tessellator {
    for z in 0..self.depth() {
      for y in 0..self.height() {
        for x in 0..self.width() {
          let position = (x, y, z);
          let voxel = self.get(x, y, z);

          voxel.tessellate(position, mesh);
        }
      }
    }
  }

  /// Computes the index into the voxel array for the given (x, y, z) coordinates.
  #[inline(always)]
  fn compute_index(x: usize, y: usize, z: usize) -> usize {
    x + y * CHUNK_WIDTH + z * CHUNK_WIDTH * CHUNK_HEIGHT
  }
}

/// Represents a type that can be tessellated into a `Mesh`.
pub trait Tessellator {
  type Vertex;
  type Index;

  /// Tessellates the shape into the given mesh builder.
  fn tessellate(
    &self,
    position: (usize, usize, usize),
    mesh: &mut impl Mesh<Vertex=Self::Vertex, Index=Self::Index>,
  );
}

#[cfg(test)]
mod tests {
  use crate::graphics::BufferedMesh;
  use crate::maths::{vec2, Vector2};

  use super::*;

  type Chunk = super::Chunk<Block>;

  #[repr(u8)]
  #[derive(Copy, Clone, Debug, Eq, PartialEq)]
  enum Block {
    Void = 0,
    Grass = 1,
    Water = 2,
  }

  impl Default for Block {
    fn default() -> Self {
      Self::Void
    }
  }

  impl Voxel for Block {
    type Id = u8;

    fn from_id(id: Self::Id) -> Self {
      unsafe { std::mem::transmute(id) }
    }

    fn to_id(&self) -> Self::Id {
      *self as Self::Id
    }
  }

  impl Tessellator for Block {
    type Vertex = Vector2<f32>;
    type Index = u16;

    fn tessellate(
      &self,
      position: (usize, usize, usize),
      mesh: &mut impl Mesh<Vertex=Self::Vertex, Index=Self::Index>,
    ) {
      mesh.add_quad(&[
        vec2(0., 0.),
        vec2(1., 0.),
        vec2(0., 1.),
        vec2(1., 1.),
      ]);
    }
  }

  #[test]
  fn chunk_should_read_and_write_voxels() {
    let mut chunk = Chunk::new();

    for z in 0..chunk.depth() {
      for y in 0..chunk.height() {
        for x in 0..chunk.width() {
          chunk.set(x, y, z, match x % 3 {
            1 => Block::Grass,
            2 => Block::Water,
            _ => Block::Void,
          });
        }
      }
    }
  }

  #[test]
  fn chunk_should_tessellate_to_a_mesh() {
    let chunk = Chunk::new();
    let mut mesh = BufferedMesh::new();

    chunk.tessellate(&mut mesh);
  }
}