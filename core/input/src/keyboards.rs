use common::lua::{FromLua, IntoLua, Lua};

/// A keyboard event.
#[derive(Debug, Clone)]
pub enum KeyboardEvent {
  KeyDown(VirtualKey),
  KeyUp(VirtualKey),
}

/// Possible key codes on a keyboard.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum VirtualKey {
  Escape,
  F0,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  Digit0,
  Digit1,
  Digit2,
  Digit3,
  Digit4,
  Digit5,
  Digit6,
  Digit7,
  Digit8,
  Digit9,
  ArrowUp,
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  Space,
  Backspace,
  Tab,
  Enter,
}

/// A keyboard input device.
pub trait KeyboardDevice {
  fn is_key_down(&self, key: VirtualKey) -> bool;
  fn is_key_up(&self, key: VirtualKey) -> bool;
}
