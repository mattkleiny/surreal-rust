use super::*;

/// A shape that can be drawn to a canvas.
pub trait CanvasShape {}

/// A canvas that can be drawn to procedurally.
///
/// A canvas implements primitive routines that make it simple to compose images
/// from parts. It can be used as a foundation for higher-level rendering tasks.
pub struct Canvas {
  graphics: GraphicsServer,
}

impl Canvas {
  /// Creates a new canvas.
  pub fn new(graphics: &GraphicsServer) -> Canvas {
    Canvas {
      graphics: graphics.clone(),
    }
  }

  /// Begins a new frame. This should be called before any drawing operations.
  pub fn begin(&mut self) {
    todo!()
  }

  /// Flushes the canvas to the screen.
  pub fn flush(&mut self) {
    todo!()
  }

  /// Clears the canvas to the given color.
  pub fn clear(&mut self, color: Color) {
    todo!()
  }

  /// Draws a line to the canvas.
  pub fn draw_line(&mut self, start: Point, end: Point, color: Color) {
    todo!()
  }

  /// Draws a rectangle to the canvas.
  pub fn draw_rect(&mut self, rect: Rect, color: Color) {
    todo!()
  }

  /// Draws a circle to the canvas.
  pub fn draw_circle(&mut self, center: Point, radius: f32, color: Color) {
    todo!()
  }

  /// Draws a shape to the canvas.
  pub fn draw_shape(&mut self, shape: &impl CanvasShape) {
    todo!()
  }

  /// Draws a string of text to the canvas.
  pub fn draw_text(&mut self, text: &str, position: Point, color: Color) {
    todo!()
  }

  /// Draws an image to the canvas.
  pub fn draw_image(&mut self, image: &Image, position: Point) {
    todo!()
  }

  /// Draws a portion of an image to the canvas.
  pub fn draw_image_rect(&mut self, image: &Image, source: Rect, destination: Rect) {
    todo!()
  }
}
