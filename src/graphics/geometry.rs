use crate::maths::Vector2;

use super::*;

// Represents a point in the batch.
type Point = Vector2<f32>;

/// A polygon that can be rendered in a geometry batch.
pub type Polygon = Vec<Vertex2>;

/// A shape that can be emitted in a geometry batch.
pub trait Shape {
  /// Emits the given shape into the geometry batch.
  fn emit(&self, batch: &mut GeometryBatch);
}

/// A single command in the geometry batch
enum Command {
  DrawLine(Point, Point),
  DrawTriangle(Point, Point, Point),
  DrawTriangleStrip(Vec<Point>),
  DrawQuad(Point, Point, Point, Point),
  DrawCircle(Point, f32, u16),
  DrawPolygon(Polygon),
  DrawShape(Box<dyn Shape>),
}

/// A fast and lightweight geometry batch renderer.
///
/// This batch pre-allocates an array of vertices and re-uses it to tessellate shapes and polygons.
pub struct GeometryBatch {
  mesh: Mesh<Vertex2>,
  commands: Vec<Command>,
  vertices: Vec<Vertex2>,
  material: Option<Material>,
}

impl GeometryBatch {
  /// Restarts the batch with the given material.
  pub fn begin(&mut self, material: &Material) {
    self.material = Some(material.clone());
    self.commands.clear();
    self.vertices.clear();
  }

  /// Draws a line in the batch.
  pub fn draw_line(&mut self, a: Point, b: Point) {
    self.commands.push(Command::DrawLine(a, b));
  }
  
  /// Draws a triangle in the batch.
  pub fn draw_triangle(&mut self, a: Point, b: Point, c: Point) {
    self.commands.push(Command::DrawTriangle(a, b, c));
  }
  
  /// Draws a strip of triangles in the batch.
  pub fn draw_triangle_strip(&mut self, points: &[Point]) {
    self.commands.push(Command::DrawTriangleStrip(Vec::from(points)));
  }

  /// Draws a quad in the batch.
  pub fn draw_quad(&mut self, a: Point, b: Point, c: Point, d: Point) {
    self.commands.push(Command::DrawQuad(a, b, c, d));
  }
  
  /// Draws a circle in the batch.
  pub fn draw_circle(&mut self, center: Point, radius: f32, segments: u16) {
    self.commands.push(Command::DrawCircle(center, radius, segments));
  }
  
  /// Draws a polygon in the batch.
  pub fn draw_polygon(&mut self, polygon: Polygon) {
    self.commands.push(Command::DrawPolygon(polygon));
  }
  
  /// Draws a shape in the batch.
  pub fn draw_shape(&mut self, shape: impl Shape + 'static) {
    self.commands.push(Command::DrawShape(Box::new(shape)));
  }

  /// Flushes the batch content to the GPU.
  pub fn flush(&mut self) {
    // make sure we have something to write, first
    if self.commands.len() == 0 { return; }
    let Some(material) = &self.material else { return; };

    // tesselate all commands into vertices, first
    while let Some(command) = self.commands.pop() {
      todo!()
    }

    // upload vertices and draw the mesh
    self.mesh.with_buffers(|vertices, _| {
      vertices.write_data(&self.vertices);
    });

    self.mesh.draw_sub_mesh(
      &material, 
      PrimitiveTopology::Triangles, 
      self.vertices.len(), 
      0 // index count
    );

    self.vertices.clear();
  }

  /// Tessellates the given command into vertices in the buffer
  fn tessellate(&mut self, command: Command) {
    match command {
      Command::DrawLine(a, b) => todo!(),
      Command::DrawTriangle(a, b, c) => todo!(),
      Command::DrawTriangleStrip(vertices) => todo!(),
      Command::DrawQuad(a, b, c, d) => todo!(),
      Command::DrawCircle(center, radius, segments) => todo!(),
      Command::DrawPolygon(vertices) => todo!(),
      Command::DrawShape(shape) => shape.emit(self),
    }
  }
}