use std::future::Future;

/// Blocks the current thread until the given future completes.
pub fn block<F: Future>(_body: impl FnOnce() -> F) -> F::Output {
  todo!()
}

/// Allows a [`Future`] to be blocked on.
pub trait BlockableFuture: Future {
  /// Blocks the current thread until the future completes.
  fn block(self) -> Self::Output;
}

impl<F: Future> BlockableFuture for F {
  #[inline(always)]
  fn block(self) -> Self::Output {
    block(|| self)
  }
}
