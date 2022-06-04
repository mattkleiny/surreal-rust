//! A simple application layer for the engine.
//!
//! Applications are entry points for more complex engine usages, and
//! form the core of the engine and foundation for event plumbing.

use std::{any::Any, marker::PhantomData};

use crate::{maths::Vector2, platform::Platform, platform::PlatformHost};

/// Entry point for a Surreal-based application.
pub struct Application<P: Platform> {
  host: P::Host,
}

impl<P: Platform + 'static> Application<P> {
  /// Creates a new application on the given platform.
  pub fn new(platform: P) -> Self {
    Self {
      host: platform.create_host(),
    }
  }

  /// Runs the application with the given main body.
  pub fn run<L: ApplicationListener<P> + 'static>(self, listener: L) {
    // TODO: clean this up?
    /// An event relay will forward events from the platform out to the listener.
    struct EventRelay<L: ApplicationListener<P>, P: Platform>(L, PhantomData<P>);

    impl<L: ApplicationListener<P>, P: Platform> EventListener for EventRelay<L, P> {
      fn on_event(&mut self, event: &dyn Event) {
        self.0.on_event(event);
      }
    }

    self.host.pump(EventRelay(listener, PhantomData));
  }
}

/// Represents a listener that can receive events from an application.
pub trait ApplicationListener<P: Platform> {
  /// Invoked when the application should tick.
  fn tick(&self);

  /// Invoked when the application should render.
  fn render(&self);

  /// Invoked when an event is received from the platform or application.
  fn on_event(&self, event: &dyn Any);
}

/// Represents an event type in the application.
///
/// Event types allow for dynamic dispatch based on a flexible set of
/// event sources; this allows for a wide range of event handling without
/// specifying the entire superset in this interface.
pub trait Event: Any {}

/// Any type that is representable as `Any` is a valid `Event`.
impl<T> Event for T where T: Any {}

impl dyn Event {
  /// Converts this event to `Any`, allowing downcasting to an appropriate type.
  pub fn as_any(&self) -> &dyn Any {
    self as &dyn Any
  }

  /// Returns the type of the event.
  pub fn is<T: 'static>(&self) -> bool {
    self.as_any().is::<T>()
  }

  /// Converts this event to the given type.
  pub fn to<T: 'static>(&self) -> Option<&T> {
    self.as_any().downcast_ref::<T>()
  }
}

/// Represents a listener for generic `Event` types.
///
/// Event types can be checked with the `is` method, and cast with the `to` method.
///
/// An event listener is typically passed down to the platform to allow it to call
/// back up to specific application instances and notify them.
pub trait EventListener {
  /// Handles the given `Event`.
  fn on_event(&mut self, event: &dyn Event);
}

// platform events
pub struct PlatformTickEvent();
pub struct PlatformRenderEvent();
pub struct PlatformResizedEvent(pub usize, pub usize);
pub struct PlatformClosedEvent();

// input events
pub struct KeyPressedEvent(pub crate::input::Key);
pub struct KeyReleasedEvent(pub crate::input::Key);
pub struct MouseMovedEvent(pub Vector2<usize>);
pub struct MouseScrolledEvent(pub Vector2<usize>);
pub struct MousePressedEvent(pub crate::input::MouseButton);
pub struct MouseReleasedEvent(pub crate::input::MouseButton);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_send_and_receive_events() {
    /// An example event listener implementation.
    struct EventReceiver {
      tick_count: u32,
    }

    impl EventListener for EventReceiver {
      fn on_event(&mut self, event: &dyn Event) {
        if event.is::<PlatformTickEvent>() {
          self.tick_count += 1;
        }
      }
    }

    let mut receiver = EventReceiver {
      tick_count: 0,
    };

    receiver.on_event(&PlatformTickEvent());
    receiver.on_event(&PlatformTickEvent());
    receiver.on_event(&PlatformRenderEvent());

    assert_eq!(receiver.tick_count, 2);
  }
}
