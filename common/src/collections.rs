//! Collections and data structures.

use std::{
  collections::{HashMap, HashSet},
  hash::BuildHasherDefault,
};

pub use anymap::*;
pub use arena::*;
pub use graphs::*;
pub use grid::*;
pub use multimap::*;
pub use priorityqueue::*;
pub use quadtree::*;
pub use ringbuffer::*;
pub use smallvec::{smallvec, SmallVec};
pub use spatialhash::*;

mod anymap;
mod arena;
mod graphs;
mod grid;
mod multimap;
mod priorityqueue;
mod quadtree;
mod ringbuffer;
mod spatialhash;

/// A faster hash set that is not resilient to DoS attacks.
pub type FastHashSet<K> = HashSet<K, BuildHasherDefault<rustc_hash::FxHasher>>;

/// A faster hash map that is not resilient to DoS attacks.
pub type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<rustc_hash::FxHasher>>;

/// A faster multimap that is not resilient to DoS attacks.
pub type FastMultiMap<K, V> = MultiMap<K, V, BuildHasherDefault<rustc_hash::FxHasher>>;

/// A faster any-map that is not resilient to DoS attacks.
pub type FastAnyMap = AnyMap<BuildHasherDefault<rustc_hash::FxHasher>>;

/// A faster spatial hash grid that is not resilient to DoS attacks.
pub type FastSpatialHashGrid<T> = SpatialHashGrid<T, BuildHasherDefault<rustc_hash::FxHasher>>;
