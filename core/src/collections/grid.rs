use crate::maths::Shape;

/// A simple 2d grid of [`T`]s.
#[derive(Clone, Debug)]
pub struct Grid<T> {
  stride: usize,
  items: Vec<T>,
}

impl<T> Grid<T> {
  /// Creates a new grid with the given dimensions.
  pub fn new(width: usize, height: usize) -> Self
  where
    T: Clone + Default,
  {
    Self {
      stride: width,
      items: vec![T::default(); width * height],
    }
  }

  /// Converts the given slice into a grid.
  pub fn from_slice(stride: usize, slice: &[T]) -> Self
  where
    T: Clone,
  {
    Self {
      stride,
      items: slice.to_vec(),
    }
  }

  /// Returns the total length of the grid (width * height).
  #[inline]
  pub fn len(&self) -> usize {
    self.items.len()
  }

  /// Returns the stride/size between each row of the grid.
  #[inline]
  pub fn stride(&self) -> usize {
    self.stride
  }

  /// Returns the width of the grid.
  #[inline]
  pub fn width(&self) -> usize {
    self.stride()
  }

  /// Returns the height of the grid.
  #[inline]
  pub fn height(&self) -> usize {
    self.len() / self.stride()
  }

  /// Is the given point a valid index into the grid?
  #[inline]
  pub fn is_valid(&self, x: i32, y: i32) -> bool {
    self.is_valid_position(x, y)
  }

  /// Is the given position a valid index into the grid?
  #[inline]
  pub fn is_valid_position(&self, x: i32, y: i32) -> bool {
    x >= 0 && x < self.width() as i32 && y >= 0 && y < self.height() as i32
  }

  /// Accesses an item from the grid.
  #[inline]
  pub fn get(&self, x: i32, y: i32) -> Option<&T> {
    if self.is_valid_position(x, y) {
      self.items.get(x as usize + y as usize * self.stride)
    } else {
      None
    }
  }

  /// Accesses an item from the grid without checking bounds.
  #[inline]
  pub unsafe fn get_unchecked(&self, x: i32, y: i32) -> &T {
    &self.items[x as usize + y as usize * self.stride]
  }

  /// Sets an item from the grid.
  #[inline]
  pub fn set(&mut self, x: i32, y: i32, value: T) {
    if self.is_valid_position(x, y) {
      self.items[x as usize + y as usize * self.stride] = value
    }
  }

  /// Sets an item from the grid without checking bounds.
  #[inline]
  pub unsafe fn set_unchecked(&mut self, x: i32, y: i32, value: T) {
    self.items[x as usize + y as usize * self.stride] = value
  }

  /// Fills the grid with the given value.
  pub fn fill(&mut self, value: T)
  where
    T: Clone,
  {
    self.items.fill(value);
  }

  /// Clears the grid.
  pub fn clear(&mut self)
  where
    T: Clone + Default,
  {
    self.fill(T::default());
  }

  /// Returns the items as a slice.
  pub fn as_slice(&self) -> &[T] {
    self.items.as_slice()
  }

  /// Returns the items as a mutable slice.
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    self.items.as_mut_slice()
  }

  /// Flips the grid horizontally.
  pub fn flip_horizontally(&mut self) {
    todo!()
  }

  /// Flips the grid vertically.
  pub fn flip_vertically(&mut self) {
    todo!()
  }

  /// Mirrors the grid horizontally.
  pub fn mirror_horizontal(&mut self) {
    todo!()
  }

  /// Mirrors the grid vertically.
  pub fn mirror_vertically(&mut self) {
    todo!()
  }

  /// Rasterizes the given shape into the canvas.
  pub fn draw(&mut self, value: T, shape: &impl Shape)
  where
    T: Clone,
  {
    shape.rasterize(value, self);
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
        grid.set(x as i32, y as i32, Color32::random());
      }
    }
  }
}
