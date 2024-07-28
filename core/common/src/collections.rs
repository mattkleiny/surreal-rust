//! Collections and data structures.

use std::{
  collections::{HashMap, HashSet},
  hash::BuildHasherDefault,
};

pub use anymap::*;
pub use arena::*;
pub use array::*;
pub use graphs::*;
pub use grids::*;
pub use multimap::*;
pub use priorityqueue::*;
pub use quadtree::*;
pub use ringbuffer::*;
pub use smallvec::{smallvec, SmallVec};
pub use spatialhash::*;

mod anymap;
mod arena;
mod array;
mod graphs;
mod grids;
mod multimap;
mod priorityqueue;
mod quadtree;
mod ringbuffer;
mod spatialhash;

/// A faster hasher that is not resilient to DoS attacks.
type FastHasher = BuildHasherDefault<rustc_hash::FxHasher>;

/// A faster hash set that is not resilient to DoS attacks.
pub type FastHashSet<K> = HashSet<K, FastHasher>;

/// A faster hash map that is not resilient to DoS attacks.
pub type FastHashMap<K, V> = HashMap<K, V, FastHasher>;

/// A faster multimap that is not resilient to DoS attacks.
pub type FastMultiMap<K, V> = MultiMap<K, V, FastHasher>;

/// A faster any-map that is not resilient to DoS attacks.
pub type FastAnyMap = AnyMap<FastHasher>;

/// A faster spatial hash grid that is not resilient to DoS attacks.
pub type FastSpatialHashMap<T> = SpatialHashMap<T, FastHasher>;
