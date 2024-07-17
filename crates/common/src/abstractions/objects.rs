use std::{
  any::Any,
  fmt::{Debug, Formatter},
  hash::{Hash, Hasher},
  ops::{Deref, DerefMut},
  sync::Mutex,
};

use macros::Trace;

use crate::{impl_arena_index, Arena, Singleton};

impl_arena_index!(ObjectIndex, "An index of an object in the garbage collector");

/// Represents a garbage-collected object with a potentially reified type.
pub struct Object<T: ?Sized = dyn Any> {
  entry: GC<ObjectEntry>,
  _phantom: std::marker::PhantomData<T>,
}

/// The internal entry for an object in the garbage collector.
///
/// This is used to track the object's reference count, and to allow for
/// garbage collection of the object when it is no longer referenced.
#[derive(Trace)]
struct ObjectEntry {
  is_marked: bool,
  reference: *const [u8],
}

impl<T: 'static> Object<T> {
  /// Creates a new object with the given value.
  pub fn new(_value: T) -> Self {
    todo!()
  }

  /// Casts the object to a different type.
  #[inline(always)]
  pub fn cast<U: ?Sized>(self) -> Object<U> {
    Object {
      entry: self.entry,
      _phantom: std::marker::PhantomData,
    }
  }
}

impl<T: ?Sized> Clone for Object<T> {
  fn clone(&self) -> Self {
    Self {
      entry: self.entry.clone(),
      _phantom: std::marker::PhantomData,
    }
  }
}

impl<T: ?Sized> PartialEq for Object<T> {
  fn eq(&self, other: &Self) -> bool {
    self.entry.as_ptr() == other.entry.as_ptr()
  }
}

impl<T: ?Sized> Hash for Object<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.entry.as_ptr().hash(state);
  }
}

impl<T: ?Sized> Deref for Object<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    todo!()
  }
}

impl<T: ?Sized> DerefMut for Object<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    todo!()
  }
}

impl<T: ?Sized> Debug for Object<T> {
  /// Formats the object as a pointer to the object.
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "Object({:p})", self.entry.as_ptr())
  }
}

/// A smart pointer to a garbage-collected object.
///
/// The object is automatically deallocated when the last reference is dropped.
pub struct GC<T: Trace> {
  index: ObjectIndex,
  _phantom: std::marker::PhantomData<T>,
}

impl<T: Trace + 'static> GC<T> {
  /// Creates a new garbage-collected object.
  pub fn new(value: T) -> Self {
    Self {
      index: GarbageCollector::instance().allocate(value),
      _phantom: std::marker::PhantomData,
    }
  }
}

impl<T: Trace> GC<T> {
  /// Returns a raw pointer to the inner value.
  pub fn as_ptr(&self) -> *const T {
    self.as_ref() as *const T
  }

  /// Returns a mutable raw pointer to the inner value.
  pub fn as_mut_ptr(&mut self) -> *mut T {
    self.as_mut() as *mut T
  }
}

impl<T: Trace> Clone for GC<T> {
  fn clone(&self) -> Self {
    GarbageCollector::instance().increment_reference(self.index);

    Self {
      index: self.index,
      _phantom: std::marker::PhantomData,
    }
  }
}

impl<T: Trace + Debug> Debug for GC<T> {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    self.as_ref().fmt(formatter)
  }
}

impl<T: Trace> AsRef<T> for GC<T> {
  fn as_ref(&self) -> &T {
    GarbageCollector::instance().get(self.index).unwrap()
  }
}

impl<T: Trace> AsMut<T> for GC<T> {
  fn as_mut(&mut self) -> &mut T {
    GarbageCollector::instance().get_mut(self.index).unwrap()
  }
}

impl<T: Trace> Deref for GC<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.as_ref()
  }
}

impl<T: Trace> DerefMut for GC<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut()
  }
}

impl<T: Trace> Drop for GC<T> {
  fn drop(&mut self) {
    GarbageCollector::instance().decrement_reference(self.index);
  }
}

/// A trait for objects that can be traced by the garbage collector.
///
/// This trait is used to mark objects as reachable by the garbage collector.
/// When an object is traced, the garbage collector will mark the object as
/// reachable, and will recursively trace any other objects that the object
/// references.
///
/// # Safety
///
/// This trait is unsafe because for incorrectly defined manual implementations,
/// it's possible to leak memory. Rely on the derive macro for this trait where
/// possible.
pub unsafe trait Trace {
  /// Traces the object, marking all reachable objects.
  fn trace(&self, context: &mut TraceContext);
}

