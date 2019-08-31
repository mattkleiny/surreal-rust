//! Mesh management abstractions.

use super::*;

/// Represents a mesh of vertices and indices that can be rendered on the graphics device.
pub struct Mesh<D: GraphicsDevice, V> {
  vertex_array: D::VertexArray,
  vertex_buffer: D::Buffer,
  index_buffer: D::Buffer,
  vertices: Vec<V>,
  indices: Vec<u16>,
}

impl<D: GraphicsDevice, V> Mesh<D, V> {
  pub fn new(device: &D) -> Self {
    unsafe {
      let vertex_array = device.create_vertex_array();
      let vertex_buffer = device.create_buffer();
      let index_buffer = device.create_buffer();

      device.upload_to_buffer::<()>(&vertex_buffer, BufferData::Uninitialized(0), BufferTarget::Vertex, BufferUploadMode::Dynamic);
      device.upload_to_buffer::<()>(&index_buffer, BufferData::Uninitialized(0), BufferTarget::Index, BufferUploadMode::Dynamic);

      Self {
        vertex_array,
        vertex_buffer,
        index_buffer,
        vertices: Vec::new(),
        indices: Vec::new(),
      }
    }
  }

  /// Renders the mesh on the graphics device.
  pub fn draw(&self, device: &D, shader_program: &D::Program, primitive_type: PrimitiveType) {
    unsafe {
      device.bind_buffer(&self.vertex_array, &self.vertex_buffer, BufferTarget::Vertex);
      device.bind_buffer(&self.vertex_array, &self.index_buffer, BufferTarget::Index);
    }
  }
}