//! Voxel management and mesh conversion for Surreal.
//!
//! This module provides a series of operations for working with voxel-based
//! data structures, converting voxel representations, chunking and meshing and
//! palette indexing.
//!
//! The core of the module is the [`Voxel`] type, which represents a single
//! voxel in a larger [`VoxelChunk`]. A voxel indicates the kind of type or
//! material present at any 3d point in a discretized space.
//!
//! [`VoxelChunk`]s can be combined with [`VoxelMerge`] operations, and
//! templated via [`VoxelBrush`] types.

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 128;
pub const CHUNK_DEPTH: usize = 16;

/// A voxel in a chunk.
pub type Voxel = u8;

/// An array of [`Voxel`]s at chunk capacity.
pub type ChunkArray = [Voxel; CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH];

/// A chunk of voxels backed by a [`Vec`].
///
/// [`Voxel`]s are stored in a flat array, with the first dimension being the
/// width, the second being the height, and the third being the depth.
///
/// The origin of the chunk is at the bottom left corner, with the positive X
/// axis pointing right, the positive Y axis pointing up, and the positive Z
/// axis pointing out of the screen.
#[derive(Clone)]
pub struct VoxelChunk {
  voxels: ChunkArray,
}

impl Default for VoxelChunk {
  fn default() -> Self {
    Self {
      voxels: [0; CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH],
    }
  }
}

impl VoxelChunk {
  /// The size of the chunk, in (width, height, depth) units.
  pub fn size(&self) -> (u16, u16, u16) {
    (CHUNK_WIDTH as u16, CHUNK_HEIGHT as u16, CHUNK_DEPTH as u16)
  }

  /// Returns a slice of all the [`Voxel`]s in the chunk.
  pub fn voxels(&self) -> &ChunkArray {
    &self.voxels
  }

  /// Returns a slice of all the [`Voxel`]s in the chunk.
  pub fn voxels_mut(&mut self) -> &mut ChunkArray {
    &mut self.voxels
  }
}

/// A brush that can be painted into a [`VoxelChunk`].
pub trait VoxelBrush {
  /// Paints the brush into the given [`VoxelChunk`].
  fn paint(&self, chunk: &mut VoxelChunk, options: &BrushOptions);
}

/// Options for painting a [`VoxelChunk`] with a [`VoxelBrush`].
#[derive(Default, Clone, Debug)]
pub struct BrushOptions {
  /// The voxel to fill the brush with.
  pub fill_voxel: Voxel,
}

impl VoxelBrush for VoxelChunk {
  fn paint(&self, chunk: &mut VoxelChunk, _options: &BrushOptions) {
    chunk.voxels_mut().copy_from_slice(&self.voxels);
  }
}

impl VoxelBrush for surreal::maths::Plane {
  fn paint(&self, _chunk: &mut VoxelChunk, _options: &BrushOptions) {
    todo!()
  }
}

impl VoxelBrush for surreal::maths::Sphere {
  fn paint(&self, _chunk: &mut VoxelChunk, _options: &BrushOptions) {
    todo!()
  }
}

impl VoxelBrush for surreal::maths::Cube {
  fn paint(&self, _chunk: &mut VoxelChunk, _options: &BrushOptions) {
    todo!()
  }
}

impl VoxelBrush for surreal::maths::Cylinder {
  fn paint(&self, _chunk: &mut VoxelChunk, _options: &BrushOptions) {
    todo!()
  }
}

impl VoxelBrush for surreal::maths::Trapezoid {
  fn paint(&self, _chunk: &mut VoxelChunk, _options: &BrushOptions) {
    todo!()
  }
}
