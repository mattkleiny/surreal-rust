/// A voxel in a chunk.
pub type Voxel = u8;

/// A statically sized chunk of [`Voxel`]s.
pub struct StaticChunk<const X: usize, const Y: usize, const Z: usize> where [(); X * Y * Z]: Sized {
  voxels: [Voxel; X * Y * Z],
}

impl<const X: usize, const Y: usize, const Z: usize> Default for StaticChunk<X, Y, Z> where [(); X * Y * Z]: Sized {
  /// Creates a new empty chunk.
  fn default() -> Self {
    Self {
      voxels: [Voxel::default(); X * Y * Z],
    }
  }
}

impl<const X: usize, const Y: usize, const Z: usize> StaticChunk<X, Y, Z> where [(); X * Y * Z]: Sized {
  pub const WIDTH: usize = X;
  pub const HEIGHT: usize = Y;
  pub const DEPTH: usize = Z;
  pub const VOLUME: usize = X * Y * Z;

  /// Reads a value from the chunk.
  pub fn get(&self, x: usize, y: usize, z: usize) -> Option<Voxel> {
    if x < X && y < Y && z < Z {
      Some(self.voxels[x + y * X + z * X * Y])
    } else {
      None
    }
  }

  /// Sets a value in the chunk.
  pub fn set(&mut self, x: usize, y: usize, z: usize, value: Voxel) {
    if x >= X || y >= Y || z >= Z {
      return;
    }

    self.voxels[x + y * X + z * X * Y] = value
  }

  /// Fills the chunk with a value.
  pub fn fill(&mut self, value: Voxel) {
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

  /// The width of the chunk, in units.
  #[inline]
  pub fn width(&self) -> usize {
    self.width
  }

  /// The height of the chunk, in units.
  #[inline]
  pub fn height(&self) -> usize {
    self.height
  }

  /// The depth of the chunk, in units.
  #[inline]
  pub fn depth(&self) -> usize {
    self.depth
  }

  /// The total volume of the chunk, in units.
  #[inline]
  pub fn volume(&self) -> usize {
    self.width * self.height * self.depth
  }

  /// Reads a value from the chunk.
  pub fn get(&self, x: usize, y: usize, z: usize) -> Option<Voxel> {
    if x < self.width && y < self.height && z < self.depth {
      Some(self.voxels[x + y * self.width + z * self.width * self.height])
    } else {
      None
    }
  }

  /// Sets a value in the chunk.
  pub fn set(&mut self, x: usize, y: usize, z: usize, value: Voxel) {
    if x >= self.width || y >= self.height || z >= self.depth {
      return;
    }

    self.voxels[x + y * self.width + z * self.width * self.height] = value
  }

  /// Fills the chunk with a value.
  pub fn fill(&mut self, value: Voxel) {
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
    type Chunk = StaticChunk<16, 128, 16>;

    let mut chunk = Chunk::default();

    chunk.fill(32);

    assert_eq!(chunk.get(0, 0, 0), Some(32u8));
    assert_eq!(chunk.get(15, 127, 15), Some(32u8));
  }
}