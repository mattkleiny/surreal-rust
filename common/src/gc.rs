//! A lightweight garbage collector.

use std::{
  any::{Any, TypeId},
  cell::RefCell,
  fmt::Debug,
  hash::Hash,
  rc::Rc,
  sync::atomic,
};

use crate::{FastHashMap, FreeList};

/// A type that can be traced for garbage collection.
pub trait Trace: Any {
  /// Call `collector.edge(gc)` for each `Gc<T>` reference within `self`.
  fn trace(&self, collector: &mut Collector);
}

/// A reference to a garbage-collected `T`.
pub struct Gc<T> {
  heap_id: u32,
  index: u32,
  _phantom: std::marker::PhantomData<*mut T>,
}

impl<T> Clone for Gc<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T> Copy for Gc<T> {}

impl<T> Debug for Gc<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct(&format!("Gc<{}>", std::any::type_name::<T>()))
      .field("heap_id", &self.heap_id)
      .field("index", &self.index)
      .finish()
  }
}

impl<T> Hash for Gc<T> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.heap_id.hash(state);
    self.index.hash(state);
  }
}

impl<T> PartialEq<Self> for Gc<T> {
  fn eq(&self, other: &Self) -> bool {
    self.heap_id == other.heap_id && self.index == other.index
  }
}

impl<T: Trace> PartialEq<Root<T>> for Gc<T>
where
  T: Trace,
{
  fn eq(&self, other: &Root<T>) -> bool {
    *self == other.unrooted()
  }
}

impl<T: Trace> From<Root<T>> for Gc<T> {
  fn from(root: Root<T>) -> Self {
    root.unrooted()
  }
}

impl<'a, T: Trace> From<&'a Root<T>> for Gc<T> {
  fn from(root: &'a Root<T>) -> Self {
    root.unrooted()
  }
}

/// A set of rooted references to garbage-collected `T`s.
struct RootSet<T: Trace> {
  inner: Rc<RefCell<FreeList<Gc<T>>>>,
}

impl<T: Trace> Clone for RootSet<T> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone(),
    }
  }
}

impl<T: Trace> Default for RootSet<T> {
  fn default() -> Self {
    Self {
      inner: Rc::new(RefCell::new(FreeList::default())),
    }
  }
}

impl<T: Trace> RootSet<T> {
  fn insert(&self, gc: Gc<T>) -> Root<T> {
    let mut inner = self.inner.borrow_mut();
    let index = inner.alloc(gc);

    Root {
      roots: self.clone(),
      index,
    }
  }

  fn remove(&self, index: u32) {
    let mut inner = self.inner.borrow_mut();

    inner.dealloc(index);
  }

  fn trace(&self, collector: &mut Collector) {
    let inner = self.inner.borrow();

    for (_, gc) in inner.iter() {
      collector.edge(*gc);
    }
  }
}

/// A rooted reference to a GC-managed `T`.
pub struct Root<T: Trace> {
  roots: RootSet<T>,
  index: u32,
}

impl<T: Trace> Clone for Root<T> {
  fn clone(&self) -> Self {
    self.roots.insert(self.unrooted())
  }
}

impl<T: Trace> PartialEq<Root<T>> for Root<T> {
  fn eq(&self, other: &Root<T>) -> bool {
    self.unrooted() == other.unrooted()
  }
}

impl<T: Trace> PartialEq<Gc<T>> for Root<T> {
  fn eq(&self, other: &Gc<T>) -> bool {
    self.unrooted() == *other
  }
}

impl<T: Trace> Drop for Root<T> {
  fn drop(&mut self) {
    self.roots.remove(self.index);
  }
}

impl<T: Trace> Root<T> {
  /// Get an unrooted [`Gc<T>`][crate::Gc] reference pointing to the same `T`
  pub fn unrooted(&self) -> Gc<T> {
    let inner = (*self.roots.inner).borrow();

    *inner.get(self.index)
  }
}

struct Arena<T: Trace> {
  roots: RootSet<T>,
  elements: FreeList<T>,
}

// We don't default to 0-capacity arenas because the arenas themselves are
// lazily constructed, and so by the time we are constructing an arena, we will
// always immediately push onto it.
const DEFAULT_ARENA_CAPACITY: usize = 32;

