//! C# style event observation system for Rust with simple ownership rules.

/// A listener receives events from an `Event` source.
pub trait EventListener<E> {
  fn on_event(&mut self, event: &E);
}

/// Allow delegates to be used as listeners.
impl<E, F: FnMut(&E)> EventListener<E> for F {
  fn on_event(&mut self, event: &E) {
    self(event);
  }
}
/// An event that can register listeners and notify them at a later time.
pub struct Event<E> {
  listeners: Vec<Box<dyn EventListener<E>>>,
}

impl<E> Default for Event<E> {
  fn default() -> Self {
    Self::new()
  }
}

impl<E> Event<E> {
  /// Creates a new event.
  pub fn new() -> Self {
    Self {
      listeners: Vec::new(),
    }
  }

  /// Adds a new listener to the event.
  pub fn add_listener(&mut self, listener: impl EventListener<E> + 'static) {
    self.listeners.push(Box::new(listener));
  }

  /// Removes all listeners from the event.
  pub fn clear_listeners(&mut self) {
    self.listeners.clear();
  }

  /// Notifies event listeners.
  pub fn notify(&self, event: &E) {
    for listener in &self.listeners {
      let listener = unsafe { very_bad_function(listener) };

      listener.on_event(event);
    }
  }
}

unsafe fn very_bad_function<T>(reference: &T) -> &mut T {
  let const_ptr = reference as *const T;
  let mut_ptr = const_ptr as *mut T;
  &mut *mut_ptr
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_work() {
    let mut event = Event::new();

    let listener1 = &|value: &usize| {
      println!("Listener 1 is {:?}", value);
    };

    let listener2 = &|value: &usize| {
      println!("Listener 2 is {:?}", value);
    };

    event.add_listener(listener1);
    event.add_listener(listener2);

    event.notify(&42);
  }
}
