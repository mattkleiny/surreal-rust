use std::collections::BTreeMap;

/// A simple 2d grid of [`T`]s.
#[derive(Clone, Debug)]
pub struct DenseGrid<T> {
  stride: usize,
  items: Vec<T>,
}

impl<T> DenseGrid<T> {
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

/// A sparse grid of [`T`]s.
#[derive(Default, Clone, Debug)]
pub struct SparseGrid<T> {
  entries: BTreeMap<isize, T>,
}

/// A type that contains a position in 2-space.
pub trait Positioned {
  fn x(&self) -> i32;
  fn y(&self) -> i32;
}

impl<T> SparseGrid<T> {
  /// The number of entries in the grid.
  pub fn len(&self) -> usize {
    self.entries.len()
  }

  /// Is the grid empty of items?
  pub fn is_empty(&self) -> bool {
    self.entries.is_empty()
  }

  /// Calculates the width of the grid.
  pub fn width(&self) -> u32
  where
    T: Positioned,
  {
    self.size().0
  }

  /// Calculates the height of the grid.
  pub fn height(&self) -> u32
  where
    T: Positioned,
  {
    self.size().1
  }

  /// Calculates the size of the grid.
  pub fn size(&self) -> (u32, u32)
  where
    T: Positioned,
  {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for entry in self.entries.values() {
      let x = entry.x();
      let y = entry.y();

      if x < min_x {
        min_x = x;
      }

      if x > max_x {
        max_x = x;
      }

      if y < min_y {
        min_y = y;
      }

      if y > max_y {
        max_y = y;
      }
    }

    ((max_x - min_x) as u32, (max_y - min_y) as u32)
  }

  /// Sets the value at the given position.
  pub fn set(&mut self, x: i32, y: i32, value: T) {
    self.entries.insert(xy_hash(x, y), value);
  }

  /// Sets the value at the given position without checking bounds.
  pub fn set_unchecked(&mut self, x: i32, y: i32, value: T) {
    self.entries.insert(xy_hash(x, y), value);
  }

  /// Gets the value at the given pos
  pub fn get(&self, x: i32, y: i32) -> Option<&T> {
    self.entries.get(&xy_hash(x, y))
  }

  /// Gets the value at the given pos without checking bounds.
  pub unsafe fn get_unchecked(&self, x: i32, y: i32) -> &T {
    self.entries.get(&xy_hash(x, y)).unwrap()
  }

  /// Mutably get the value at the given position.
  pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
    self.entries.get_mut(&xy_hash(x, y))
  }

  /// Mutably gets the value at the given pos without checking bounds.
  pub fn get_mut_unchecked(&mut self, x: i32, y: i32) -> &mut T {
    self.entries.get_mut(&xy_hash(x, y)).unwrap()
  }

  /// Clears the grid.
  pub fn clear(&mut self) {
    self.entries.clear();
  }

  /// Iterates over the grid.
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.entries.values()
  }

  /// Mutably iterates over the grid.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    self.entries.values_mut()
  }
}

impl<'a, T> IntoIterator for &'a SparseGrid<T> {
  type Item = &'a T;
  type IntoIter = impl Iterator<Item = Self::Item>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut SparseGrid<T> {
  type Item = &'a mut T;
  type IntoIter = impl Iterator<Item = Self::Item>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

/// Gets the hash for the given position.
#[inline(always)]
const fn xy_hash(x: i32, y: i32) -> isize {
  x as isize | (y as isize) << 32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dense_get_valid_position() {
    let mut grid = DenseGrid::new(3, 3);
    grid.set(1, 1, 42);

    assert_eq!(grid.get(1, 1), Some(&42));
  }

  #[test]
  fn test_dense_get_invalid_position() {
    let grid = DenseGrid::<usize>::new(3, 3);

    assert_eq!(grid.get(4, 4), None);
  }

  #[test]
  fn test_dense_get_unchecked() {
    let mut grid = DenseGrid::new(3, 3);
    grid.set(1, 1, 42);

    unsafe {
      assert_eq!(grid.get_unchecked(1, 1), &42);
    }
  }

  #[test]
  #[should_panic]
  fn test_dense_get_unchecked_out_of_bounds() {
    let grid = DenseGrid::<usize>::new(3, 3);

    unsafe {
      grid.get_unchecked(4, 4);
    }
  }

  #[test]
  fn test_dense_set_valid_position() {
    let mut grid = DenseGrid::new(3, 3);
    grid.set(1, 1, 42);

    assert_eq!(grid.get(1, 1), Some(&42));
  }

  #[test]
  fn test_dense_set_invalid_position() {
    let mut grid = DenseGrid::new(3, 3);
    grid.set(4, 4, 42);

    assert_eq!(grid.get(4, 4), None);
  }

  #[test]
  fn test_dense_set_unchecked() {
    let mut grid = DenseGrid::new(3, 3);

    unsafe {
      grid.set_unchecked(1, 1, 42);
    }

    assert_eq!(grid.get(1, 1), Some(&42));
  }

  #[test]
  #[should_panic]
  fn test_dense_set_unchecked_out_of_bounds() {
    let mut grid = DenseGrid::new(3, 3);

    unsafe {
      grid.set_unchecked(4, 4, 42);
    }
  }
}
