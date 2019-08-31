//! Mesh management abstractions.

use super::*;

/// Represents a mesh of vertices and indices that can be rendered on the graphics device.
pub struct Mesh<D: GraphicsDevice> {
  vertex_array: D::VertexArray,
  vertex_buffer: D::Buffer,
  index_buffer: D::Buffer,
}

impl<D: GraphicsDevice> Mesh<D> {
  pub fn new(device: &D) -> Self {
    unsafe {
      Self {
        vertex_array: device.create_vertex_array(),
        vertex_buffer: device.create_buffer(),
        index_buffer: device.create_buffer(),
      }
    }
  }

  /// Uploads the mesh data to the graphics device.
  pub fn upload_to_gpu<V>(&mut self, device: &D, vertices: &[V], indices: &[u16]) {
    unsafe {
      device.upload_to_buffer(
        &self.vertex_buffer,
        BufferData::Memory(vertices),
        BufferTarget::Vertex,
        BufferUploadMode::Dynamic,
      );

      device.upload_to_buffer(
        &self.index_buffer,
        BufferData::Memory(indices),
        BufferTarget::Index,
        BufferUploadMode::Dynamic,
      );
    }
  }

  /// Renders the mesh on the graphics device.
  pub fn render(&self, device: &D, index_count: usize, shader_program: &D::Program, textures: &[&D::Texture], primitive_type: PrimitiveType, projection_view: Mat4) {
    unsafe {
      device.bind_buffer(&self.vertex_array, &self.vertex_buffer, BufferTarget::Vertex);
      device.bind_buffer(&self.vertex_array, &self.index_buffer, BufferTarget::Index);

      let u_projection_view = device.get_uniform(shader_program, "u_projView");

      let render_state = RenderState {
        render_target: &RenderTarget::Default,
        vertex_array: &self.vertex_array,
        shader_program,
        primitive_type,
        uniforms: &[
          (&u_projection_view, UniformData::Mat4(projection_view))
        ],
        textures,
        viewport: RectI::new(0, 0, 1920, 1080),
        rasterizer: RasterizerState::default(),
      };

      device.draw_arrays(index_count as u32, &render_state);
    }
  }
}