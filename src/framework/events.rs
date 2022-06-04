//! A simple event relay system.

use std::any::Any;

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
pub trait EventListener {
  /// Handles the given `Event`.
  fn on_event(&mut self, event: &dyn Event);
}
