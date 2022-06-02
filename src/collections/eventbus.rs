//! A dynamic event bus implementation that permits multiple dispatch
//! per unique event type.

use std::any::{Any, TypeId};

use super::MultiMap;

/// This event is raised in lieue of another event going unhandled in the system.
pub struct DeadLetterEvent(Box<dyn Any>);

/// A bus for forwarding events between different handlers of the same type.
///
/// This is used to allow handlers to communicate with each other without
/// having to know about each other.
pub struct EventBus {
  handlers: MultiMap<TypeId, Box<dyn Any>>,
}

/// An event handler for a particular event type.
type EventHandler<E> = Box<dyn Fn(&mut E) + 'static>;

/// A set of handlers for a particular event type.
type EventHandlers<E> = Vec<EventHandler<E>>;

impl EventBus {
  pub fn new() -> EventBus {
    EventBus {
      handlers: MultiMap::new(),
    }
  }

  /// Registers a handler for the given event type.
  pub fn register<E: Any>(&mut self, _handler: impl Fn(&E) + 'static) {
    todo!();
  }

  /// Unregisters a handler for the given event type.
  pub fn unregister<E: Any>(&mut self, _handler: impl Fn(&E) + 'static) {
    todo!();
  }

  /// Unregisters all handlers for the given event type.
  pub fn unregister_all<E: Any>(&mut self) {
    todo!();
  }

  /// Publishes the given event to the event bus.
  ///
  /// If the event is unhandled a `DeadLetterEvent` will be raised.
  pub fn publish<E: Any>(&self, event: E) {
    if !self.dispatch(&event) {
      self.dispatch(&DeadLetterEvent(Box::new(event)));
    }
  }

  /// Dispatches the given event internall to all handlers that will accept it.
  fn dispatch<E: Any>(&self, _event: &E) -> bool {
    todo!();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn eventbus_works() {
    fn handler_1(event: &String) {
      println!("my_handler {:?}", event);
    }

    fn handler_2(event: &String) {
      println!("my_handler2 {:?}", event);
    }

    fn dead_event_handler(event: &DeadLetterEvent) {
      match event.0.downcast_ref::<u64>() {
        Some(value) => println!("dead_event_handler: {}", value),
        None => println!("dead_event_handler: this wasn't for me"),
      }
    }

    let mut bus = EventBus::new();

    bus.register(handler_1);
    bus.register(handler_1);
    bus.register(handler_2);
    bus.register(|event: &i32| println!("my_closure_handler {:?}", event));
    bus.register(dead_event_handler);

    bus.publish("Hello World".to_string());
    bus.publish("Hello World");
    bus.publish(123 as i32);
    bus.publish(123123123 as u64);
  }
}
