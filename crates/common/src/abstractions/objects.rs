use std::{
  fmt::Debug,
  ops::{Deref, DerefMut},
  sync::Mutex,
};

use crate::{impl_arena_index, Arena, Singleton};

impl_arena_index!(ObjectIndex, "An index of an object in the garbage collector");

/// A reference to a garbage-collected object.
pub struct GC<T: Trace + 'static> {
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

  /// Returns a reference to the inner value.
  pub fn as_ref(&self) -> &T {
    GarbageCollector::instance().get(self.index).unwrap()
  }

  /// Returns a mutable reference to the inner value.
  pub fn as_mut(&mut self) -> &mut T {
    GarbageCollector::instance().get_mut(self.index).unwrap()
  }

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

impl<T: Trace + Debug + 'static> Debug for GC<T> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.as_ref().fmt(formatter)
  }
}

impl<T: Trace + 'static> Deref for GC<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.as_ref()
  }
}

impl<T: Trace + 'static> DerefMut for GC<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut()
  }
}

impl<T: Trace + 'static> Drop for GC<T> {
  fn drop(&mut self) {
    GarbageCollector::instance().decrement_reference(self.index);
  }
}

/// A garbage-collected object.
///
/// This trait is used to mark objects that are managed by a garbage collector.
///
/// The garbage collector will automatically free memory when the object is no
/// longer reachable.
///
/// In order to reference an object outside the garbage collector, use a GC<T>
/// smart pointer.
pub trait Object {}

/// A trait for objects that can be traced by the garbage collector.
pub unsafe trait Trace {
  /// Traces the object, marking all reachable objects.
  fn trace(&self, gc: &mut GarbageCollector);
}

/// Blanket implementation of the [`Object`] trait all traceable types.
impl<T: Trace> Object for T {}

/// Implements the [`Trace`] trait for a type that does not contain any cycles.
macro_rules! impl_empty_trace {
  ($type:ty) => {
    unsafe impl Trace for $type {
      #[inline(always)]
      fn trace(&self, _gc: &mut GarbageCollector) {
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
impl_empty_trace!(i8);
impl_empty_trace!(i16);
impl_empty_trace!(i32);
impl_empty_trace!(i64);
impl_empty_trace!(f32);
impl_empty_trace!(f64);

/// A simple mark-sweep garbage collector.
///
/// This garbage collector uses a simple mark-sweep algorithm to free memory.
/// It is not optimized for performance, but is simple to implement.
#[derive(Singleton, Default)]
pub struct GarbageCollector {
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
  fn allocate<T: Trace + 'static>(&self, value: T) -> ObjectIndex {
    let mut entries = self.entries.lock().unwrap();
    let allocation = Box::leak(Box::new(value));

    entries.insert(GarbageCollectorEntry {
      object: allocation as *const dyn Trace,
      reference_count: 1,
    })
  }

  /// Increments the reference count of an object.
  fn increment_reference(&self, index: ObjectIndex) {
    let mut entries = self.entries.lock().unwrap();
    let entry = entries.get_mut(index).unwrap();

    entry.reference_count += 1;
  }

  /// Decrements the reference count of an object.
  ///
  /// If the reference count reaches zero, the object is removed from the
  /// garbage collector.
  fn decrement_reference(&self, index: ObjectIndex) {
    let mut entries = self.entries.lock().unwrap();
    let entry = entries.get_mut(index).unwrap();

    entry.reference_count -= 1;

    if entry.reference_count == 0 {
      entries.remove(index);
    }
  }

  /// Dereferences an object index to a reference.
  fn get<T: Trace + 'static>(&self, index: ObjectIndex) -> Option<&T> {
    let entries = self.entries.lock().unwrap();
    let entry = entries.get(index)?;

    Some(unsafe { &*(entry.object as *const T) })
  }

  /// Dereferences an object index to a mutable reference.
  fn get_mut<T: Trace + 'static>(&self, index: ObjectIndex) -> Option<&mut T> {
    let entries = self.entries.lock().unwrap();
    let entry = entries.get(index)?;

    Some(unsafe { &mut *(entry.object as *mut T) })
  }
}

#[cfg(test)]
mod tests {
  use macros::Object;

  use super::*;

  #[derive(Object, Debug)]
  pub struct TestObject {
    value: u32,
  }

  #[test]
  fn test_basic_object_allocation_and_free() {
    let instance = GC::new(TestObject { value: 100u32 });

    assert_eq!(instance.value, 100u32);

    drop(instance);

    let entries = GarbageCollector::instance().entries.lock().unwrap();

    assert_eq!(entries.len(), 0);
  }

  #[test]
  fn test_object_allocation_and_clone() {
    let instance1 = GC::new(TestObject { value: 100u32 });
    let mut instance2 = instance1.clone();

    assert_eq!(instance1.value, 100u32);

    instance2.value = 200u32;

    assert_eq!(instance1.value, 200u32);

    drop(instance1);

    let entries = GarbageCollector::instance().entries.lock().unwrap();

    assert_eq!(entries.len(), 1);
  }
}
