use std::collections::BTreeMap;
use std::ops::{AddAssign, SubAssign};

/// A simple event that can statically subscribed to.
pub struct Event<E> {
  listeners: BTreeMap<usize, fn(&E)>,
}

impl<E> Event<E> {
  pub fn new() -> Self {
    Self { listeners: BTreeMap::new() }
  }

  /// Publishes the given event to all handlers.
  pub fn publish(&self, event: &E) {
    for (_, listener) in self.listeners.iter() {
      listener(&event);
    }
  }
}

impl<E> AddAssign<fn(&E)> for Event<E> {
  /// Registers a listener on the event.
  fn add_assign(&mut self, listener: fn(&E)) {
    let address = unsafe { *(listener as *const usize) };
    self.listeners.insert(address as usize, listener);
  }
}

impl<E> SubAssign<fn(&E)> for Event<E> {
  /// Removes a listener from the event.
  fn sub_assign(&mut self, listener: fn(&E)) {
    let address = unsafe { *(listener as *const usize) };
    self.listeners.remove(&(address as usize));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_subscribe_and_publish_events() {
    let mut event = Event::new();

    event += |id| print!("Hello, {:?}", id);

    event.publish(&42)
  }
}