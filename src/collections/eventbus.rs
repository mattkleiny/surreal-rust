//! A dynamic event bus implementation that permits multiple dispatch
//! per unique event type.

use std::{any::Any, cell::RefCell, marker::PhantomData};

use super::{AnyMap, Event, EventListener};

/// This event is raised in lieue of another event going unhandled in the system.
pub struct DeadLetterEvent(Box<dyn Any>);

/// A bus for forwarding events between different handlers of the same type.
///
/// This is used to allow handlers to communicate with each other without
/// having to know about each other.
pub struct EventBus<'a> {
  events: RefCell<AnyMap>,
  _lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a> EventBus<'a> {
  /// Creates a new event bus.
  pub fn new() -> Self {
    EventBus {
      events: RefCell::new(AnyMap::new()),
      _lifetime: PhantomData,
    }
  }

  /// Registers a handler for the given event `E`.
  pub fn register<E: 'static>(&mut self, listener: impl EventListener<E> + 'static) {
    let mut events = self.events.borrow_mut();
    let event = events.get_or_create::<Event<E>>();

    event.add_listener(listener);
  }

  /// Unregisters all handlers for the given event `E`.
  pub fn unregister_all<E: 'static>(&mut self) {
    let mut events = self.events.borrow_mut();
    let event = events.get_or_create::<Event<E>>();

    event.clear_listeners();
  }

  /// Publishes the given event to the event bus.
  ///
  /// If the event is unhandled a `DeadLetterEvent` will be raised.
  pub fn publish<E: Any>(&self, data: E) {
    if !self.dispatch(&data) {
      self.dispatch(&DeadLetterEvent(Box::new(data)));
    }
  }

  /// Dispatches the given event internall to all handlers that will accept it.
  fn dispatch<E: 'static>(&self, data: &E) -> bool {
    let events = self.events.borrow();

    if let Some(event) = events.get::<Event<E>>() {
      event.notify(data);
      true
    } else {
      false
    }
  }
}
