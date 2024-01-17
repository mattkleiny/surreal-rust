//! Input engine for Surreal.

pub use keyboards::*;
pub use mouse::*;

mod keyboards;
mod mouse;

/// The input engine.
///
/// This struct is the main interface for the input engine. It provides
/// functionality for adding and removing input devices, and for querying the
/// state of input devices.
#[derive(Default)]
pub struct InputEngine {
  _keyboards: Vec<KeyboardDevice>,
  _mice: Vec<MouseDevice>,
}

impl InputEngine {
  /// Creates a new input engine from the given host.
  ///
  /// This method creates a new input engine from the given host. The host
  /// provides information about the input devices that are available on the
  /// system.
  pub fn new(_host: &dyn InputHost) -> Self {
    todo!()
  }
}

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

/// Possible kinds of input devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputDeviceKind {
  Keyboard,
  Mouse,
}

/// Information about an input device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDeviceInfo {
  pub name: String,
  pub kind: InputDeviceKind,
}

/// An abstraction over a host capable of running input.
pub trait InputHost {
  /// Returns a list of all input devices that are available on the system.
  fn enumerate_devices(&self) -> &[InputDeviceInfo];

  /// Returns the input device with the given name.
  fn get_device(&self, name: &str) -> Option<Box<dyn InputDevice>>;
}

/// A trait for input devices.
///
/// This trait is implemented by all input devices, such as keyboards, mice,
/// joysticks, and gamepads. It is used to provide a common interface for all
/// input sources and to allow for input devices to be added and removed at
/// runtime.
pub trait InputDevice {
  /// Updates the state of the device.
  fn update(&mut self, delta_time: f32);

  /// Drains all pending input events from the device.
  fn drain_events(&mut self) -> Vec<InputEvent>;
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
