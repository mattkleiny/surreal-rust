use std::{future::Future, pin::Pin, task::Poll};

/// Blocks the current thread until the given future completes.
pub fn block<F: Future>(body: impl FnOnce() -> F) -> F::Output {
  body().block()
}

/// Allows a [`Future`] to be blocked on.
pub trait BlockableFuture {
  type Output;

  /// Blocks the current thread until the future completes.
  fn block(self) -> Self::Output;
}

impl<F: Future> BlockableFuture for F {
  type Output = F::Output;

  fn block(self) -> Self::Output {
    let mut future = Box::pin(self);
    loop {
      match future.as_mut().try_poll() {
        Poll::Ready(value) => return value,
        Poll::Pending => std::thread::yield_now(),
      }
    }
  }
}

/// Allows polling for a future without scheduling a wakeup.
pub trait TryPoll {
  type Output;

  /// Attempts to resolve the future to a final value.
  fn try_poll(self: Pin<&mut Self>) -> Poll<Self::Output>;
}

/// Allows a [`Future`] to attempted to be polled.
impl<F: ?Sized + Future> TryPoll for F {
  type Output = F::Output;

  fn try_poll(mut self: Pin<&mut Self>) -> Poll<Self::Output> {
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(&waker);

    self.as_mut().poll(&mut context)
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
