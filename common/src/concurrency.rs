//! Building blocks for working with asynchronous code.

use std::future::Future;

/// A future that can be blocked.
pub trait BlockableFuture: Future {
  fn block(self) -> Self::Output;
}

impl<F: Future> BlockableFuture for F {
  #[inline(always)]
  fn block(self) -> Self::Output {
    block(|| self)
  }
}

/// Blocks the current thread until the given future completes.
pub fn block<F: Future>(_body: impl FnOnce() -> F) -> F::Output {
  todo!()
}
