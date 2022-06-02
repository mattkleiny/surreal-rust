//! A dynamic event bus implementation that permits multiple dispatch
//! per unique event type.

use std::{
  any::{Any, TypeId},
  collections::HashMap,
};

/// A bus for forwarding events between different handlers of the same type.
///
/// This is used to allow handlers to communicate with each other without
/// having to know about each other.
pub struct EventBus {
  handlers: HashMap<TypeId, Box<dyn Fn(&dyn Any)>>,
}

impl EventBus {
  /// Creates a new event bus.
  pub fn new() -> Self {
    EventBus {
      handlers: HashMap::new(),
    }
  }

  /// Determines if a handler exists for the given event type.
  pub fn is_registered<E: 'static>(&self) -> bool {
    self.handlers.contains_key(&TypeId::of::<E>())
  }

  /// Registers an event handler in the bus.
  pub fn register<E: 'static, F: Fn(&E) + 'static>(&mut self, callback: F) {
    self.handlers.insert(
      TypeId::of::<E>(),
      Box::new(move |event| callback(event.downcast_ref().unwrap())),
    );
  }

  /// Unregisters an event handler from the bus.
  pub fn unregister<E: 'static>(&mut self) {
    self.handlers.remove(&TypeId::of::<E>());
  }

  /// Publishes an event to the bus.
  pub fn publish<E: 'static>(&self, event: &E) {
    if let Some(handler) = self.handlers.get(&event.type_id()) {
      handler(event);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::prelude::PlatformTickEvent;

  use super::*;

  #[test]
  fn it_should_work() {
    let mut bus = EventBus::new();

    bus.register(|_: &PlatformTickEvent| {
      println!("It worked!");
    });

    bus.publish(&PlatformTickEvent());
  }
}
