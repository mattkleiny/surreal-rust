//! A simple hierarchical bit-set implementation for use in masks and type filtering.

/// A `BitSet` is a simple set designed to track which indices are placed into it.
///
/// Note, a `BitSet` is limited by design to only `usize**4` indices. Adding
/// beyond this limit will cause the `BitSet` to panic.
#[derive(Clone, Debug, Default)]
pub struct BitSet {
  layer3: usize,
  layer2: Vec<usize>,
  layer1: Vec<usize>,
  layer0: Vec<usize>,
}

impl BitSet {
  pub fn new() -> Self {
    Self {
      layer3: 0,
      layer2: Vec::new(),
      layer1: Vec::new(),
      layer0: Vec::new(),
    }
  }

  pub fn add(&mut self, id: Index) -> &mut Self {
    unimplemented!()
  }

  pub fn remove(&mut self, id: Index) -> &mut Self {
    unimplemented!()
  }

  pub fn contains(&self, id: Index) -> bool {
    unimplemented!()
  }
}

// The number of bits in the world size of the current architecture.
#[cfg(target_pointer_width = "32")]
const BITS: usize = 5;

#[cfg(target_pointer_width = "64")]
const BITS: usize = 6;

/// The type we're using internally for bit-set indexing.
type Index = u64;

/// Allows accessing row properties for a particular index.
trait Row: Sized + Copy {
  /// Location of the bit in the row.
  fn row(self, shift: usize) -> usize;

  /// Index of the row that the bit is in.
  fn offset(self, shift: usize) -> usize;

  /// The bit mask of the row the bit is in.
  #[inline(always)]
  fn mask(self, shift: usize) -> usize {
    1usize << self.row(shift)
  }
}

impl Row for Index {
  #[inline(always)]
  fn row(self, shift: usize) -> usize {
    ((self >> shift) as usize) & ((1 << BITS) - 1)
  }

  #[inline(always)]
  fn offset(self, shift: usize) -> usize {
    self as usize / (1 << shift)
  }
}