impl<T: Trace> Default for Arena<T> {
  fn default() -> Self {
    Arena {
      roots: RootSet::<T>::default(),
      elements: FreeList::with_capacity(DEFAULT_ARENA_CAPACITY),
    }
  }
}

impl<T: Trace> Arena<T> {
  #[inline]
  fn try_alloc(&mut self, heap_id: u32, value: T) -> Result<Root<T>, T> {
    let index = self.elements.try_alloc(value)?;

    Ok(self.root(Gc {
      heap_id,
      index,
      _phantom: std::marker::PhantomData,
    }))
  }

  fn alloc_slow(&mut self, heap_id: u32, value: T) -> Root<T> {
    if self.elements.len() == self.elements.capacity() {
      let additional = self.elements.len();
      self.elements.reserve(additional);
    }

    let index = self.elements.try_alloc(value).ok().unwrap();

    self.root(Gc {
      heap_id,
      index,
      _phantom: std::marker::PhantomData,
    })
  }

  #[inline]
  fn root(&self, gc: Gc<T>) -> Root<T> {
    self.roots.insert(gc)
  }
}

trait ArenaObject: Any {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;

  fn trace_roots(&self, collector: &mut Collector);
  fn trace_one(&mut self, index: u32, collector: &mut Collector);

  fn capacity(&self) -> usize;

  fn sweep(&mut self, mark_bits: &MarkBits);
}

impl<T: Trace> ArenaObject for Arena<T> {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn trace_roots(&self, collector: &mut Collector) {
    self.roots.trace(collector);
  }

  fn trace_one(&mut self, index: u32, collector: &mut Collector) {
    self.elements.get(index).trace(collector);
  }

  fn capacity(&self) -> usize {
    self.elements.capacity()
  }

  fn sweep(&mut self, mark_bits: &MarkBits) {
    let capacity = self.elements.capacity();
    let capacity = u32::try_from(capacity).unwrap();
    for index in 0..capacity {
      if !mark_bits.get(index) {
        self.elements.dealloc(index);
      }
    }
  }
}

/// The garbage collector for a heap.
///
/// GC-managed objects should report all of their references to other GC-managed
/// objects (aka "edges") to the collector in their [`Trace`] implementations.
///
/// See the docs for [`Trace`] for more information.
// This type is only exposed to users so they can report edges, but internally
// this does a bit more than that:
//
// * It maintains the mark stack work lists that contain all the GC objects
//   we've seen but have not yet finished processing.
//
// * It maintains the mark bits for all GC objects in the heap, which keep track
//   of which GC objects we have and have not seen while tracing the live set.
pub struct Collector {
  heap_id: u32,
  mark_stacks: FastHashMap<TypeId, Vec<u32>>,
  mark_bits: FastHashMap<TypeId, MarkBits>,
}

impl Collector {
  fn new(heap_id: u32) -> Self {
    Self {
      heap_id,
      mark_stacks: FastHashMap::default(),
      mark_bits: FastHashMap::default(),
    }
  }

  /// Report a reference to another GC-managed object (aka an "edge" in the
  /// heap graph).
  ///
  /// See the docs for [`Trace`] for more information.
  ///
  /// Panics when given cross-heap edges. See the "Cross-`Heap` GC References"
  /// section of [`Heap`]'s documentation for details on cross-heap edges.
  pub fn edge<T>(&mut self, to: Gc<T>)
  where
    T: Trace,
  {
    assert_eq!(to.heap_id, self.heap_id);

    let ty = TypeId::of::<T>();
    let mark_bits = self.mark_bits.get_mut(&ty).unwrap();

    if mark_bits.set(to.index) {
      return;
    }
    let mark_stack = self.mark_stacks.entry(ty).or_default();

    mark_stack.push(to.index);
  }

  fn next_non_empty_mark_stack(&self) -> Option<TypeId> {
    self
      .mark_stacks
      .iter()
      .find_map(|(ty, stack)| if stack.is_empty() { None } else { Some(*ty) })
  }

  fn pop_mark_stack(&mut self, type_id: TypeId) -> Option<u32> {
    self.mark_stacks.get_mut(&type_id).unwrap().pop()
  }
}

