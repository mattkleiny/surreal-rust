//! A simple toolkit for voxel-based tessellation and rendering.

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
/// Chunks are groups of voxels that can be manipulated in bulk.
pub struct Chunk<V> where V: Voxel {
  voxels: [V::Id; CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH]
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
  pub fn get(&self, x: usize, y: usize, z: usize) -> V {
    V::from_id(self.get_raw(x, y, z))
  }

  /// Gets the voxel ID at the given (x, y, z) position.
  pub fn get_raw(&self, x: usize, y: usize, z: usize) -> V::Id {
    self.voxels[Self::compute_index(x, y, z)]
  }

  /// Sets the voxel at the given (x, y, z) position.
  pub fn set(&mut self, x: usize, y: usize, z: usize, voxel: V) {
    self.set_raw(x, y, z, voxel.to_id());
  }

  /// Sets the voxel ID at the given (x, y, z) position.
  pub fn set_raw(&mut self, x: usize, y: usize, z: usize, id: V::Id) {
    self.voxels[Self::compute_index(x, y, z)] = id;
  }

  /// Accesses the voxel IDs as a slice.
  pub fn as_slice(&self) -> &[V::Id] {
    &self.voxels
  }

  /// Mutably accesses the voxel IDs as a slice.
  pub fn as_mut_slice(&mut self) -> &mut [V::Id] {
    &mut self.voxels
  }

  /// Tessellates this chunk directly into a mesh.
  pub fn to_mesh(&self) -> ChunkMesh where V: Tessellator {
    ChunkMesh::tessellate(self)
  }

  /// Computes the index into the voxel array for the given (x, y, z) coordinates.
  #[inline(always)]
  fn compute_index(x: usize, y: usize, z: usize) -> usize {
    x + y * CHUNK_WIDTH + z * CHUNK_WIDTH * CHUNK_HEIGHT
  }
}

pub type Position = (f32, f32, f32);
pub type UV = (f32, f32);
pub type Index = u16;

/// A vertex in a constructed `Mesh`.
#[repr(C)]
#[derive(Default, Clone, Debug)]
pub struct Vertex {
  pub position: Position,
  pub uv: UV,
}

/// Represents a type that can be tessellated into a `Mesh`.
pub trait Tessellator {
  /// Tessellates the shape into the mesh builder.
  fn tessellate(&self, position: (usize, usize, usize), mesh: &mut impl Mesh);
}

/// A type that supports procedural construction of `Mesh`es.
pub trait Mesh {
  /// Accesses the vertices of the mesh.
  fn vertices(&self) -> &[Vertex];

  /// Mutably accesses the vertices of the mesh.
  fn vertices_mut(&mut self) -> &mut [Vertex];

  /// Accesses the indices of the mesh.
  fn indices(&self) -> &[Index];

  /// Mutably accesses the indices of the mesh.
  fn indices_mut(&mut self) -> &mut [Index];

  /// Adds a single vertex to the mesh.
  fn add_vertex(&mut self, vertex: &Vertex);

  /// Adds a single index to the mesh.
  fn add_index(&mut self, index: Index);

  /// Adds a triangle of vertices to the mesh.
  fn add_triangle(&mut self, vertices: &[Vertex; 3]) {
    let index = self.vertices().len() as u16;

    self.add_vertex(&vertices[0]);
    self.add_vertex(&vertices[1]);
    self.add_vertex(&vertices[2]);

    self.add_index(index + 0);
    self.add_index(index + 1);
    self.add_index(index + 2);
  }

  /// Adds a quad of vertices to the mesh.
  fn add_quad(&mut self, vertices: &[Vertex; 4]) {
    let index = self.vertices().len() as u16;

    self.add_vertex(&vertices[0]);
    self.add_vertex(&vertices[1]);
    self.add_vertex(&vertices[2]);
    self.add_vertex(&vertices[3]);

    self.add_index(index + 0);
    self.add_index(index + 1);
    self.add_index(index + 2);
    self.add_index(index + 0);
  }

  /// Adds a triangle fan of vertices to the mesh.
  fn add_triangle_fan(&mut self, vertices: &[Vertex]) {
    unimplemented!()
  }
}

/// A mesh of vertices/triangles that can tessellated directly from a `Chunk` representation.
pub struct ChunkMesh {
  vertices: Vec<Vertex>,
  indices: Vec<Index>,
}

impl ChunkMesh {
  /// Constructs a new empty mesh.
  pub fn new() -> Self {
    Self {
      vertices: Vec::new(),
      indices: Vec::new(),
    }
  }

  /// Tessellates the given chunk into a mesh.
  pub fn tessellate<V>(chunk: &Chunk<V>) -> Self where V: Voxel + Tessellator {
    let mut mesh = Self::new();

    for z in 0..chunk.depth() {
      for y in 0..chunk.height() {
        for x in 0..chunk.width() {
          let voxel = chunk.get(x, y, z);
          let position = (x, y, z);

          voxel.tessellate(position, &mut mesh);
        }
      }
    }

    mesh // baked and good to go
  }
}

impl Mesh for ChunkMesh {
  #[inline(always)]
  fn vertices(&self) -> &[Vertex] {
    &self.vertices
  }

  #[inline(always)]
  fn vertices_mut(&mut self) -> &mut [Vertex] {
    &mut self.vertices
  }

  #[inline(always)]
  fn indices(&self) -> &[u16] {
    &self.indices
  }

  #[inline(always)]
  fn indices_mut(&mut self) -> &mut [u16] {
    &mut self.indices
  }

  #[inline(always)]
  fn add_vertex(&mut self, vertex: &Vertex) {
    self.vertices.push(vertex.clone());
  }

  #[inline(always)]
  fn add_index(&mut self, index: u16) {
    self.indices.push(index);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[repr(C)]
  #[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
  struct Block(u8);

  impl Block {
    const VOID: Block = Self(0);
    const GRASS: Block = Self(1);
    const WATER: Block = Self(2);
  }

  impl Voxel for Block {
    type Id = u8;

    #[inline(always)]
    fn from_id(id: Self::Id) -> Self {
      Self(id)
    }

    #[inline(always)]
    fn to_id(&self) -> Self::Id {
      self.0
    }
  }

  impl Tessellator for Block {
    fn tessellate(&self, (x, y, z): (usize, usize, usize), mesh: &mut impl Mesh) {
      unimplemented!()
    }
  }

  type Chunk = super::Chunk<Block>;

  #[test]
  fn chunk_should_read_and_write_voxels() {
    let mut chunk = Chunk::new();

    for group in chunk.as_mut_slice().chunks_exact_mut(3) {
      group[0] = Block::VOID.0;
      group[1] = Block::GRASS.0;
      group[2] = Block::WATER.0;
    }
  }

  #[test]
  fn mesh_should_tessellate_from_a_simple_chunk() {
    let chunk = Chunk::new();
    let mesh = chunk.to_mesh();

    assert!(mesh.vertices.len() > 0);
    assert!(mesh.indices.len() > 0);
  }
}