use common::impl_variant_enum;

/// A keyboard input device.
pub trait KeyboardDevice {
  /// All pending keyboard events.
  fn events(&self) -> &[KeyboardEvent];
}

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
  F10,
  F11,
  F12,
  ArrowUp,
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  Space,
  Backspace,
  Tab,
  Enter,
}

impl_variant_enum!(VirtualKey, u32);
