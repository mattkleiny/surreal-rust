use super::*;

/// A bounded rectangle in 2 dimensions formed from the two corner points.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rectangle<T> {
  min: Vector2<T>,
  max: Vector2<T>,
}

impl<T: Numeric> Rectangle<T> {
  /// Creates a new rectangle from the given corner points.
  pub fn new(min: Vector2<T>, max: Vector2<T>) -> Self {
    Self { min, max }
  }

  /// Creates a new rectangle from the given center and size.
  pub fn from_size(center: Vector2<T>, size: Vector2<T>) -> Self {
    Self::new(center - size / T::from_f32(2.), center + size / T::from_f32(2.))
  }

  /// Creates a new rectangle from the given corner points.
  pub fn from_corner_points(left: T, top: T, right: T, bottom: T) -> Self {
    Self::new(vec2(left, top), vec2(right, bottom))
  }

  /// The minimum corner of the rectangle.
  pub fn min(&self) -> Vector2<T> {
    self.min
  }

  /// The maximum corner of the rectangle.
  pub fn max(&self) -> Vector2<T> {
    self.max
  }

  /// The left hand side of the rectangle.
  pub fn left(&self) -> T {
    self.min.x
  }

  /// The right hand side of the rectangle.
  pub fn right(&self) -> T {
    self.max.x
  }

  /// The top hand side of the rectangle.
  pub fn top(&self) -> T {
    self.min.y
  }

  /// The bottom hand side of the rectangle.
  pub fn bottom(&self) -> T {
    self.max.y
  }

  /// The top left corner of the rectangle.
  pub fn top_left(&self) -> Vector2<T> {
    vec2(self.left(), self.top())
  }

  /// The top right corner of the rectangle.
  pub fn top_right(&self) -> Vector2<T> {
    vec2(self.right(), self.top())
  }

  /// The bottom left corner of the rectangle.
  pub fn bottom_left(&self) -> Vector2<T> {
    vec2(self.left(), self.bottom())
  }

  /// The bottom right corner of the rectangle.
  pub fn bottom_right(&self) -> Vector2<T> {
    vec2(self.right(), self.bottom())
  }

  /// The width of the rectangle.
  pub fn width(&self) -> T {
    self.right() - self.left()
  }

  /// The height of the rectangle.
  pub fn height(&self) -> T {
    self.bottom() - self.top()
  }

  /// The total area of the rectangle.
  pub fn area(&self) -> T {
    self.width() * self.height()
  }

  /// The size of the rectangle in vector form.
  pub fn size(&self) -> Vector2<T> {
    vec2(self.width(), self.height())
  }

  /// Creates a new rectangle clamped to the given (X, Y) bounds.
  pub fn clamp(&self, left: T, top: T, right: T, bottom: T) -> Self {
    Self::from_corner_points(
      clamp(self.left(), left, right),
      clamp(self.top(), top, bottom),
      clamp(self.right(), left, right),
      clamp(self.bottom(), top, bottom),
    )
  }

  /// Determines if the rectangle contains the given point.
  pub fn contains_point(&self, point: Vector2<T>) -> bool {
    point.x >= self.min.x && point.y >= self.min.y && point.y <= self.max.y && point.y <= self.max.y
  }
}

/// Allows rasterization of rectangles into canvases.
macro_rules! implement_rect_raster {
  ($type:ty) => {
    impl Rasterable for Rectangle<$type> {
      fn rasterize<T: Clone>(&self, value: T, canvas: &mut impl RasterCanvas<T>) {
        for y in self.top()..self.bottom() {
          for x in self.left()..self.right() {
            canvas.set((x, y), value.clone());
          }
        }
      }
    }
  };
}

implement_rect_raster!(u8);
implement_rect_raster!(u16);
implement_rect_raster!(u32);
implement_rect_raster!(u64);
implement_rect_raster!(usize);
implement_rect_raster!(i8);
implement_rect_raster!(i16);
implement_rect_raster!(i32);
implement_rect_raster!(i64);
implement_rect_raster!(isize);

impl Rasterable for Rectangle<f32> {
  fn rasterize<T: Clone>(&self, value: T, canvas: &mut impl RasterCanvas<T>) {
    let top = self.top().floor() as isize;
    let bottom = self.bottom().ceil() as isize;
    let left = self.left().floor() as isize;
    let right = self.right().ceil() as isize;

    for y in top..bottom {
      for x in left..right {
        canvas.set((x, y), value.clone());
      }
    }
  }
}

impl Rasterable for Rectangle<f64> {
  fn rasterize<T: Clone>(&self, value: T, canvas: &mut impl RasterCanvas<T>) {
    let top = self.top().floor() as isize;
    let bottom = self.bottom().ceil() as isize;
    let left = self.left().floor() as isize;
    let right = self.right().ceil() as isize;

    for y in top..bottom {
      for x in left..right {
        canvas.set((x, y), value.clone());
      }
    }
  }
}
