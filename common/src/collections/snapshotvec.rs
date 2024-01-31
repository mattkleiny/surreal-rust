use std::{fmt::Debug, sync::RwLock};

/// A vec that supports simultaneous reads and writes.
///
/// This is useful for cases where you want to read from a vec while also
/// writing to it. For example, you might want to iterate over a vec while
/// also pushing to it.
pub struct SnapshotVec<T> {
  data: RwLock<Vec<T>>,
  snapshot: Option<Vec<T>>,
}

impl<T> SnapshotVec<T> {
  pub fn new() -> Self {
    Self {
      data: RwLock::new(Vec::new()),
      snapshot: None,
    }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      data: RwLock::new(Vec::with_capacity(capacity)),
      snapshot: None,
    }
  }

  pub fn len(&self) -> usize {
    if let Some(snapshot) = &self.snapshot {
      snapshot.len()
    } else {
      self.data.read().unwrap().len()
    }
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn push(&mut self, _value: T) {
    todo!()
  }

  pub fn pop(&mut self) -> Option<T> {
    todo!()
  }

  pub fn clear(&mut self) {
    todo!()
  }

  pub fn snapshot(&mut self) {
    todo!()
  }
}

impl<T> Default for SnapshotVec<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: Debug> Debug for SnapshotVec<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_list().entries(self.snapshot.iter()).finish()
  }
}

unsafe impl<T> Send for SnapshotVec<T> where T: Send {}
unsafe impl<T> Sync for SnapshotVec<T> where T: Sync {}
