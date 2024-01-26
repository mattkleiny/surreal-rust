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

/// Multiplexes events to multiple listeners.
pub struct InputMultiplexer {
  listeners: Vec<Box<dyn InputListener>>,
}

impl InputListener for InputMultiplexer {
  fn on_event(&mut self, event: &InputEvent) {
    for listener in &mut self.listeners {
      listener.on_event(event);
    }
  }
}
