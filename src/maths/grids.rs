use std::ops::{Index, IndexMut};

/// A grid allows random access to 2d data.
pub trait Grid<T> {
  /// Returns the stride/size between each row of the grid.
  fn stride(&self) -> usize;

  /// Returns the total length of the grid (width * height).
  fn length(&self) -> usize;

  /// Returns the width of the grid.
  fn width(&self) -> usize { self.length() % self.stride() }

  /// Returns the height of the grid.
  fn height(&self) -> usize { self.length() / self.stride() }
}

/// A grid slice is a [T] that can be used in a grid format.
#[derive(Debug)]
pub struct GridSlice<'a, T> {
  slice: &'a [T],
  stride: usize,
}

impl<'a, T> GridSlice<'a, T> {
  /// Creates a new grid slice from the given slice.
  pub fn from(slice: &'a [T], stride: usize) -> Self {
    Self {
      slice,
      stride,
    }
  }
}

impl<'a, T> Grid<T> for GridSlice<'a, T> {
  fn stride(&self) -> usize {
    self.stride
  }

  fn length(&self) -> usize {
    self.slice.len()
  }
}

impl<'a, T> Index<(usize, usize)> for GridSlice<'a, T> {
  type Output = T;

  fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
    &self.slice[x + y * self.stride]
  }
}

/// A grid slice mut is a mut [T] that can be used in a grid format.
#[derive(Debug)]
pub struct GridSliceMut<'a, T> {
  slice: &'a mut [T],
  stride: usize,
}

impl<'a, T> GridSliceMut<'a, T> {
  /// Creates a new grid slice from the given slice.
  pub fn from(slice: &'a mut [T], stride: usize) -> Self {
    Self {
      slice,
      stride
    }
  }
}

impl<'a, T> Grid<T> for GridSliceMut<'a, T> {
  fn stride(&self) -> usize {
    self.stride
  }

  fn length(&self) -> usize {
    self.slice.len()
  }
}

impl<'a, T> Index<(usize, usize)> for GridSliceMut<'a, T> {
  type Output = T;

  fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
    &self.slice[x + y * self.stride]
  }
}

impl<'a, T> IndexMut<(usize, usize)> for GridSliceMut<'a, T> {
  fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
    &mut self.slice[x + y * self.stride]
  }
}

/// Allows conversion to a `GridSlice`.
pub trait ToGridSlice {
  type Element;

  fn to_grid_slice(&self, stride: usize) -> GridSlice<Self::Element>;
  fn to_grid_slice_mut(&mut self, stride: usize) -> GridSliceMut<Self::Element>;
}

impl<T> ToGridSlice for [T] {
  type Element = T;

  fn to_grid_slice(&self, stride: usize) -> GridSlice<Self::Element> {
    GridSlice::from(&self[..], stride)
  }

  fn to_grid_slice_mut(&mut self, stride: usize) -> GridSliceMut<Self::Element> {
    GridSliceMut::from(&mut self[..], stride)
  }
}

impl<T> ToGridSlice for &mut [T] {
  type Element = T;

  fn to_grid_slice(&self, stride: usize) -> GridSlice<Self::Element> {
    GridSlice::from(self, stride)
  }

  fn to_grid_slice_mut(&mut self, stride: usize) -> GridSliceMut<Self::Element> {
    GridSliceMut::from(self, stride)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn grid_slice_should_read_and_write_simple_data() {
    let mut array = [0f32; 64 * 64];
    let mut slice = array.to_grid_slice_mut(64);

    for y in 0..slice.height() {
      for x in 0..slice.width() {
        slice[(x, y)] = x as f32 + y as f32;
      }
    }
  }
}