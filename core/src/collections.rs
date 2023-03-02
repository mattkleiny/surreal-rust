//! Collections and data structures.

pub use anymap::*;
pub use arena::*;
pub use grid::*;
pub use multimap::*;
pub use priorityqueue::*;
pub use resources::*;
pub use ringbuffer::*;
pub use smallvec::{smallvec, SmallVec};

mod anymap;
mod arena;
mod grid;
mod multimap;
mod priorityqueue;
mod resources;
mod ringbuffer;

/// A faster hash set that is not resilient to DoS attacks.
pub type FastHashSet<K> = rustc_hash::FxHashSet<K>;

/// A faster hash map that is not resilient to DoS attacks.
pub type FastHashMap<K, V> = rustc_hash::FxHashMap<K, V>;
