//! General utilities.

pub use object::*;
pub use services::*;
pub use singleton::*;
pub use size::*;
pub use timing::*;
pub use variant::*;
pub use version::*;

mod object;
mod services;
mod singleton;
mod size;
mod timing;
mod variant;
mod version;

/// Re-export `bytemuck` for consumers of Surreal.
pub mod bytemuck {
  pub use bytemuck::*;
}

/// Creates an unsafe mutable alias to the given value.
///
/// This breaks many assumptions in the Rust type system, so use with great
/// caution.
#[inline(always)]
pub(crate) fn unsafe_mutable_alias<'a, T>(value: &T) -> &'a mut T {
  unsafe {
    let pointer = value as *const T;
    let pointer = pointer as *mut T;

    &mut *pointer
  }
}
