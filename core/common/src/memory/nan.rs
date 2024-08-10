//! A NaN boxed value
//!
//! This is a technique for storing a value in a single f64, where the value is
//! stored in the bits of the f64, and NaN representations are used to encode
//! the tag of the value itself.

use std::ops::{Deref, DerefMut};

/// A NaN boxed value.
///
/// The value is stored in the bits of the f64, and NaN representations are used
/// to encode the tag of the value itself. The type must be [`ToNanValue`] in
/// order to be stored.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NaN<V> {
  value: NanValue,
  _marker: std::marker::PhantomData<V>,
}

#[derive(Copy, Clone)]
pub union NanValue {
  as_u64: u64,
  as_f64: f64,
}

const NANISH_MASK: u64 = 0x7ffc000000000000;
const BOOL_MASK: u64 = 0x7ffe000000000002;
const I32_MASK: u64 = 0x7ffc000000000000;
const TRUE_MASK: u64 = BOOL_MASK | 3;
const FALSE_MASK: u64 = BOOL_MASK | 2;
const POINTER_MASK: u64 = 0xfffc000000000000;
const NULL_MASK: u64 = 0x7ffe000000000000;

impl NanValue {
  pub fn from_bool(value: bool) -> Self {
    NanValue::from_u64(if value { TRUE_MASK } else { FALSE_MASK })
  }

  pub fn from_i32(value: i32) -> Self {
    NanValue::from_u64(value as u64 | I32_MASK)
  }

  pub fn from_ptr<T>(value: *const T) -> Self {
    NanValue::from_u64(value as u64 | POINTER_MASK)
  }

  pub fn from_u64(value: u64) -> Self {
    NanValue { as_u64: value }
  }

  pub fn from_f64(value: f64) -> Self {
    NanValue { as_f64: value }
  }

  pub fn is_f64(&self) -> bool {
    unsafe { self.as_u64 & NANISH_MASK != NANISH_MASK }
  }

  pub fn is_bool(&self) -> bool {
    unsafe { self.as_u64 & BOOL_MASK == BOOL_MASK }
  }

  pub fn is_i32(&self) -> bool {
    unsafe { self.as_u64 & NANISH_MASK == I32_MASK }
  }

  pub fn is_ptr(&self) -> bool {
    unsafe { self.as_u64 & NANISH_MASK == POINTER_MASK }
  }

  pub fn is_null(&self) -> bool {
    unsafe { self.as_u64 == NULL_MASK }
  }

  pub fn as_f64(&self) -> f64 {
    unsafe { self.as_f64 }
  }

  pub fn as_bool(&self) -> bool {
    unsafe { self.as_u64 == TRUE_MASK }
  }

  pub fn as_i32(&self) -> i32 {
    unsafe { self.as_u64 as i32 }
  }

  pub fn as_ptr<T>(&self) -> *const T {
    unsafe { (self.as_u64 & 0xFFFFFFFFFFFF) as *const T }
  }
}

impl<V: ToNanValue> NaN<V> {
  /// Creates a new NaN box with the given value.
  #[inline]
  pub fn new(value: V) -> Self {
    todo!()
  }

  /// Returns a reference to the inner value.
  #[inline]
  pub fn as_ref(&self) -> &V {
    todo!()
  }

  /// Returns a mutable reference to the inner value.
  #[inline]
  pub fn as_mut(&mut self) -> &mut V {
    todo!()
  }

  /// Converts the NaN box into the inner value.
  #[inline]
  pub fn into_inner(self) -> V {
    todo!()
  }

  /// Erases the type information and returns the inner value.
  pub fn erase(self) -> NaN<()> {
    todo!()
  }
}

impl NaN<()> {
  /// Determines if the NaN box is a specific type.
  pub fn is<T: ToNanValue>(&self) -> bool {
    todo!()
  }

  /// Attempts to reify the NaN box as a specific type.
  pub fn reify<T: ToNanValue>(self) -> Option<NaN<T>> {
    todo!()
  }
}

impl<T: ToNanValue> Deref for NaN<T> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_ref()
  }
}

impl<T: ToNanValue> DerefMut for NaN<T> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut()
  }
}

/// A trait for types that can be stored in a NaN box.
pub trait ToNanValue: Sized {
  /// Converts the value into a NaN box.
  #[inline]
  fn to_nan_box(self) -> NaN<Self> {
    NaN::new(self)
  }

  /// Converts a NaN box into the inner value.
  #[inline]
  fn from_nan_box(nan: NaN<Self>) -> Self {
    nan.into_inner()
  }

  /// Determines if the given value is of the current type.
  fn matches(value: NanValue) -> bool {
    todo!()
  }
}

macro_rules! impl_nan_tag {
  ($type:ty) => {
    impl ToNanValue for $type {}
  };
}

impl_nan_tag!(bool);
impl_nan_tag!(u8);
impl_nan_tag!(u16);
impl_nan_tag!(u32);
impl_nan_tag!(i8);
impl_nan_tag!(i16);
impl_nan_tag!(i32);
impl_nan_tag!(f32);
impl_nan_tag!(f64);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_nan_value_type_check() {
    assert!(NanValue::from_bool(true).is_bool());
    assert!(NanValue::from_bool(false).is_bool());
    assert!(NanValue::from_i32(42).is_i32());
    assert!(NanValue::from_f64(42.0).is_f64());
  }
}
