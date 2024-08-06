//! A lightweight task runtime for Rust.

use std::{
  future::Future,
  pin::Pin,
  sync::Mutex,
  task::{Context, Poll},
};

/// A continuation that can be executed.
type Continuation = dyn FnOnce() -> ();

/// A scheduler for tasks.
#[derive(Default)]
pub struct TaskScheduler {
  continuations: Mutex<Vec<Box<Continuation>>>,
}

/// A task that can be executed concurrently.
pub struct Task<T> {
  status: TaskStatus<T>,
}

/// The status of a task.
enum TaskStatus<T> {
  Pending,
  Completed(T),
}

impl<T> Task<T> {
  /// Creates a new task from a result.
  pub fn from_result(result: T) -> Self {
    Self {
      status: TaskStatus::Completed(result),
    }
  }
}

impl TaskScheduler {
  /// Returns the current task scheduler.
  pub fn current() -> &'static Self {
    todo!()
  }

  /// Schedules a [`Continuation`] to be executed.
  pub fn schedule(continuation: Box<Continuation>) {
    let scheduler = Self::current();
    let mut continuations = scheduler.continuations.lock().unwrap();

    continuations.push(continuation);
  }

  /// Processes all [`Continuation`]s in the task scheduler.
  pub fn process() {
    let scheduler = Self::current();
    let mut continuations = scheduler.continuations.lock().unwrap();

    for continuation in continuations.drain(..) {
      continuation();
    }
  }
}

impl<T: Unpin> Future for Task<T> {
  type Output = T;

  fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
    todo!()
  }
}
