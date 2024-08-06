//! A very lightweight fiber runtime for Rust.
//!
//! Fibers are a form of lightweight cooperative multitasking that can be used
//! to write asynchronous code in a synchronous style. Fibers are similar to
//! threads, but they are scheduled cooperatively rather than preemptively.
//!
//! To spawn a fiber, you can use the [`FiberTask::spawn`] function with a
//! future that returns a value. The fiber will run concurrently with other
//! fibers and will need to be manually stepped by calling
//! [`FiberTask::resume`] until it is completed.

use std::{future::Future, pin::Pin, task::Poll};

use super::TryPoll;

/// Starts a new fiber task.
#[inline(always)]
pub fn fiber<F: Future + 'static>(future: F) -> FiberTask<F::Output> {
  FiberTask::spawn(future)
}

/// Yields execution until the next frame.
pub fn next_frame() -> impl Future<Output = ()> {
  /// A no-op instruction for a fiber that yield's it's value.
  struct FiberYield {
    done: bool,
  }

  impl Future for FiberYield {
    type Output = ();

    #[inline(always)]
    fn poll(mut self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
      if self.done {
        Poll::Ready(())
      } else {
        self.done = true;
        Poll::Pending
      }
    }
  }

  FiberYield { done: false }
}

/// A fiber task that can be executed cooperatively.
pub struct FiberTask<T> {
  status: FiberStatus<T>,
  future: Pin<Box<dyn Future<Output = T>>>,
}

/// The internal status of a [`FiberTask`].
enum FiberStatus<T> {
  Pending,
  Completed(T),
  Finalized,
}

impl<T> FiberStatus<T> {
  /// Attempts to take the value from the status.
  fn take(&mut self) -> Option<T> {
    match std::mem::replace(self, FiberStatus::Finalized) {
      FiberStatus::Pending => None,
      FiberStatus::Completed(value) => Some(value),
      FiberStatus::Finalized => None,
    }
  }
}

impl<T> FiberTask<T> {
  /// Spawns a new fiber task from a future.
  pub fn spawn<F: Future<Output = T> + 'static>(future: F) -> Self {
    Self {
      status: FiberStatus::Pending,
      future: Box::pin(future),
    }
  }

  /// Resumes control to the fiber. If completed, returns the value.
  pub fn resume(&mut self) -> Option<T> {
    match self.status {
      FiberStatus::Pending => match self.future.as_mut().try_poll() {
        Poll::Ready(value) => {
          self.status = FiberStatus::Completed(value);
          self.status.take()
        }
        Poll::Pending => None,
      },
      FiberStatus::Completed(_) => self.status.take(),
      FiberStatus::Finalized => None,
    }
  }

  /// Resumes the fiber to completion.
  pub fn complete(&mut self) -> T {
    if matches!(self.status, FiberStatus::Finalized) {
      panic!("Fiber has already been finalized");
    }

    loop {
      if let Some(value) = self.resume() {
        return value;
      }
    }
  }
}

/// Allows a [`FiberTask`] to be polled.
impl<T: Unpin> Future for FiberTask<T> {
  type Output = T;

  fn poll(mut self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
    if let Some(value) = self.resume() {
      Poll::Ready(value)
    } else {
      Poll::Pending
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::BlockableFuture;

  #[test]
  fn test_basic_fiber_usage() {
    let mut fiber = fiber(async { 42 });
    let result = fiber.complete();

    assert_eq!(result, 42);
  }

  #[test]
  #[should_panic]
  fn test_fiber_panic_after_completion() {
    let mut fiber = fiber(async { 42 });
    let _ = fiber.complete();

    fiber.complete();
  }

  #[test]
  fn test_fiber_future() {
    let fiber = fiber(async { 42 });
    let result = fiber.block();

    assert_eq!(result, 42);
  }

  #[test]
  fn test_fiber_yield() {
    let mut fiber = fiber(async {
      next_frame().await;
      next_frame().await;
      next_frame().await;

      42
    });

    assert_eq!(fiber.resume(), None);
    assert_eq!(fiber.resume(), None);
    assert_eq!(fiber.resume(), None);
    assert_eq!(fiber.resume(), Some(42));
  }
}
