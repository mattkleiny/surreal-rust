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
pub trait RID: Eq + std::hash::Hash {
  /// Converts the given `u64` to a resource ID.
  fn from_u64(id: u64) -> Self;

  /// Converts the resource ID to it's base `u64`.
  fn to_u64(&self) -> u64;
}

/// Creates An opaque ID for a resource in a implementation.
///
/// This is an opaque handle that can be used to identify a resource in the server.
#[macro_export]
macro_rules! impl_rid_type {
  ($name:ident) => {
    #[repr(transparent)]
    #[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct $name(pub u64);

    impl $name {
      #[inline]
      pub const fn new(id: u64) -> Self {
        Self(id)
      }
    }

    impl $crate::utilities::RID for $name {
      #[inline]
      fn from_u64(id: u64) -> Self {
        Self(id)
      }

      #[inline]
      fn to_u64(&self) -> u64 {
        self.0
      }
    }
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