/// Context for tracing objects using [`Trace`].
pub struct TraceContext {
  // TODO: implement me
}

/// Implements the [`Trace`] trait for a type that does not contain any cycles.
macro_rules! impl_empty_trace {
  ($type:ty) => {
    unsafe impl Trace for $type {
      #[inline(always)]
      fn trace(&self, _: &mut TraceContext) {
        // no-op
      }
    }
  };
}

impl_empty_trace!(());
impl_empty_trace!(bool);
impl_empty_trace!(char);
impl_empty_trace!(u8);
impl_empty_trace!(u16);
impl_empty_trace!(u32);
impl_empty_trace!(u64);
impl_empty_trace!(usize);
impl_empty_trace!(i8);
impl_empty_trace!(i16);
impl_empty_trace!(i32);
impl_empty_trace!(i64);
impl_empty_trace!(isize);
impl_empty_trace!(f32);
impl_empty_trace!(f64);
impl_empty_trace!(*const [u8]);
impl_empty_trace!(*const dyn Any);

/// A simple mark-sweep garbage collector.
///
/// This garbage collector uses a simple mark-sweep algorithm to free memory.
/// It is not optimized for performance, but is simple to implement.
#[derive(Singleton, Default)]
struct GarbageCollector {
  entries: Mutex<Arena<ObjectIndex, GarbageCollectorEntry>>,
}

/// An entry in the [`GarbageCollector`].
#[derive(Debug)]
struct GarbageCollectorEntry {
  object: *const dyn Trace,
  reference_count: usize,
}

unsafe impl Send for GarbageCollector {}
unsafe impl Sync for GarbageCollector {}

impl GarbageCollector {
  /// Allocates a new object in the garbage collector.
  pub fn allocate<T: Trace + 'static>(&self, value: T) -> ObjectIndex {
    let mut entries = self.entries.lock().unwrap();
    let allocation = Box::leak(Box::new(value));

    entries.insert(GarbageCollectorEntry {
      object: allocation as *const dyn Trace,
      reference_count: 1,
    })
  }

  /// Increments the reference count of an object.
  pub fn increment_reference(&self, index: ObjectIndex) {
    let mut entries = self.entries.lock().unwrap();
    let entry = entries.get_mut(index).unwrap();

    entry.reference_count += 1;
  }

  /// Decrements the reference count of an object.
  ///
  /// If the reference count reaches zero, the object is removed from the
  /// garbage collector.
  pub fn decrement_reference(&self, index: ObjectIndex) {
    let mut entries = self.entries.lock().unwrap();
    let entry = entries.get_mut(index).unwrap();

    entry.reference_count -= 1;

    if entry.reference_count == 0 {
      entries.remove(index);
    }
  }

  /// Dereferences an object index to a reference.
  pub fn get<T: Trace>(&self, index: ObjectIndex) -> Option<&T> {
    let entries = self.entries.lock().unwrap();
    let entry = entries.get(index)?;

    Some(unsafe { &*(entry.object as *const T) })
  }

  /// Dereferences an object index to a mutable reference.
  pub fn get_mut<T: Trace>(&self, index: ObjectIndex) -> Option<&mut T> {
    let entries = self.entries.lock().unwrap();
    let entry = entries.get(index)?;

    Some(unsafe { &mut *(entry.object as *mut T) })
  }
}

#[cfg(test)]
mod tests {
  use macros::Trace;

  use super::*;

  #[derive(Trace, Debug)]
  pub struct TestStruct {
    value_1: u32,
    value_2: f32,
  }

  #[test]
  fn test_allocation_and_free() {
    let instance = GC::new(TestStruct {
      value_1: 100,
      value_2: std::f32::consts::PI,
    });

    assert_eq!(instance.value_1, 100u32);

    drop(instance);

    let entries = GarbageCollector::instance().entries.lock().unwrap();

    assert_eq!(entries.len(), 0);
  }

  #[test]
  fn test_allocation_and_clone() {
    let instance1 = GC::new(TestStruct {
      value_1: 100,
      value_2: std::f32::consts::PI,
    });
    let mut instance2 = instance1.clone();

    assert_eq!(instance1.value_1, 100u32);
    instance2.value_1 = 200u32;
    assert_eq!(instance1.value_1, 200u32);

    drop(instance1);

    let entries = GarbageCollector::instance().entries.lock().unwrap();

    assert_eq!(entries.len(), 1);
  }
}
