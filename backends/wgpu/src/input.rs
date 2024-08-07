//! Input handling for winit.

use input::{KeyboardDevice, KeyboardEvent, MouseButton, MouseDevice, MouseEvent, VirtualKey};
use winit::{event::ElementState, keyboard::PhysicalKey};

/// A keyboard device for winit.
#[derive(Default)]
pub struct WinitKeyboardDevice {
  events: Vec<KeyboardEvent>,
}

impl KeyboardDevice for WinitKeyboardDevice {
  fn events(&self) -> &[KeyboardEvent] {
    &self.events
  }
}

impl WinitKeyboardDevice {
  pub fn handle_input(&mut self, event: &winit::event::KeyEvent) {
    if let PhysicalKey::Code(code) = event.physical_key {
      if let Some(virtual_key) = convert_scancode(code) {
        match event.state {
          ElementState::Pressed => self.events.push(KeyboardEvent::KeyDown(virtual_key)),
          ElementState::Released => self.events.push(KeyboardEvent::KeyUp(virtual_key)),
        }
      }
    }
  }

  pub fn clear_events(&mut self) {
    self.events.clear();
  }
}

/// A mouse device for Winit.
#[derive(Default)]
pub struct WinitMouseDevice {
  events: Vec<MouseEvent>,
}

impl MouseDevice for WinitMouseDevice {
  fn events(&self) -> &[MouseEvent] {
    &self.events
  }
}

impl WinitMouseDevice {
  pub fn handle_input(&mut self, state: ElementState, button: winit::event::MouseButton) {
    let button = match button {
      winit::event::MouseButton::Left => MouseButton::Left,
      winit::event::MouseButton::Right => MouseButton::Right,
      winit::event::MouseButton::Middle => MouseButton::Middle,
      _ => return,
    };

    self.events.push(match state {
      ElementState::Pressed => MouseEvent::MouseDown(button),
      ElementState::Released => MouseEvent::MouseUp(button),
    });
  }

  pub fn clear_events(&mut self) {
    self.events.clear();
  }
}

/// Converts a winit scancode to a virtual key.
fn convert_scancode(scan_code: winit::keyboard::KeyCode) -> Option<VirtualKey> {
  use input::VirtualKey::*;

  match scan_code {
    winit::keyboard::KeyCode::Escape => Some(Escape),
    _ => None,
  }
}
