use crate::collections::Grid;

use super::{vec2, Rectangle, Vector2};

/// Represents a shape that can be rasterized into a [`Grid`].
pub trait Shape {
  fn rasterize<T: Clone>(&self, value: T, target: &mut Grid<T>);
}

/// A simple circle in 2-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Circle<N> {
  pub center: Vector2<N>,
  pub radius: N,
}

/// Allow rasterization of integrally sized circles.
impl Shape for Circle<isize> {
  fn rasterize<T: Clone>(&self, value: T, grid: &mut Grid<T>) {
    let center = self.center;
    let radius = self.radius;

    let size = vec2(radius, radius);
    let rectangle = Rectangle::from_size(center, size);
    let rectangle = rectangle.clamp(0, 0, grid.width() as isize - 1, grid.height() as isize - 1);

    for y in rectangle.top()..rectangle.bottom() {
      for x in rectangle.left()..rectangle.right() {
        let point = vec2(x, y);
        if (point - center).length_squared() <= radius {
          grid.set(x as i32, y as i32, value.clone());
        }
      }
    }
  }
}
