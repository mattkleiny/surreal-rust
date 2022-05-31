use crate::maths::{Vector2, vec2};

use super::*;

// Represents a point in the batch.
type Point = Vector2<f32>;

/// A shape that can be emitted in a geometry batch.
pub trait Shape {
  /// Emits the given shape into the geometry batch.
  fn emit(&self, batch: &mut GeometryBatch);
}

/// A single command in the geometry batch
enum Command<'a> {
  DrawLine(Point, Point),
  DrawTriangle(Point, Point, Point),
  DrawTriangleStrip(Vec<Point>),
  DrawQuad(Point, Point, Point, Point),
  DrawCircle(Point, f32, u16),
  DrawShape(Box<dyn Shape>),
  DrawSprite {
    texture: &'a TextureRegion<'a>,
    tint: Color32,
    position: Point,
    rotation: f32,
  },
}

/// A fast and lightweight geometry batch renderer.
///
/// This batch pre-allocates an array of vertices and re-uses it to tessellate shapes and polygons.
pub struct GeometryBatch<'a> {
  mesh: Mesh<Vertex2>,
  commands: Vec<Command<'a>>,
  vertices: Vec<Vertex2>,
  indices: Vec<Index>,
  material: Option<Material>,
}

impl<'a> GeometryBatch<'a> {
  /// Creates a new geometry batch.
  pub fn new(server: &GraphicsServer) -> Self {
    Self {
      mesh: Mesh::new(server, BufferUsage::Dynamic),
      commands: Vec::new(),
      vertices: Vec::new(),
      indices: Vec::new(),
      material: None,
    }
  }

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
  
  /// Draws a shape in the batch.
  pub fn draw_shape(&mut self, shape: impl Shape + 'static) {
    self.commands.push(Command::DrawShape(Box::new(shape)));
  }
  
  /// Draws a sprite in the batch.
  pub fn draw_sprite(&mut self, texture: &'a TextureRegion<'a>, position: Point, rotation: f32, tint: Color32) {
    self.commands.push(Command::DrawSprite { texture, tint, position, rotation });
  }

  /// Flushes the batch content to the GPU.
  pub fn flush(&mut self) {
    // tesselate all commands into vertices, first
    while let Some(command) = self.commands.pop() {
      self.tessellate(command);
    }

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

  /// Tessellates the given command into vertices in the buffer
  fn tessellate(&mut self, command: Command) {
    let offset = self.vertices.len() as Index;
 
    match command {
      Command::DrawLine(a, b) => {
        // TODO: get the winding order correct?
        self.vertices.push(Vertex2 { position: a, uv: vec2(0., 0.), color: Color32::WHITE });
        self.vertices.push(Vertex2 { position: b, uv: vec2(1., 1.), color: Color32::WHITE });
        
        self.indices.push(offset + 0);
        self.indices.push(offset + 1);
      },
      Command::DrawTriangle(a, b, c) => {
        self.vertices.push(Vertex2 { position: a, uv: vec2(0., 0.), color: Color32::WHITE });
        self.vertices.push(Vertex2 { position: b, uv: vec2(0.5, 1.), color: Color32::WHITE });
        self.vertices.push(Vertex2 { position: c, uv: vec2(1., 0.), color: Color32::WHITE });

        self.indices.push(offset + 0);
        self.indices.push(offset + 1);
        self.indices.push(offset + 2);
      },
      Command::DrawTriangleStrip(_vertices) => {
        todo!()
      },
      Command::DrawQuad(a, b, c, d) => {
        self.vertices.push(Vertex2 { position: a, uv: vec2(0., 0.), color: Color32::WHITE });
        self.vertices.push(Vertex2 { position: b, uv: vec2(0., 1.), color: Color32::WHITE });
        self.vertices.push(Vertex2 { position: c, uv: vec2(1., 1.), color: Color32::WHITE });
        self.vertices.push(Vertex2 { position: d, uv: vec2(1., 0.), color: Color32::WHITE });

        self.indices.push(offset + 0);
        self.indices.push(offset + 1);
        self.indices.push(offset + 2);

        self.indices.push(offset + 0);
        self.indices.push(offset + 2);
        self.indices.push(offset + 3);
      },
      Command::DrawCircle(center, radius, segments) => {
        todo!()
      },
      Command::DrawShape(shape) => {
        shape.emit(self)
      },
      Command::DrawSprite { texture, tint, position, rotation } => {
        todo!()
      }
    }
  }
}