/// A collection of GC-managed objects.
pub struct Heap {
  // The unique ID for this heap. Used to ensure that `Gc<T>`s are only used
  // with their associated arena. Could use branded lifetimes to avoid these
  // IDs and checks statically, but the API is gross and pushes lifetimes into
  // everything.
  id: u32,

  // A map from `type_id(T)` to `Arena<T>`.
  arenas: FastHashMap<TypeId, Box<dyn ArenaObject>>,
  collector: Collector,
}

impl Default for Heap {
  fn default() -> Self {
    Heap::new()
  }
}

impl<T> std::ops::Index<Root<T>> for Heap
where
  T: Trace,
{
  type Output = T;
  fn index(&self, root: Root<T>) -> &Self::Output {
    &self[root.unrooted()]
  }
}

impl<T> std::ops::IndexMut<Root<T>> for Heap
where
  T: Trace,
{
  fn index_mut(&mut self, root: Root<T>) -> &mut Self::Output {
    &mut self[root.unrooted()]
  }
}

impl<'a, T> std::ops::Index<&'a Root<T>> for Heap
where
  T: Trace,
{
  type Output = T;
  fn index(&self, root: &'a Root<T>) -> &Self::Output {
    &self[root.unrooted()]
  }
}

impl<'a, T> std::ops::IndexMut<&'a Root<T>> for Heap
where
  T: Trace,
{
  fn index_mut(&mut self, root: &'a Root<T>) -> &mut Self::Output {
    &mut self[root.unrooted()]
  }
}

impl<T> std::ops::Index<Gc<T>> for Heap
where
  T: Trace,
{
  type Output = T;
  fn index(&self, index: Gc<T>) -> &Self::Output {
    self.get(index)
  }
}

impl<T> std::ops::IndexMut<Gc<T>> for Heap
where
  T: Trace,
{
  fn index_mut(&mut self, gc: Gc<T>) -> &mut Self::Output {
    self.get_mut(gc)
  }
}

impl Heap {
  /// Construct a new `Heap`.
  #[inline]
  pub fn new() -> Self {
    let id = Self::next_id();
    Self {
      id,
      arenas: FastHashMap::default(),
      collector: Collector::new(id),
    }
  }

  #[inline]
  fn next_id() -> u32 {
    static ID_COUNTER: atomic::AtomicU32 = atomic::AtomicU32::new(0);
    ID_COUNTER.fetch_add(1, atomic::Ordering::AcqRel)
  }

  #[inline]
  fn arena<T>(&self) -> Option<&Arena<T>>
  where
    T: Trace,
  {
    let arena = self.arenas.get(&TypeId::of::<T>())?;
    Some(arena.as_any().downcast_ref().unwrap())
  }

  #[inline]
  fn arena_mut<T>(&mut self) -> Option<&mut Arena<T>>
  where
    T: Trace,
  {
    let arena = self.arenas.get_mut(&TypeId::of::<T>())?;
    Some(arena.as_any_mut().downcast_mut().unwrap())
  }

  #[inline]
  fn ensure_arena<T>(&mut self) -> &mut Arena<T>
  where
    T: Trace,
  {
    self
      .arenas
      .entry(TypeId::of::<T>())
      .or_insert_with(|| Box::new(Arena::<T>::default()) as _)
      .as_any_mut()
      .downcast_mut()
      .unwrap()
  }

  /// Allocate an object in the heap.
  #[inline]
  pub fn alloc<T>(&mut self, value: T) -> Root<T>
  where
    T: Trace,
  {
    let heap_id = self.id;
    let arena = self.ensure_arena::<T>();

    match arena.try_alloc(heap_id, value) {
      Ok(root) => root,
      Err(value) => self.alloc_slow(value),
    }
  }

  #[inline(never)]
  fn alloc_slow<T>(&mut self, value: T) -> Root<T>
  where
    T: Trace,
  {
    // TODO: need to temporarily root `value` across this GC so that its
    // edges don't get collected.
    self.gc();

    let heap_id = self.id;
    let arena = self.ensure_arena::<T>();

    arena.alloc_slow(heap_id, value)
  }

  /// Get a shared reference to an allocated object in the heap.
  #[inline]
  pub fn get<T>(&self, gc: impl Into<Gc<T>>) -> &T
  where
    T: Trace,
  {
    let gc = gc.into();
    assert_eq!(self.id, gc.heap_id);
    let arena = self.arena::<T>().unwrap();
    arena.elements.get(gc.index)
  }

