use super::{vec2, RasterCanvas, Rasterable, Rectangle, Vector2};

/// A simple circle in 2-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Circle<N> {
  pub center: Vector2<N>,
  pub radius: N,
}

/// Allow rasterization of integrally sized circles.
impl Rasterable for Circle<isize> {
  fn rasterize<T: Clone>(&self, value: T, canvas: &mut impl RasterCanvas<T>) {
    let center = self.center;
    let radius = self.radius;

    let size = vec2(radius, radius);
    let rectangle = Rectangle::from_size(center, size);
    let rectangle = rectangle.clamp(0, 0, canvas.width() as isize - 1, canvas.height() as isize - 1);

    for y in rectangle.top()..rectangle.bottom() {
      for x in rectangle.left()..rectangle.right() {
        let point = vec2(x, y);
        if (point - center).length_squared() <= radius {
          canvas.set(point, value.clone());
        }
      }
    }
  }
}
