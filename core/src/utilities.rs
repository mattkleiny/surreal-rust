//! General utilities.

pub use fsm::*;
pub use object::*;
pub use singleton::*;
pub use size::*;
pub use timing::*;
pub use variant::*;

mod fsm;
mod object;
mod singleton;
mod size;
mod timing;
mod variant;

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
