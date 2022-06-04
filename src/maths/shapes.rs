use crate::maths::{vec2, Grid, Raster, Rectangle, Vector2};

/// A simple circle in 2-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Circle {
  pub center: Vector2<isize>,
  pub radius: isize,
}

impl Raster for Circle {
  fn draw_to<G: Grid<T>, T: Clone>(&self, grid: &mut G, value: T) {
    let center = self.center;
    let radius = self.radius;

    let size = vec2(radius, radius);
    let rectangle = Rectangle::from_size(center, size).clamp(
      0,
      0,
      grid.width() as isize - 1,
      grid.height() as isize - 1,
    );

    for y in rectangle.top()..rectangle.bottom() {
      for x in rectangle.left()..rectangle.right() {
        let point = vec2(x, y);

        if (point - center).length_squared() <= radius {
          grid.set(point, value.clone());
        }
      }
    }
  }
}
