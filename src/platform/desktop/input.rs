use std::collections::HashSet;

use winit::event::{ElementState, MouseButton};

use crate::input::{InputServer, Key, KeyboardDevice, MouseDevice};
use crate::maths::Vector2;

/// The server for input management.
pub struct DesktopInputServer {
  keyboards: Vec<DesktopKeyboardDevice>,
  mice: Vec<DesktopMouseDevice>,
}

impl DesktopInputServer {
  /// Creates a new input server.
  pub fn new() -> Self {
    Self {
      keyboards: Vec::from([DesktopKeyboardDevice::new()]),
      mice: Vec::from([DesktopMouseDevice::new()]),
    }
  }

  pub fn on_keyboard_event(&mut self, event: winit::event::KeyboardInput) {
    // TODO: make this support multiple devices?
    let keyboard = self.keyboards.first_mut().unwrap();

    keyboard.on_keyboard_event(event);
  }

  pub fn on_mouse_move(&mut self, position: Vector2<f32>, window_size: Vector2<f32>) {
    // TODO: make this support multiple devices?
    let mouse = self.mice.first_mut().unwrap();

    mouse.on_mouse_moved(position, window_size);
  }

  pub fn on_mouse_event(&mut self, button: MouseButton, state: ElementState) {
    // TODO: make this support multiple devices?
    let mouse = self.mice.first_mut().unwrap();

    mouse.on_mouse_event(button, state);
  }
}

unsafe impl InputServer for DesktopInputServer {
  fn keyboard_devices(&self) -> &[&dyn KeyboardDevice] {
    todo!()
  }

  fn mouse_devices(&self) -> &[&dyn MouseDevice] {
    todo!()
  }

  fn primary_keyboard_device(&self) -> Option<&dyn KeyboardDevice> {
    self
      .keyboards
      .first()
      .map(|device| device as &dyn KeyboardDevice)
  }

  fn primary_mouse_device(&self) -> Option<&dyn MouseDevice> {
    self.mice.first().map(|device| device as &dyn MouseDevice)
  }
}

/// A keyboard device for desktop platforms.
struct DesktopKeyboardDevice {
  pressed_keys: HashSet<Key>,
}

impl DesktopKeyboardDevice {
  pub fn new() -> Self {
    Self {
      pressed_keys: HashSet::new(),
    }
  }

  pub fn on_keyboard_event(&mut self, event: winit::event::KeyboardInput) {
    if let Some(virtual_key_code) = event.virtual_keycode {
      if event.state == ElementState::Pressed {
        self.pressed_keys.insert(virtual_key_code);
      } else {
        self.pressed_keys.remove(&virtual_key_code);
      }
    }
  }
}

impl KeyboardDevice for DesktopKeyboardDevice {
  fn is_key_up(&self, key: Key) -> bool {
    !self.pressed_keys.contains(&key)
  }

  fn is_key_down(&self, key: Key) -> bool {
    self.pressed_keys.contains(&key)
  }

  fn is_key_pressed(&self, key: Key) -> bool {
    // TODO: implement me properly
    self.pressed_keys.contains(&key)
  }
}

/// A mouse device for desktop platforms.
struct DesktopMouseDevice {
  position: Vector2<f32>,
  normalised_position: Vector2<f32>,
  pressed_buttons: HashSet<MouseButton>,
}

impl DesktopMouseDevice {
  pub fn new() -> Self {
    Self {
      position: Vector2::new(0., 0.),
      normalised_position: Vector2::new(0., 0.),
      pressed_buttons: HashSet::new(),
    }
  }

  pub fn on_mouse_moved(&mut self, new_position: Vector2<f32>, window_size: Vector2<f32>) {
    self.position = new_position;
    self.normalised_position = new_position / window_size;
  }

  pub fn on_mouse_event(&mut self, button: MouseButton, state: ElementState) {
    match state {
      ElementState::Pressed => self.pressed_buttons.insert(button),
      ElementState::Released => self.pressed_buttons.remove(&button),
    };
  }
}

impl MouseDevice for DesktopMouseDevice {
  fn position(&self) -> Vector2<f32> {
    self.position
  }

  fn normalised_position(&self) -> Vector2<f32> {
    self.normalised_position
  }

  fn is_button_up(&self, button: MouseButton) -> bool {
    !self.pressed_buttons.contains(&button)
  }

  fn is_button_down(&self, button: MouseButton) -> bool {
    self.pressed_buttons.contains(&button)
  }

  fn is_button_pressed(&self, button: MouseButton) -> bool {
    // TODO: implement me properly
    self.pressed_buttons.contains(&button)
  }
}
