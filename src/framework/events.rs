//! A simple event layer for the engine.
//!
//! Events are a way for the platform to communicate back to the engine.

use std::any::Any;

use crate::maths::Vector2;

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
  fn as_any(&self) -> &dyn Any {
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
pub struct PlatformResizeEvent(pub usize, pub usize);
pub struct PlatformFocusEvent(pub bool);
pub struct PlatformCloseEvent();

// input events
pub struct KeyPressEvent(pub crate::input::Key);
pub struct KeyReleaseEvent(pub crate::input::Key);
pub struct MouseMoveEvent(pub Vector2<f32>);
pub struct MouseScrollEvent(pub Vector2<f32>);
pub struct MousePressEvent(pub crate::input::MouseButton);
pub struct MouseReleaseEvent(pub crate::input::MouseButton);

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

    let mut receiver = EventReceiver { tick_count: 0 };

    receiver.on_event(&PlatformTickEvent());
    receiver.on_event(&PlatformTickEvent());
    receiver.on_event(&PlatformRenderEvent());

    assert_eq!(receiver.tick_count, 2);
  }
}
