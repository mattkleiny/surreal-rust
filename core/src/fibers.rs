//! Async building blocks for game programming.

use std::{
  future::Future,
  pin::Pin,
  sync::{Arc, Condvar, Mutex},
  task::{Context, Poll, Wake, Waker},
};

/// Yields the current thread back to the async scheduler.
#[inline]
pub fn yield_fiber() -> impl Future<Output = ()> {
  /// A [`Future`] that yields the current thread.
  struct Yield(bool);

  impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
      // The futures executor is implemented as a FIFO queue, so all this future
      // does is re-schedule the future back to the end of the queue, giving room
      // for other futures to progress.
      if !self.0 {
        self.0 = true;
        context.waker().wake_by_ref();
        Poll::Pending
      } else {
        Poll::Ready(())
      }
    }
  }

  Yield(false)
}

/// Blocks the current thread until the given future is resolved.
pub fn block_on<F: Future>(mut future: F) -> F::Output {
  // TODO: simplify this, build a proper fiber scheduler

  enum SignalState {
    Empty,
    Waiting,
    Notified,
  }

  struct Signal {
    state: Mutex<SignalState>,
    cond: Condvar,
  }

  impl Signal {
    fn new() -> Self {
      Self {
        state: Mutex::new(SignalState::Empty),
        cond: Condvar::new(),
      }
    }

    fn wait(&self) {
      let mut state = self.state.lock().unwrap();
      match *state {
        SignalState::Notified => {
          // Notify() was called before we got here, consume it here without waiting and
          // return immediately.
          *state = SignalState::Empty;
          return;
        }
        // This should not be possible because our signal is created within a function and never
        // handed out to any other threads. If this is the case, we have a serious problem
        // so we panic immediately to avoid anything more problematic happening.
        SignalState::Waiting => {
          unreachable!("Multiple threads waiting on the same signal: Open a bug report!");
        }
        SignalState::Empty => {
          // Nothing has happened yet, and we're the only thread waiting (as should be the
          // case!). Set the state accordingly and begin polling the condvar in
          // a loop until it's no longer telling us to wait. The loop prevents
          // incorrect spurious wakeups.
          *state = SignalState::Waiting;
          while let SignalState::Waiting = *state {
            state = self.cond.wait(state).unwrap();
          }
        }
      }
    }

    fn notify(&self) {
      let mut state = self.state.lock().unwrap();
      match *state {
        // The signal was already notified, no need to do anything because the thread will be waking
        // up anyway
        SignalState::Notified => {}
        // The signal wasnt notified but a thread isnt waiting on it, so we can avoid doing
        // unnecessary work by skipping the condvar and leaving behind a message telling the
        // thread that a notification has already occurred should it come along in the
        // future.
        SignalState::Empty => *state = SignalState::Notified,
        // The signal wasnt notified and there's a waiting thread. Reset the signal so it can be
        // wait()'ed on again and wake up the thread. Because there should only be a single
        // thread waiting, `notify_all` would also be valid.
        SignalState::Waiting => {
          *state = SignalState::Empty;
          self.cond.notify_one();
        }
      }
    }
  }

  impl Wake for Signal {
    fn wake(self: Arc<Self>) {
      self.notify();
    }
  }

  // Pin the future so that it can be polled.
  // SAFETY: We shadow `fut` so that it cannot be used again. The future is now
  // pinned to the stack and will not be moved until the end of this scope. This
  // is, incidentally, exactly what the `pin_mut!` macro from `pin_utils` does.
  let mut future = unsafe { std::pin::Pin::new_unchecked(&mut future) };

  // Signal used to wake up the thread for polling as the future moves to
  // completion. We need to use an `Arc` because, although the lifetime of `fut`
  // is limited to this function, the underlying IO abstraction might keep the
  // signal alive for far longer. `Arc` is a thread-safe way to allow
  // this to happen. TODO: Investigate ways to reuse this `Arc<Signal>`... perhaps
  // via a `static`?
  let signal = Arc::new(Signal::new());

  // Create a context that will be passed to the future.
  let waker = Waker::from(Arc::clone(&signal));
  let mut context = Context::from_waker(&waker);

  // Poll the future to completion
  loop {
    match future.as_mut().poll(&mut context) {
      Poll::Pending => signal.wait(),
      Poll::Ready(item) => break item,
    }
  }
}
