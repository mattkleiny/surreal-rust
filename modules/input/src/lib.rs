//! Input engine for Surreal.

pub use gamepads::*;
pub use joysticks::*;
pub use keyboards::*;
pub use mouse::*;
pub use touch::*;

mod gamepads;
mod joysticks;
mod keyboards;
mod mouse;
mod touch;

/// The input engine.
///
/// This struct is the main interface for the input engine. It provides
/// functionality for adding and removing input devices, and for querying the
/// state of input devices.
#[allow(dead_code)]
#[derive(Default)]
pub struct InputEngine {
  gamepads: Vec<Gamepad>,
  keyboards: Vec<Keyboard>,
  mice: Vec<Mouse>,
  joysticks: Vec<Joystick>,
  touch_screens: Vec<TouchScreen>,
}

impl InputEngine {
  /// Creates a new input engine from the given host.
  ///
  /// This method creates a new input engine from the given host. The host
  /// provides information about the input devices that are available on the
  /// system.
  pub fn from_host(_host: &impl InputHost) -> Self {
    todo!()
  }

  /// Receives a list of input events from the host.
  ///
  /// This method is called by the host to provide the input engine with a list
  /// of input events. The input engine then processes the events and updates
  /// the state of all input devices.
  pub fn process(&mut self, _delta_time: f32, _events: &[InputEvent]) {
    todo!()
  }
}

/// A trait for input devices.
///
/// This trait is implemented by all input devices, such as keyboards, mice,
/// joysticks, and gamepads. It is used to provide a common interface for all
/// input sources and to allow for input devices to be added and removed at
/// runtime.
pub trait InputDevice {
  /// Updates the state of the input device in response to some input event.
  fn on_event(&mut self, event: &InputEvent);
}

/// An input event.
///
/// This enum represents an input event, such as a key press or a mouse button
/// press. It is provided by the underlying platform and is passed to the input
/// engine for processing.
pub enum InputEvent {}

/// A trait for input hosts.
///
/// This trait is implemented by all input hosts, such as the game window and
/// testing harness, and provides a source of input event information out of the
/// application.
pub trait InputHost {
  fn enumerate_devices(&self) -> Vec<InputDeviceInfo>;
}

/// Information about an input device.
pub struct InputDeviceInfo {
  pub name: String,
  pub kind: InputDeviceKind,
}

/// Possible kinds of input devices.
pub enum InputDeviceKind {
  Keyboard,
  Mouse,
  Joystick,
  Gamepad,
  TouchScreen,
}
