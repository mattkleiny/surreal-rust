//! General utilities.

pub use object::*;
pub use servers::*;
pub use singleton::*;
pub use size::*;
pub use timing::*;
pub use variant::*;

mod object;
mod servers;
mod singleton;
mod size;
mod timing;
mod variant;

/// An opaque ID for a resource in one of the [`Server`] implementations.
///
/// This is an opaque handle that can be used to identify a resource in the server.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RID(pub u64);

impl RID {
  #[inline(always)]
  pub const fn new(id: u64) -> Self {
    Self(id)
  }
}

macro_rules! impl_rid_type {
  ($type:ty) => {
    impl From<$type> for RID {
      #[inline(always)]
      fn from(value: $type) -> Self {
        Self(value as u64)
      }
    }

    impl Into<$type> for RID {
      #[inline(always)]
      fn into(self) -> $type {
        self.0 as $type
      }
    }
  };
}

impl_rid_type!(i8);
impl_rid_type!(i16);
impl_rid_type!(i32);
impl_rid_type!(i64);
impl_rid_type!(i128);

impl_rid_type!(u8);
impl_rid_type!(u16);
impl_rid_type!(u32);
impl_rid_type!(u64);
impl_rid_type!(u128);
impl_rid_type!(usize);

/// Creates an unsafe mutable alias to the given value.
///
/// This breaks many assumptions in the Rust type system, so use with great caution.
#[inline(always)]
pub(crate) fn unsafe_mutable_alias<'a, T>(value: &T) -> &'a mut T {
  unsafe {
    let pointer = value as *const T;
    let pointer = pointer as *mut T;

    &mut *pointer
  }
}
