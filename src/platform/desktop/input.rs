use std::collections::HashSet;

use egui::RawInput;
use winit::event::{ElementState, MouseButton};

use crate::input::*;
use crate::maths::{range, Vector2};
use crate::ui::RawInputProvider;

/// The server for input management.
pub struct DesktopInput {
  keyboard: DesktopKeyboardDevice,
  mouse: DesktopMouseDevice,
  raw_input: RawInput,
}

impl DesktopInput {
  /// Creates a new input server.
  pub fn new() -> Self {
    Self {
      keyboard: DesktopKeyboardDevice::new(),
      mouse: DesktopMouseDevice::new(),
      raw_input: Default::default(),
    }
  }

  /// Ticks the input system, apply state changes.
  pub fn tick(&mut self) {
    self.keyboard.tick();
    self.mouse.tick();
  }

  /// Notifies of a keyboard event.
  pub fn on_keyboard_event(&mut self, event: winit::event::KeyboardInput) {
    self.keyboard.on_keyboard_event(event);
  }

  /// Notifies of a mouse movement event.
  pub fn on_mouse_move(&mut self, position: Vector2<f32>, window_size: Vector2<f32>) {
    self.mouse.on_mouse_moved(position, window_size);
  }

  /// Notifies of a mouse button event.
  pub fn on_mouse_event(&mut self, button: MouseButton, state: ElementState) {
    self.mouse.on_mouse_event(button, state);
  }
}

impl InputBackend for DesktopInput {
  fn keyboard_device(&self) -> Option<&dyn KeyboardDevice> {
    Some(&self.keyboard)
  }

  fn mouse_device(&self) -> Option<&dyn MouseDevice> {
    Some(&self.mouse)
  }
}

impl RawInputProvider for DesktopInput {
  fn get_raw_input(&self) -> &RawInput {
    &self.raw_input
  }
}

/// A keyboard device for desktop platforms.
struct DesktopKeyboardDevice {
  previous_keys: HashSet<Key>,
  current_keys: HashSet<Key>,
}

impl DesktopKeyboardDevice {
  /// Creates a new keyboard device.
  pub fn new() -> Self {
    Self {
      previous_keys: HashSet::new(),
      current_keys: HashSet::new(),
    }
  }

  /// Ticks the keyboard device.
  pub fn tick(&mut self) {
    self.previous_keys.clear();
  }

  /// Handles keyboard events.
  pub fn on_keyboard_event(&mut self, event: winit::event::KeyboardInput) {
    if let Some(virtual_key_code) = event.virtual_keycode {
      if event.state == ElementState::Pressed {
        self.current_keys.insert(virtual_key_code);
        self.previous_keys.insert(virtual_key_code);
      } else {
        self.current_keys.remove(&virtual_key_code);
      }
    }
  }
}

impl KeyboardDevice for DesktopKeyboardDevice {
  fn is_key_up(&self, key: Key) -> bool {
    !self.current_keys.contains(&key)
  }

  fn is_key_down(&self, key: Key) -> bool {
    self.current_keys.contains(&key)
  }

  fn is_key_pressed(&self, key: Key) -> bool {
    self.previous_keys.contains(&key)
  }
}

/// A mouse device for desktop platforms.
struct DesktopMouseDevice {
  position: Vector2<f32>,
  normalised_position: Vector2<f32>,
  previous_buttons: HashSet<MouseButton>,
  current_buttons: HashSet<MouseButton>,
}

impl DesktopMouseDevice {
  /// Creates a new mouse device.
  pub fn new() -> Self {
    Self {
      position: Vector2::new(0., 0.),
      normalised_position: Vector2::new(0., 0.),
      previous_buttons: HashSet::new(),
      current_buttons: HashSet::new(),
    }
  }

  /// Ticks the mouse.
  pub fn tick(&mut self) {
    self.previous_buttons.clear();
  }

  /// Handles mouse movement.
  pub fn on_mouse_moved(&mut self, new_position: Vector2<f32>, window_size: Vector2<f32>) {
    self.position = new_position;
    self.normalised_position = new_position / window_size;
  }

  /// Handles mouse click events.
  pub fn on_mouse_event(&mut self, button: MouseButton, state: ElementState) {
    match state {
      ElementState::Pressed => {
        self.current_buttons.insert(button);
        self.previous_buttons.insert(button);
      }
      ElementState::Released => {
        self.current_buttons.remove(&button);
      }
    };
  }
}

impl MouseDevice for DesktopMouseDevice {
  fn position(&self) -> Vector2<f32> {
    self.position
  }

  fn normalised_position(&self) -> Vector2<f32> {
    self.normalised_position.clamp(range(0.01, 0.99))
  }

  fn is_button_up(&self, button: MouseButton) -> bool {
    !self.current_buttons.contains(&button)
  }

  fn is_button_down(&self, button: MouseButton) -> bool {
    self.current_buttons.contains(&button)
  }

  fn is_button_pressed(&self, button: MouseButton) -> bool {
    self.previous_buttons.contains(&button)
  }
}
