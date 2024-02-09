/// A free list for managing a pool of reusable indices.
pub struct FreeList<T> {
  entries: Vec<Entry<T>>,
  free: Option<u32>,
  len: usize,
}

/// An entry in the free list.
enum Entry<T> {
  Occupied(T),
  Free(Option<u32>),
}

impl<T> Default for FreeList<T> {
  fn default() -> Self {
    Self {
      entries: Vec::default(),
      free: None,
      len: 0,
    }
  }
}

impl<T> FreeList<T> {
  /// Creates a new empty free list.
  pub fn new() -> Self {
    FreeList::default()
  }

  /// Creates a new free list with the given capacity.
  pub fn with_capacity(capacity: usize) -> Self {
    let mut list = FreeList::new();
    list.reserve(capacity);
    list
  }

  /// Reserves additional capacity for the free list.
  pub fn reserve(&mut self, additional: usize) {
    self.entries.reserve(additional);

    while self.entries.len() < self.entries.capacity() {
      let index = self.entries.len() as u32;
      let next_free = std::mem::replace(&mut self.free, Some(index));

      self.entries.push(Entry::Free(next_free));
    }
  }

  /// Returns the capacity of the free list.
  pub fn capacity(&self) -> usize {
    self.entries.capacity()
  }

  /// Returns the number of elements in the free list.
  pub fn len(&self) -> usize {
    self.len
  }

  /// Attempts to allocate a new index for the given value.
  pub fn try_alloc(&mut self, value: T) -> Result<u32, T> {
    if let Some(index) = self.free {
      let next_free = match self.entries[index as usize] {
        Entry::Free(next_free) => next_free,
        Entry::Occupied { .. } => unreachable!(),
      };

      self.free = next_free;
      self.entries[index as usize] = Entry::Occupied(value);
      self.len += 1;

      Ok(index)
    } else {
      Err(value)
    }
  }

  /// Allocates a new index for the given value.
  pub fn alloc(&mut self, value: T) -> u32 {
    self.try_alloc(value).unwrap_or_else(|value| {
      self.reserve(1);
      self.len += 1;

      let index = self.entries.len();
      self.entries.push(Entry::Occupied(value));

      index as u32
    })
  }

  /// Returns a reference to the value at the given index.
  pub fn get(&self, index: u32) -> &T {
    match &self.entries[index as usize] {
      Entry::Occupied(x) => x,
      Entry::Free(_) => unreachable!(),
    }
  }

  /// Returns a mutable reference to the value at the given index.
  pub fn get_mut(&mut self, index: u32) -> &mut T {
    match &mut self.entries[index as usize] {
      Entry::Occupied(x) => x,
      Entry::Free(_) => unreachable!(),
    }
  }

  /// Returns an iterator over the occupied entries in the free list.
  pub fn iter(&self) -> impl Iterator<Item = (u32, &T)> + '_ {
    self.entries.iter().enumerate().filter_map(|(i, e)| match e {
      Entry::Occupied(x) => Some((u32::try_from(i).unwrap(), x)),
      Entry::Free(_) => None,
    })
  }

  /// Deallocates the given index, making it available for reuse.
  pub fn dealloc(&mut self, index: u32) {
    match &mut self.entries[index as usize] {
      Entry::Free(_) => {}
      entry @ Entry::Occupied(_) => {
        let next_free = std::mem::replace(&mut self.free, Some(index));
        *entry = Entry::Free(next_free);

        self.len -= 1;
      }
    }
  }
}
