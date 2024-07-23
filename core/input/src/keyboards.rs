/// Possible key codes on a keyboard.
#[derive(Debug, Clone)]
pub enum KeyCode {
  ScanCode(u8),
  VirtualKey(VirtualKey),
}

/// Possible virtual keys on a keyboard.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
pub trait KeyboardDevice {
  fn is_key_down(&self, key: VirtualKey) -> bool;
  fn is_key_up(&self, key: VirtualKey) -> bool;
}
