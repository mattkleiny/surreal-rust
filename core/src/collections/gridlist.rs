/// A grid list is a list of items that are stored in a grid structure.
#[derive(Clone)]
pub struct GridList<T> {
  grid: Vec<Vec<T>>,
  width: usize,
  height: usize,
}

impl<T: Clone + PartialEq> GridList<T> {
  /// Creates a new grid list.
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      grid: vec![Vec::with_capacity(0); width * height],
      width,
      height,
    }
  }

  /// Returns the width of the grid list.
  pub fn width(&self) -> usize {
    self.width
  }

  /// Returns the height of the grid list.
  pub fn height(&self) -> usize {
    self.height
  }

  /// Returns true if the grid list is empty.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns the number of elements in the grid list.
  pub fn len(&self) -> usize {
    self.grid.iter().map(|v| v.len()).sum()
  }

  /// Adds an item to the grid list
  pub fn add(&mut self, x: usize, y: usize, item: T) {
    self.grid[x + y * self.width].push(item);
  }

  /// Removes an item from the grid list.
  pub fn remove(&mut self, _item: T) -> Option<T> {
    todo!()
  }

  /// Returns an iterator over the items in the grid list.
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.grid.iter().flatten()
  }

  /// Returns a mutable iterator over the items in the grid list.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    self.grid.iter_mut().flatten()
  }
}
