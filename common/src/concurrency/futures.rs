use std::future::Future;

/// Blocks the current thread until the given future completes.
pub fn block<F: Future>(body: impl FnOnce() -> F) -> F::Output {
  body().block()
}

/// Allows a [`Future`] to be blocked on.
pub trait BlockableFuture: Future {
  /// Blocks the current thread until the future completes.
  fn block(self) -> Self::Output;
}

impl<F: Future> BlockableFuture for F {
  fn block(self) -> Self::Output {
    let mut boxed = Box::pin(self);
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);

    loop {
      if let std::task::Poll::Ready(value) = boxed.as_mut().poll(&mut context) {
        return value;
      }

      std::thread::yield_now();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_block_should_yield_future_result() {
    let task1 = async { 1 };
    let task2 = async { 2 };

    assert_eq!(task1.block() + task2.block(), 3);
  }
}
