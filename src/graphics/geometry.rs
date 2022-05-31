use crate::maths::{Vector2, vec2, Rectangle, Circle};

use super::*;

// Represents a point in the batch.
type Point = Vector2<f32>;
/// A fast and lightweight geometry batch renderer.
///
/// This batch pre-allocates an array of vertices and re-uses it to tessellate shapes and polygons.
pub struct GeometryBatch {
  mesh: Mesh<Vertex2>,
  vertices: Vec<Vertex2>,
  indices: Vec<Index>,
  material: Option<Material>,
}

impl GeometryBatch {
  /// Creates a new geometry batch.
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      mesh: Mesh::new(server, BufferUsage::Dynamic),
      vertices: Vec::new(),
      indices: Vec::new(),
      material: None,
    }
  }

  /// Restarts the batch with the given material.
  pub fn begin(&mut self, material: &Material) {
    self.material = Some(material.clone());
    self.vertices.clear();
  }

  /// Draws a line in the batch.
  pub fn draw_line(&mut self, a: Point, b: Point, color: Color32, _thickness: f32) {
    let base_offset = self.vertices.len() as Index;

    // TODO: get the winding order correct?
    self.vertices.push(Vertex2 { position: a, uv: vec2(0., 0.), color });
    self.vertices.push(Vertex2 { position: b, uv: vec2(1., 1.), color });
    
    self.indices.push(base_offset + 0);
    self.indices.push(base_offset + 1);
  }
  
  /// Draws a triangle in the batch.
  pub fn draw_triangle(&mut self, a: Point, b: Point, c: Point, color: Color32) {
    let base_offset = self.vertices.len() as Index;

    self.vertices.push(Vertex2 { position: a, uv: vec2(0., 0.), color });
    self.vertices.push(Vertex2 { position: b, uv: vec2(0.5, 1.), color });
    self.vertices.push(Vertex2 { position: c, uv: vec2(1., 0.), color });

    self.indices.push(base_offset + 0);
    self.indices.push(base_offset + 1);
    self.indices.push(base_offset + 2);
  }
  
  /// Draws a strip of triangles in the batch.
  pub fn draw_triangle_strip(&mut self, points: &[Point], color: Color32) {
    if points.len() < 3 { return; }

    let base_offset = self.vertices.len() as Index;
    
    self.vertices.push(Vertex2 { position: points[0], uv: vec2(0., 0.), color });

    for i in 1..points.len() - 1 {
      let offset = self.vertices.len() as Index;

      self.vertices.push(Vertex2 { position: points[i + 0], uv: vec2(0., 0.), color });
      self.vertices.push(Vertex2 { position: points[i + 1], uv: vec2(0., 0.), color });

      self.indices.push(base_offset);
      self.indices.push(offset + 0);
      self.indices.push(offset + 1);
    }
  }

  /// Draws a rectangle in the batch.
  pub fn draw_rectangle(&mut self, rectangle: Rectangle<f32>, color: Color32) {
    let base_offset = self.vertices.len() as Index;
    
    self.vertices.push(Vertex2 { position: rectangle.bottom_left(), uv: vec2(0., 0.), color });
    self.vertices.push(Vertex2 { position: rectangle.top_left(), uv: vec2(0., 1.), color });
    self.vertices.push(Vertex2 { position: rectangle.top_right(), uv: vec2(1., 1.), color });
    self.vertices.push(Vertex2 { position: rectangle.bottom_right(), uv: vec2(1., 0.), color });

    self.indices.push(base_offset + 0);
    self.indices.push(base_offset + 1);
    self.indices.push(base_offset + 2);

    self.indices.push(base_offset + 0);
    self.indices.push(base_offset + 2);
    self.indices.push(base_offset + 3);
  }
  
  /// Draws a circle in the batch.
  pub fn draw_circle(&mut self, center: Point, radius: f32, segments: u16, color: Color32) {
    let mut points = Vec::with_capacity(segments as usize);

    for i in 0..segments {
      let angle = i as f32 / segments as f32 * std::f32::consts::PI * 2.0;

      let x = center.x + radius * angle.cos();
      let y = center.y + radius * angle.sin();

      points.push(vec2(x, y));
    }

    self.draw_triangle_strip(&points, color);
  }
  
  /// Draws a sprite in the batch.
  pub fn draw_sprite(&mut self, _texture: &TextureRegion, _position: Point, _rotation: f32, _tint: Color32) {
    todo!();
  }

  /// Flushes the batch content to the GPU.
  pub fn flush(&mut self) {
    // ensure we're in a valid state to render something
    if self.vertices.len() == 0 { return; };
    if self.indices.len() == 0 { return; };
    let Some(material) = &self.material else { return; };

    // upload and draw the mesh
    self.mesh.with_buffers(|vertices, indices| {
      vertices.write_data(&self.vertices);
      indices.write_data(&self.indices);
    });

    self.mesh.draw_sub_mesh(
      &material, 
      PrimitiveTopology::Triangles, 
      self.vertices.len(), 
      self.indices.len(),
    );

    self.vertices.clear();
  }
}