  /// Get a shared reference to an allocated object in the heap.
  #[inline]
  pub fn get_mut<T>(&mut self, gc: impl Into<Gc<T>>) -> &mut T
  where
    T: Trace,
  {
    let gc = gc.into();
    assert_eq!(self.id, gc.heap_id);
    let arena = self.arena_mut::<T>().unwrap();
    arena.elements.get_mut(gc.index)
  }

  /// Root a reference to a GC object.
  #[inline]
  pub fn root<T>(&self, gc: Gc<T>) -> Root<T>
  where
    T: Trace,
  {
    assert_eq!(self.id, gc.heap_id);
    let arena = self.arena::<T>().unwrap();
    arena.root(gc)
  }

  /// Collect garbage.
  #[inline(never)]
  pub fn gc(&mut self) {
    debug_assert!(self.collector.mark_stacks.values().all(|s| s.is_empty()));

    // Reset/pre-allocate the mark bits.
    for (ty, arena) in &self.arenas {
      self.collector.mark_bits.entry(*ty).or_default().reset(arena.capacity());
    }

    // Mark all roots.
    for arena in self.arenas.values() {
      arena.trace_roots(&mut self.collector);
    }

    // Mark everything transitively reachable from the roots.
    //
    // NB: We have a two-level fixed-point loop to avoid checking if every
    // mark stack is non-empty on every iteration of the hottest, inner-most
    // loop.
    while let Some(type_id) = self.collector.next_non_empty_mark_stack() {
      while let Some(index) = self.collector.pop_mark_stack(type_id) {
        self
          .arenas
          .get_mut(&type_id)
          .unwrap()
          .trace_one(index, &mut self.collector);
      }
    }

    // Sweep.
    for (ty, arena) in &mut self.arenas {
      let mark_bits = &self.collector.mark_bits[ty];
      arena.sweep(mark_bits);
    }
  }
}

/// A simple bitset implementation for marking.
#[derive(Default)]
struct MarkBits(Vec<u8>);

impl MarkBits {
  /// Get the mark bit for the given index.
  ///
  /// Panics if the index is out of bounds.
  pub fn get(&self, index: u32) -> bool {
    let index = index as usize;

    let byte_index = index / 8;
    let bit_index = index % 8;

    let mask = 1 << bit_index;

    self.0[byte_index] & mask != 0
  }

  /// Sets the mark bit for the given index, and returns the old mark bit
  /// state.
  ///
  /// Panics if the index is out of bounds.
  pub fn set(&mut self, index: u32) -> bool {
    let index = index as usize;

    let byte_index = index / 8;
    let bit_index = index % 8;

    let mask = 1 << bit_index;
    let old_byte = self.0[byte_index];

    self.0[byte_index] = old_byte | mask;

    old_byte & mask != 0
  }

  /// Reset the bits, and ensure there are enough bits for the given capacity.
  pub fn reset(&mut self, capacity: usize) {
    self.0.clear();
    self.0.resize(capacity, 0);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // Define a GC-managed tree of `T` values.
  #[derive(Debug)]
  struct Tree<T: Trace> {
    value: Gc<T>,
    parent: Option<Gc<Tree<T>>>,
    left: Option<Gc<Tree<T>>>,
    right: Option<Gc<Tree<T>>>,
  }

  // Another GC type!
  struct Cat {
    cuteness: u32,
  }

  impl<T: Trace> Trace for Tree<T> {
    fn trace(&self, collector: &mut Collector) {
      collector.edge(self.value);
      if let Some(parent) = self.parent {
        collector.edge(parent);
      }
      if let Some(left) = self.left {
        collector.edge(left);
      }
      if let Some(right) = self.right {
        collector.edge(right);
      }
    }
  }

  impl Trace for Cat {
    fn trace(&self, _collector: &mut Collector) {
      // no-op
    }
  }

  #[test]
  fn test_basic_heap_api() {
    let mut heap = Heap::new();

    let root = heap.alloc(Cat { cuteness: 100 });

    let _tree = heap.alloc(Tree {
      value: root.unrooted(),
      parent: None,
      left: None,
      right: None,
    });

    let cuteness = heap[&root].cuteness;
    assert_eq!(cuteness, 100);

    heap.gc();
  }
}
