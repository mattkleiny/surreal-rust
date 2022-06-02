use crate::maths::{Grid, GridPoint};

/// A simple 2d array of [`T`]s.
#[derive(Clone, Debug)]
pub struct Array2D<T> {
  stride: usize,
  items: Vec<T>,
}

impl<T> Array2D<T> {
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

  /// Returns the items as a slice.
  pub fn as_slice(&self) -> &[T] {
    self.items.as_slice()
  }

  /// Returns the items as a mutable slice.
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    self.items.as_mut_slice()
  }
}

impl<T> Grid<T> for Array2D<T> {
  fn stride(&self) -> usize {
    self.stride
  }

  fn length(&self) -> usize {
    self.items.len()
  }

  fn get(&self, point: impl Into<GridPoint>) -> &T {
    let (x, y) = point.into();

    &self.items[x + y * self.stride]
  }

  fn set(&mut self, point: impl Into<GridPoint>, value: T) {
    let (x, y) = point.into();

    self.items[x + y * self.stride] = value
  }

  fn fill(&mut self, value: T) where T: Clone {
    self.items.fill(value);
  }
}

#[cfg(test)]
mod tests {
  use crate::graphics::Color32;
  use crate::maths::FromRandom;

  use super::*;

  #[test]
  fn array2d_should_read_and_write_elements() {
    let mut array = Array2D::new(128, 128);

    array.fill(Color32::BLACK);

    for y in 0..array.height() {
      for x in 0..array.width() {
        array.set((x, y), Color32::random());
      }
    }
  }
}