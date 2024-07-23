//! Input engine for Surreal.

pub use keyboards::*;
pub use mouse::*;

mod keyboards;
mod mouse;

/// An input event.
///
/// This enum represents an input event, such as a key press or a mouse button
/// press. It is provided by the underlying platform and is passed to the input
/// engine for processing.
#[derive(Debug, Clone)]
pub enum InputEvent {
  KeyboardEvent(KeyboardEvent),
  MouseEvent(MouseEvent),
}

/// A listener for input events.
pub trait InputListener {
  /// Receives an input event.
  fn on_event(&mut self, event: &InputEvent);
}

/// Allows a closure to be used as an input listener.
impl<F: FnMut(&InputEvent)> InputListener for F {
  fn on_event(&mut self, event: &InputEvent) {
    self(event);
  }
}
