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

/// A voxel in a chunk.
pub type Voxel = u8;

/// A type that can be expanded to fill an existing [`VoxelChunk`] with data.
pub trait VoxelBrush {
  /// The size of the volume of the brush, in (width, height, depth) units.
  fn size(&self) -> (usize, usize, usize);

  /// The width of the brush, in units.
  fn width(&self) -> usize {
    self.size().0
  }

  /// The height of the brush, in units.
  fn height(&self) -> usize {
    self.size().1
  }

  /// The depth of the brush, in units.
  fn depth(&self) -> usize {
    self.size().2
  }

  /// The total volume of the brush, in units.
  fn volume(&self) -> usize {
    let (width, height, depth) = self.size();

    width * height * depth
  }
}

/// An operation that permits merging [`VoxelBrush`]es to produce new
/// [`VoxelBrush`]es.
pub trait VoxelMerge {
  /// Merge the given two [`VoxelBrush`]es to produce a new [`VoxelBrush`].
  fn merge(&self, a: &dyn VoxelBrush, b: &dyn VoxelBrush) -> Box<dyn VoxelBrush>;

  /// Merges the given other [`VoxelBrush`] into the given [`VoxelChunk`].
  fn merge_in_place(&self, into: &mut dyn VoxelChunk, other: &dyn VoxelBrush);
}

/// Default, commonly-used [`VoxelMerge`] operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoxelOperation {
  Union,
  Intersection,
  Subtraction,
}

impl VoxelMerge for VoxelOperation {
  fn merge(&self, _a: &dyn VoxelBrush, _b: &dyn VoxelBrush) -> Box<dyn VoxelBrush> {
    match self {
      VoxelOperation::Union => todo!(),
      VoxelOperation::Intersection => todo!(),
      VoxelOperation::Subtraction => todo!(),
    }
  }

  fn merge_in_place(&self, _chunk: &mut dyn VoxelChunk, _brush: &dyn VoxelBrush) {
    match self {
      VoxelOperation::Union => todo!(),
      VoxelOperation::Intersection => todo!(),
      VoxelOperation::Subtraction => todo!(),
    }
  }
}

/// Represents a chunk of [`Voxel`]s.
///
/// A chunk is a 3D grid of [`Voxel`]s, which can be used to represent a
/// discrete volume of space.
pub trait VoxelChunk: VoxelBrush {}

/// A default [`VoxelChunk`] implementation backed by a [`Vec`].
///
/// [`Voxel`]s are stored in a flat array, with the first dimension being the
/// width, the second being the height, and the third being the depth.
///
/// The origin of the chunk is at the bottom left corner, with the positive X
/// axis pointing right, the positive Y axis pointing up, and the positive Z
/// axis pointing out of the screen.
#[derive(Clone)]
pub struct Chunk {
  size: (u16, u16, u16),
  _voxels: Vec<Voxel>,
}

impl Chunk {
  pub const DEFAULT_WIDTH: usize = 16;
  pub const DEFAULT_HEIGHT: usize = 128;
  pub const DEFAULT_DEPTH: usize = 16;

  /// The default size of a chunk, in (width, height, depth) units.
  pub const DEFAULT_SIZE: (usize, usize, usize) = (Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT, Self::DEFAULT_DEPTH);
}

impl Default for Chunk {
  fn default() -> Self {
    Self {
      size: (Self::DEFAULT_WIDTH as u16, Self::DEFAULT_HEIGHT as u16, Self::DEFAULT_DEPTH as u16),
      _voxels: Vec::new(),
    }
  }
}

impl VoxelChunk for Chunk {}

impl VoxelBrush for Chunk {
  fn size(&self) -> (usize, usize, usize) {
    let (width, height, depth) = self.size;

    (width as usize, height as usize, depth as usize)
  }
}

impl VoxelBrush for surreal::maths::Plane {
  fn size(&self) -> (usize, usize, usize) {
    todo!()
  }
}

impl VoxelBrush for surreal::maths::Sphere {
  fn size(&self) -> (usize, usize, usize) {
    todo!()
  }
}

impl VoxelBrush for surreal::maths::Cube {
  fn size(&self) -> (usize, usize, usize) {
    todo!()
  }
}

impl VoxelBrush for surreal::maths::Cylinder {
  fn size(&self) -> (usize, usize, usize) {
    todo!()
  }
}

impl VoxelBrush for surreal::maths::Trapezoid {
  fn size(&self) -> (usize, usize, usize) {
    todo!()
  }
}
