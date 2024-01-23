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

  /// Is the grid empty of items?
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
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
  ///
  /// # Safety
  /// This method will panic if the given position is out of bounds. It's the
  /// responsibility of the caller to ensure the position is in-bounds and
  /// thus skip the bounds check.
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
  ///
  /// # Safety
  /// This method will panic if the given position is out of bounds. It's the
  /// responsibility of the caller to ensure the position is in-bounds and
  /// thus skip the bounds check.
  #[inline]
  pub unsafe fn set_unchecked(&mut self, x: i32, y: i32, value: T) {
    self.items[x as usize + y as usize * self.stride] = value
  }

  /// Swaps two items in the grid.
  pub fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
    if self.is_valid_position(x1, y1) && self.is_valid_position(x2, y2) {
      let index1 = x1 as usize + y1 as usize * self.stride;
      let index2 = x2 as usize + y2 as usize * self.stride;

      self.items.swap(index1, index2);
    }
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
    for y in 0..self.height() {
      for x in 0..self.width() / 2 {
        self.swap(x as i32, y as i32, (self.width() - x - 1) as i32, y as i32);
      }
    }
  }

  /// Flips the grid vertically.
  pub fn flip_vertically(&mut self) {
    for y in 0..self.height() / 2 {
      for x in 0..self.width() {
        self.swap(x as i32, y as i32, x as i32, (self.height() - y - 1) as i32);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_valid_position() {
    let mut grid = Grid::new(3, 3);
    grid.set(1, 1, 42);

    assert_eq!(grid.get(1, 1), Some(&42));
  }

  #[test]
  fn test_get_invalid_position() {
    let grid = Grid::<usize>::new(3, 3);

    assert_eq!(grid.get(4, 4), None);
  }

  #[test]
  fn test_get_unchecked() {
    let mut grid = Grid::new(3, 3);
    grid.set(1, 1, 42);

    unsafe {
      assert_eq!(grid.get_unchecked(1, 1), &42);
    }
  }

  #[test]
  #[should_panic]
  fn test_get_unchecked_out_of_bounds() {
    let grid = Grid::<usize>::new(3, 3);

    unsafe {
      grid.get_unchecked(4, 4);
    }
  }

  #[test]
  fn test_set_valid_position() {
    let mut grid = Grid::new(3, 3);
    grid.set(1, 1, 42);

    assert_eq!(grid.get(1, 1), Some(&42));
  }

  #[test]
  fn test_set_invalid_position() {
    let mut grid = Grid::new(3, 3);
    grid.set(4, 4, 42);

    assert_eq!(grid.get(4, 4), None);
  }

  #[test]
  fn test_set_unchecked() {
    let mut grid = Grid::new(3, 3);

    unsafe {
      grid.set_unchecked(1, 1, 42);
    }

    assert_eq!(grid.get(1, 1), Some(&42));
  }

  #[test]
  #[should_panic]
  fn test_set_unchecked_out_of_bounds() {
    let mut grid = Grid::new(3, 3);

    unsafe {
      grid.set_unchecked(4, 4, 42);
    }
  }
}
