//! A lightweight task runtime for Rust.

use std::{
  future::Future,
  pin::Pin,
  sync::Mutex,
  task::{Context, Poll},
};

crate::impl_arena_index!(TaskId, "An internal identifier for a task.");

/// A continuation that can be executed.
type Continuation = dyn FnOnce() -> ();

/// A task that can be executed concurrently.
pub struct Task<T> {
  status: TaskStatus<T>,
}

/// The status of a task.
enum TaskStatus<T> {
  Pending(TaskId),
  Completed(T),
  Finalized,
}

impl<T> TaskStatus<T> {
  /// Attempts to take the value from the status.
  fn take(&mut self) -> Option<T> {
    match std::mem::replace(self, TaskStatus::Finalized) {
      TaskStatus::Pending(_) => None,
      TaskStatus::Completed(value) => Some(value),
      TaskStatus::Finalized => None,
    }
  }
}

impl<T> Task<T> {
  /// Creates a new task from a result.
  pub fn from_result(result: T) -> Self {
    Self {
      status: TaskStatus::Completed(result),
    }
  }

  /// Spawn a new task from a future.
  pub fn spawn(_future: impl Future<Output = T> + 'static) -> Self {
    todo!()
  }
}

/// A scheduler for [`Task`]s.
#[derive(Default)]
pub struct TaskScheduler {
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

  fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
    match self.status {
      TaskStatus::Pending(_) => Poll::Pending,
      TaskStatus::Completed(_) => Poll::Ready(self.status.take().unwrap()),
      TaskStatus::Finalized => panic!("Task has already been finalized"),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_task_continuations() {
    let scheduler = TaskScheduler::current();

    scheduler.schedule(Box::new(|| {
      println!("Hello, world!");
    }));

    scheduler.process();
  }
}
