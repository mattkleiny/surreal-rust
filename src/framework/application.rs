//! A simple application layer for the engine.
//!
//! Applications are entry points for more complex engine usages, and
//! form the core of the engine and foundation for event plumbing.

use crate::{
  collections::AnyMultiMap,
  maths::Vector2,
  platform::{Platform, PlatformHost},
};

/// Entry point for a Surreal-based application.
pub struct Application<P: Platform> {
  event_bus: EventBus,
  platform_host: P::Host,
}

/// Represents a listener that can receive events from an application.
pub trait ApplicationListener<P: Platform> {
  fn run(&mut self);
}

impl<P: Platform> Application<P> {
  /// Creates a new application on the given platform.
  pub fn new(platform: P) -> Self {
    Self {
      event_bus: EventBus::new(),
      platform_host: platform.create_host(),
    }
  }

  /// Runs the application with the given main body.
  pub fn run(&mut self, mut _listener: impl ApplicationListener<P>) {
    // TODO: handle listener invocations

    self.platform_host.pump(&self.event_bus);
  }
}

/// A bus for forwarding events between different handlers of the same type.
///
/// This is used to allow handlers to communicate with each other without
/// having to know about each other.
pub struct EventBus {
  handlers: AnyMultiMap,
}

impl EventBus {
  /// Creates a new event bus.
  pub fn new() -> Self {
    Self {
      handlers: AnyMultiMap::new(),
    }
  }

  /// Registers a new handler in the event bus.
  pub fn add_handler<'a, E>(&mut self, _handler: impl EventHandler<E> + 'a) {
    todo!();
  }

  /// Removes a handler from the event bus.
  pub fn remove_handler<'a, E>(&mut self, _handler: impl EventHandler<E> + 'a) {
    todo!();
  }

  /// Publishes an event on the event bus, notifying all handlers immediately.
  pub fn publish<E>(&self, _event: E) {
    todo!()
  }
}

/// Represents a handler for a particular kind of event, `E`.
///
/// This handler can either be a structured type, or a function.
pub trait EventHandler<E> {
  fn handle_event(&mut self, _event: &E);
}

/// Allow arbitrary function handlers to be registered with the event bus.
impl<E, F: FnMut(&E) -> ()> EventHandler<E> for F {
  fn handle_event(&mut self, event: &E) {
    self(event);
  }
}

// platform events
pub struct PlatformTickEvent();
pub struct PlatformRenderEvent();
pub struct PlatformResizedEvent(pub usize, pub usize);
pub struct PlatformClosedEvent();

// input events
pub struct KeyPressdEvent(pub crate::input::Key);
pub struct KeyReleasedEvent(pub crate::input::Key);
pub struct MouseMovedEvent(pub Vector2<usize>);
pub struct MouseScrolledEvent(pub Vector2<usize>);
pub struct MousePressedEvent(pub crate::input::MouseButton);
pub struct MouseReleasedEvent(pub crate::input::MouseButton);

// TODO: platform specific events

#[cfg(test)]
mod tests {
  use crate::{
    graphics::{Color, GraphicsBackend},
    platform::HeadlessPlatform,
  };

  use super::*;

  #[test]
  fn application_should_work() {
    let application = Application::new(HeadlessPlatform);
    let platform = &application.platform_host;

    application.event_bus.publish(PlatformTickEvent());
    application.event_bus.publish(PlatformRenderEvent());

    platform.graphics.clear_color_buffer(Color::WHITE);

    todo!();
  }

  #[test]
  fn event_bus_should_notify_listeners() {
    let mut bus = EventBus::new();
    let mut invoked = false;

    bus.add_handler(|_event: &PlatformTickEvent| {
      invoked = true;
    });

    bus.publish(PlatformTickEvent());
    bus.publish(PlatformRenderEvent());

    assert!(invoked);
  }
}
