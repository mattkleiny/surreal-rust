//! Collections and data structures.

pub use anymap::*;
pub use arena::*;
pub use graphs::*;
pub use grid::*;
pub use multimap::*;
pub use priorityqueue::*;
pub use quadtree::*;
pub use resources::*;
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
mod resources;
mod ringbuffer;
mod spatialhash;

/// A faster hash set that is not resilient to DoS attacks.
pub type FastHashSet<K> = rustc_hash::FxHashSet<K>;

/// A faster hash map that is not resilient to DoS attacks.
pub type FastHashMap<K, V> = rustc_hash::FxHashMap<K, V>;