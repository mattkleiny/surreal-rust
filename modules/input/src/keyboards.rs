use crate::{InputDevice, InputEvent};

/// Possible key codes on a keyboard.
#[derive(Debug, Clone)]
pub enum KeyCode {
  ScanCode(u8),
  VirtualKey(VirtualKey),
}

/// Possible virtual keys on a keyboard.
#[derive(Debug, Clone)]
pub enum VirtualKey {
  Escape,
  F(u8),
  Digits(u8),
  Letters(char),
  ArrowKey(ArrowKey),
  Space,
  Backspace,
  Tab,
  Enter,
}

/// A possible arrow key on a keyboard.
#[derive(Debug, Clone)]
pub enum ArrowKey {
  Up,
  Down,
  Left,
  Right,
}

/// A keyboard event.
#[derive(Debug, Clone)]
pub enum KeyboardEvent {
  KeyDown(KeyCode),
  KeyUp(KeyCode),
}

/// A keyboard input device.
pub struct KeyboardDevice {
  queued_events: Vec<KeyboardEvent>,
}

impl InputDevice for KeyboardDevice {
  fn update(&mut self, _delta_time: f32) {
    todo!()
  }

  fn drain_events(&mut self) -> Vec<InputEvent> {
    let mut events = Vec::with_capacity(self.queued_events.len());

    for event in self.queued_events.drain(..) {
      events.push(InputEvent::KeyboardEvent(event));
    }

    events
  }
}
