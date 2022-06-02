//! C# style event observation system for Rust with simple ownership rules.

/// An event that can register listeners and notify them at a later time.
pub struct Event<'a, E> {
  listeners: Vec<&'a dyn EventListener<E>>,
}

/// A listener receives events from an `Event` source.
pub trait EventListener<E> {
  fn on_event(&self, event: &E);
}

/// Compare listener references by pointer equality.
impl<E> PartialEq for &dyn EventListener<E> {
  fn eq(&self, other: &Self) -> bool {
    std::ptr::eq(self as *const _, other as *const _)
  }
}

/// Allow delegates to be used as listeners.
impl<E, F: Fn(&E)> EventListener<E> for F {
  fn on_event(&self, event: &E) {
    self(event);
  }
}

impl<'a, E> Default for Event<'a, E> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a, E> Event<'a, E> {
  /// Creates a new event.
  pub fn new() -> Self {
    Self {
      listeners: Vec::new(),
    }
  }

  /// Adds a new listener to the event.
  pub fn add_listener(&mut self, listener: &'a dyn EventListener<E>) {
    self.listeners.push(listener);
  }

  /// Removes a listener from the event.
  pub fn remove_listener(&mut self, listener: &'a dyn EventListener<E>) {
    self.listeners.retain(|it| *it != listener);
  }

  /// Removes all listeners from the event.
  pub fn clear_listeners(&mut self) {
    self.listeners.clear();
  }

  /// Notifies event listeners.
  pub fn notify(&self, event: &E) {
    for listener in &self.listeners {
      listener.on_event(event);
    }
  }
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
    event.remove_listener(listener1);

    event.notify(&42);
  }
}
