//! Geometry batching for common shapes and polygon rendering.

use common::{vec2, Circle, Color32, Line, Rectangle, Vec2};

use super::*;

/// A fast and lightweight geometry batch renderer.
///
/// This batch pre-allocates an array of vertices and re-uses it to tessellate
/// shapes and polygons.
pub struct GeometryBatch {
  mesh: Mesh<GeometryVertex>,
  vertices: Vec<GeometryVertex>,
  indices: Vec<MeshIndex>,
  material: Option<Material>,
}

/// A specialized vertex for use in our geometry batch.
#[repr(C)]
#[derive(Clone, Debug, Vertex)]
struct GeometryVertex {
  #[vertex(2, F32)]
  pub position: Vec2,
  #[vertex(4, U8, normalize)]
  pub color: Color32,
}

impl GeometryBatch {
  /// Creates a new geometry batch.
  pub fn new() -> Result<Self, MeshError> {
    Ok(Self {
      mesh: Mesh::new(BufferUsage::Dynamic)?,
      vertices: Vec::new(),
      indices: Vec::new(),
      material: None,
    })
  }

  /// Restarts the batch with the given material.
  pub fn begin(&mut self, material: &Material) {
    self.material = Some(material.clone());
    self.vertices.clear();
  }

  /// Draws a shape in the batch.
  pub fn draw_brush(&mut self, brush: &impl GeometryBrush) {
    brush.draw(self);
  }

  /// Draws a triangle in the batch.
  pub fn draw_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: Color32) {
    let base_offset = self.vertices.len() as MeshIndex;

    self.vertices.push(GeometryVertex { position: a, color });
    self.vertices.push(GeometryVertex { position: b, color });
    self.vertices.push(GeometryVertex { position: c, color });

    self.indices.push(base_offset);
    self.indices.push(base_offset + 1);
    self.indices.push(base_offset + 2);
  }

  /// Draws a strip of triangles in the batch.
  pub fn draw_triangle_strip(&mut self, points: &[Vec2], color: Color32) {
    if points.len() < 3 {
      return;
    }

    let base_offset = self.vertices.len() as MeshIndex;

    self.vertices.push(GeometryVertex {
      position: points[0],
      color,
    });

    for i in 1..points.len() - 1 {
      let offset = self.vertices.len() as MeshIndex;

      self.vertices.push(GeometryVertex {
        position: points[i],
        color,
      });

      self.vertices.push(GeometryVertex {
        position: points[i + 1],
        color,
      });

      self.indices.push(base_offset);
      self.indices.push(offset);
      self.indices.push(offset + 1);
    }
  }

  /// Draws a rectangle in the batch.
  pub fn draw_rectangle(&mut self, rectangle: Rectangle, color: Color32) {
    let base_offset = self.vertices.len() as MeshIndex;

    self.vertices.push(GeometryVertex {
      position: rectangle.bottom_left(),
      color,
    });

    self.vertices.push(GeometryVertex {
      position: rectangle.top_left(),
      color,
    });

    self.vertices.push(GeometryVertex {
      position: rectangle.top_right(),
      color,
    });

    self.vertices.push(GeometryVertex {
      position: rectangle.bottom_right(),
      color,
    });

    self.indices.push(base_offset);
    self.indices.push(base_offset + 1);
    self.indices.push(base_offset + 2);

    self.indices.push(base_offset);
    self.indices.push(base_offset + 2);
    self.indices.push(base_offset + 3);
  }

  /// Draws a circle in the batch.
  pub fn draw_circle(&mut self, center: Vec2, radius: f32, segments: u16, color: Color32) {
    let mut points = Vec::with_capacity(segments as usize);

    for i in 0..segments {
      let angle = i as f32 / segments as f32 * std::f32::consts::PI * 2.0;

      let x = center.x + radius * angle.cos();
      let y = center.y + radius * angle.sin();

      points.push(vec2(x, y));
    }

    self.draw_triangle_strip(&points, color);
  }

  /// Flushes the batch content to the GPU.
  pub fn flush(&mut self) {
    // ensure we're in a valid state to render something
    if self.vertices.is_empty() {
      return;
    };

    if self.indices.is_empty() {
      return;
    };

    if self.material.is_none() {
      return;
    }

    // upload and draw the mesh
    self.mesh.with_buffers(|vertices, indices| {
      vertices.write_data(&self.vertices);
      indices.write_data(&self.indices);
    });

    self.mesh.draw_sub(
      self.material.as_mut().unwrap(),
      PrimitiveTopology::Triangles,
      self.vertices.len(),
      self.indices.len(),
    );

    self.vertices.clear();
    self.indices.clear();
  }
}

/// Allows a type to be converted into a geometry and drawn in a batch.
pub trait GeometryBrush {
  /// Draws the geometry into the given batch.
  fn draw(&self, batch: &mut GeometryBatch);
}

impl GeometryBrush for Line {
  fn draw(&self, _batch: &mut GeometryBatch) {
    todo!()
  }
}

impl GeometryBrush for Rectangle {
  fn draw(&self, batch: &mut GeometryBatch) {
    batch.draw_rectangle(*self, Color32::WHITE);
  }
}

impl GeometryBrush for Circle {
  fn draw(&self, batch: &mut GeometryBatch) {
    batch.draw_circle(self.center, self.radius, 32, Color32::WHITE);
  }
}
