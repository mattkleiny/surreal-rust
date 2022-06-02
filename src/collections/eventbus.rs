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
  pub fn register<E: 'static>(&mut self, listener: &'a dyn EventListener<E>) {
    let mut events = self.events.borrow_mut();

    // HACK: downgrade lifetime from 'static to 'a for this event listener.
    let event: &mut Event<'a, E> =
      unsafe { std::mem::transmute(events.get_or_create::<Event<E>>()) };

    event.add_listener(listener);
  }

  /// Unregisters a handler for the given event `E`.
  pub fn unregister<E: 'static>(&mut self, listener: &'a dyn EventListener<E>) {
    let mut events = self.events.borrow_mut();

    // HACK: downgrade lifetime from 'static to 'a for this event listener.
    let event: &mut Event<'a, E> =
      unsafe { std::mem::transmute(events.get_or_create::<Event<E>>()) };

    event.remove_listener(listener);
  }

  /// Unregisters all handlers for the given event `E`.
  pub fn unregister_all<E: 'static>(&mut self) {
    let mut events = self.events.borrow_mut();

    // HACK: downgrade lifetime from 'static to 'a for this event listener.
    let event: &mut Event<'a, E> =
      unsafe { std::mem::transmute(events.get_or_create::<Event<E>>()) };

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

#[cfg(test)]
mod tests {
  use std::sync::atomic::AtomicUsize;

  use super::*;

  #[test]
  fn it_should_register_and_notify_events() {
    let mut bus = EventBus::new();
    let mut counter = AtomicUsize::new(0);

    let listener = |event: &usize| {
      counter.fetch_add(*event, std::sync::atomic::Ordering::Relaxed);
    };

    bus.register(&listener);

    bus.dispatch(&1usize);
    bus.dispatch(&2usize);
    bus.unregister_all::<usize>();
    bus.dispatch(&3usize);
    bus.dispatch(&false);
    bus.dispatch(&"Hello, World!");

    let value = *counter.get_mut();

    assert_eq!(value, 3);
  }
}
