//! A lightweight task runtime for Rust.

use std::{
  future::Future,
  pin::Pin,
  sync::Mutex,
  task::{Context, Poll},
};

/// A continuation that can be executed.
type Continuation = dyn FnOnce() -> ();

/// A task that can be executed concurrently.
pub struct Task<T> {
  status: TaskStatus<T>,
  future: Option<Box<dyn Future<Output = T>>>,
}

/// The status of a task.
enum TaskStatus<T> {
  NotStarted,
  Running,
  Completed(T),
  Finalized,
}

impl<T> TaskStatus<T> {
  /// Attempts to take the value from the status.
  fn take(&mut self) -> Option<T> {
    match std::mem::replace(self, TaskStatus::Finalized) {
      TaskStatus::NotStarted => None,
      TaskStatus::Running => None,
      TaskStatus::Completed(value) => Some(value),
      TaskStatus::Finalized => None,
    }
  }
}

impl<T> Task<T> {
  /// Spawn a new task from a future.
  pub fn spawn(future: impl Future<Output = T> + 'static) -> Self {
    Self {
      status: TaskStatus::NotStarted,
      future: Some(Box::new(future)),
    }
  }

  /// Creates a new task from a result.
  pub fn from_result(result: T) -> Self {
    Self {
      status: TaskStatus::Completed(result),
      future: None,
    }
  }
}

/// A scheduler for [`Task`]s.
#[derive(Default)]
struct TaskScheduler {
  continuations: Mutex<crate::SwapVec<Box<Continuation>>>,
}

/// The global task scheduler.
static TASK_SCHEDULER: crate::UnsafeSingleton<TaskScheduler> = crate::UnsafeSingleton::default();

impl TaskScheduler {
  /// Returns the current task scheduler.
  pub fn current() -> &'static Self {
    &TASK_SCHEDULER
  }

  /// Schedules a [`Continuation`] to be executed.
  pub fn schedule(&self, continuation: Box<Continuation>) {
    let mut continuations = self.continuations.lock().unwrap();

    continuations.push(continuation);
  }

  /// Processes all [`Continuation`]s in the task scheduler.
  pub fn process(&self) {
    let mut continuations = self.continuations.lock().unwrap();

    for continuation in continuations.swap().drain(..) {
      continuation();
    }
  }
}

impl<T: Unpin> Future for Task<T> {
  type Output = T;

  fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
    match self.status {
      TaskStatus::NotStarted => {
        let waker = context.waker().clone();
        let future = self.future.take().unwrap();

        self.status = TaskStatus::Running;

        // TODO: fix this up
        let continuation = move || {
          let mut pinned = Box::into_pin(future);

          match pinned.as_mut().poll(context) {
            Poll::Ready(value) => {
              let mut task = self.as_mut();
              task.status = TaskStatus::Completed(value);
              waker.wake_by_ref();
            }
            Poll::Pending => {
              let mut task = self.as_mut();

              task.status = TaskStatus::Running;
            }
          }
        };

        // TASK_SCHEDULER.schedule(Box::new(continuation));

        Poll::Pending
      }
      TaskStatus::Running => Poll::Pending,
      TaskStatus::Completed(_) => Poll::Ready(self.status.take().unwrap()),
      TaskStatus::Finalized => panic!("Task has already been finalized"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::BlockableFuture;

  #[test]
  fn test_basic_task_continuations() {
    let scheduler = TaskScheduler::current();

    scheduler.schedule(Box::new(|| {
      println!("Hello, world!");
    }));

    scheduler.process();
  }
}
