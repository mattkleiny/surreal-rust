use std::ops::{Index, IndexMut};

/// A grid slice is a managed read/write layer over the top of some slice of data.
pub struct GridSlice<'a, T> {
  slice: &'a mut [T],
  width: usize,
  height: usize,
}

impl<'a, T> GridSlice<'a, T> {
  pub fn new(slice: &'a mut [T], stride: usize) -> Self {
    Self {
      width: stride,
      height: slice.len() / stride,
      slice,
    }
  }

  pub fn width(&self) -> usize { self.width }
  pub fn height(&self) -> usize { self.height }

  pub fn fill(&mut self, value: T) where T: Clone {
    self.slice.fill(value);
  }
}

impl<'a, T> Index<(usize, usize)> for GridSlice<'a, T> {
  type Output = T;

  fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
    &self.slice[x + y * self.width]
  }
}

impl<'a, T> IndexMut<(usize, usize)> for GridSlice<'a, T> {
  fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
    &mut self.slice[x + y * self.width]
  }
}

#[cfg(test)]
mod tests {
  use crate::graphics::Color;

  use super::*;

  #[test]
  fn it_should_read_and_write_slice_contents() {
    let mut data = vec![Color::WHITE; 32 * 32];
    let mut slice = GridSlice::new(&mut data, 32);

    for y in 0..slice.height() {
      for x in 0..slice.width() {
        slice[(0, 0)] = Color::MAGENTA;
      }
    }
  }
}