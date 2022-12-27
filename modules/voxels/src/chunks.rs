/// A voxel in a chunk.
pub type Voxel = u8;

/// Represents a chunk of [`Voxel`]s.
pub trait Chunk {
  /// The width of the chunk, in units.
  fn width(&self) -> usize;

  /// The height of the chunk, in units.
  fn height(&self) -> usize;

  /// The depth of the chunk, in units.
  fn depth(&self) -> usize;

  /// The total volume of the chunk, in units.
  fn volume(&self) -> usize {
    self.width() * self.height() * self.depth()
  }

  /// Reads a value from the chunk.
  fn get(&self, x: usize, y: usize, z: usize) -> Option<Voxel>;

  /// Sets a value in the chunk.
  fn set(&mut self, x: usize, y: usize, z: usize, value: Voxel);

  /// Fills the chunk with a value.
  fn fill(&mut self, value: Voxel);
}

/// A statically sized chunk of [`Voxel`]s.
pub struct StaticChunk<const X: usize, const Y: usize, const Z: usize>
where
  [(); X * Y * Z]: Sized,
{
  voxels: [Voxel; X * Y * Z],
}

impl<const X: usize, const Y: usize, const Z: usize> Default for StaticChunk<X, Y, Z>
where
  [(); X * Y * Z]: Sized,
{
  /// Creates a new empty [`StaticChunk`].
  fn default() -> Self {
    Self {
      voxels: [Voxel::default(); X * Y * Z],
    }
  }
}

impl<const X: usize, const Y: usize, const Z: usize> Chunk for StaticChunk<X, Y, Z>
where
  [(); X * Y * Z]: Sized,
{
  #[inline(always)]
  fn width(&self) -> usize {
    X
  }

  #[inline(always)]
  fn height(&self) -> usize {
    Y
  }

  #[inline(always)]
  fn depth(&self) -> usize {
    Z
  }

  /// Reads a value from the chunk.
  fn get(&self, x: usize, y: usize, z: usize) -> Option<Voxel> {
    if x < X && y < Y && z < Z {
      Some(self.voxels[x + y * X + z * X * Y])
    } else {
      None
    }
  }
  /// Sets a value in the chunk.
  fn set(&mut self, x: usize, y: usize, z: usize, value: Voxel) {
    if x >= X || y >= Y || z >= Z {
      return;
    }

    self.voxels[x + y * X + z * X * Y] = value
  }
  /// Fills the chunk with a value.
  fn fill(&mut self, value: Voxel) {
    self.voxels.fill(value);
  }
}

/// A dynamically sized chunk of [`Voxel`]s.
pub struct DynamicChunk {
  width: usize,
  height: usize,
  depth: usize,
  voxels: Vec<Voxel>,
}

impl DynamicChunk {
  /// Creates a new empty chunk.
  pub fn new(width: usize, height: usize, depth: usize) -> Self {
    Self {
      width,
      height,
      depth,
      voxels: vec![Voxel::default(); width * height * depth],
    }
  }
}

impl Chunk for DynamicChunk {
  /// The width of the chunk, in units.
  #[inline]
  fn width(&self) -> usize {
    self.width
  }

  /// The height of the chunk, in units.
  #[inline]
  fn height(&self) -> usize {
    self.height
  }

  /// The depth of the chunk, in units.
  #[inline]
  fn depth(&self) -> usize {
    self.depth
  }

  fn get(&self, x: usize, y: usize, z: usize) -> Option<Voxel> {
    if x < self.width && y < self.height && z < self.depth {
      Some(self.voxels[x + y * self.width + z * self.width * self.height])
    } else {
      None
    }
  }

  fn set(&mut self, x: usize, y: usize, z: usize, value: Voxel) {
    if x >= self.width || y >= self.height || z >= self.depth {
      return;
    }

    self.voxels[x + y * self.width + z * self.width * self.height] = value
  }

  fn fill(&mut self, value: Voxel) {
    self.voxels.fill(value);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dynamic_chunk_should_read_and_write_standard_data() {
    let mut chunk = DynamicChunk::new(16, 128, 16);

    chunk.fill(32);

    assert_eq!(chunk.get(0, 0, 0), Some(32u8));
    assert_eq!(chunk.get(15, 127, 15), Some(32u8));
  }

  #[test]
  fn static_chunk_should_read_and_write_standard_data() {
    let mut chunk = StaticChunk::<16, 128, 16>::default();

    chunk.fill(32);

    assert_eq!(chunk.get(0, 0, 0), Some(32u8));
    assert_eq!(chunk.get(15, 127, 15), Some(32u8));
  }
}
