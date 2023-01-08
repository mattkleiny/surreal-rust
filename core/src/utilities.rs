//! General utilities.

pub use object::*;
pub use singleton::*;
pub use size::*;
pub use timing::*;
pub use variant::*;
pub use version::*;

mod object;
mod singleton;
mod size;
mod timing;
mod variant;
mod version;

/// Abstracts over resource IDs.
pub trait RID {
  /// Converts the resource to it's base `u64`.
  fn to_u64(&self) -> u64;
}

/// Creates An opaque ID for a resource in one of the [`Server`] implementations.
///
/// This is an opaque handle that can be used to identify a resource in the server.
#[macro_export]
macro_rules! impl_rid_type {
  ($name:ident) => {
    #[repr(transparent)]
    #[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct $name(pub u64);

    impl $name {
      #[inline(always)]
      pub const fn new(id: u64) -> Self {
        Self(id)
      }
    }

    impl $crate::utilities::RID for $name {
      #[inline(always)]
      fn to_u64(&self) -> u64 {
        self.0
      }
    }

    macro_rules! impl_rid_conversion {
      ($type:ty) => {
        impl From<$type> for $name {
          #[inline(always)]
          fn from(value: $type) -> Self {
            Self(value as u64)
          }
        }

        impl Into<$type> for $name {
          #[inline(always)]
          fn into(self) -> $type {
            self.0 as $type
          }
        }
      };
    }

    impl_rid_conversion!(i8);
    impl_rid_conversion!(i16);
    impl_rid_conversion!(i32);
    impl_rid_conversion!(i64);
    impl_rid_conversion!(i128);

    impl_rid_conversion!(u8);
    impl_rid_conversion!(u16);
    impl_rid_conversion!(u32);
    impl_rid_conversion!(u64);
    impl_rid_conversion!(u128);
    impl_rid_conversion!(usize);
  };
}

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
