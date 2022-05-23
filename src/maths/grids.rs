use std::ops::{Index, IndexMut};

use crate::maths::{Rectangle, vec2, Vector2};

// TODO: think up a smarter trait for this?

/// A simple 2d grid that allows efficient random access.
pub struct Grid<T> {
  stride: usize,
  items: Vec<T>,
}

impl<T> Grid<T> {
  /// Creates a new grid with the given dimensions.
  pub fn new(width: usize, height: usize) -> Self where T: Clone + Default {
    Self {
      stride: width,
      items: vec![T::default(); width * height],
    }
  }

  /// Converts the given slice into a grid.
  pub fn from_slice(stride: usize, slice: &[T]) -> Self where T: Clone {
    Self {
      stride,
      items: slice.to_vec(),
    }
  }

  /// Returns the stride/size between each row of the grid.
  pub fn stride(&self) -> usize {
    self.stride
  }

  /// Returns the total length of the grid (width * height).
  pub fn length(&self) -> usize {
    self.items.len()
  }

  /// Returns the width of the grid.
  pub fn width(&self) -> usize {
    self.stride
  }

  /// Returns the height of the grid.
  pub fn height(&self) -> usize {
    self.length() / self.stride()
  }

  /// Fills the grid with the given value.
  pub fn fill(&mut self, value: T) where T: Clone {
    self.items.fill(value);
  }

  /// Returns the items as a slice.
  pub fn as_slice(&self) -> &[T] {
    self.items.as_slice()
  }

  /// Returns the items as a mutable slice.
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    self.items.as_mut_slice()
  }

  /// Draws a circle in the grid.
  pub fn draw_circle(&mut self, center: Vector2<isize>, radius: isize, value: T) where T: Copy {
    let rectangle = Rectangle::from_size(center, vec2(radius, radius))
      .clamp(0, 0, self.width() as isize - 1, self.height() as isize - 1);

    for y in rectangle.top()..rectangle.bottom() {
      for x in rectangle.left()..rectangle.right() {
        let point = vec2(x, y);

        if (point - center).length_squared() <= radius {
          self[(point.x as usize, point.y as usize)] = value;
        }
      }
    }
  }
}

impl<T> Index<(usize, usize)> for Grid<T> {
  type Output = T;

  fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
    &self.items[x + y * self.stride]
  }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
  fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
    &mut self.items[x + y * self.stride]
  }
}

#[cfg(test)]
mod tests {
  use crate::graphics::Color32;
  use crate::maths::FromRandom;

  use super::*;

  #[test]
  fn grid_should_read_and_write_elements() {
    let mut grid = Grid::new(128, 128);

    grid.fill(Color32::BLACK);

    for y in 0..grid.height() {
      for x in 0..grid.width() {
        grid[(x, y)] = Color32::random();
      }
    }
  }
}