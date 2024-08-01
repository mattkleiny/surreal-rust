use common::{impl_variant_enum, Vec2};

/// A mouse input device.
pub trait MouseDevice {
  /// All pending mouse events.
  fn events(&self) -> &[MouseEvent];
}

/// A mouse event.
#[derive(Debug, Clone)]
pub enum MouseEvent {
  MouseMove { position: Vec2, delta: Vec2 },
  MouseDown(MouseButton),
  MouseUp(MouseButton),
}

/// Possible mouse buttons.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
  Left,
  Right,
  Middle,
}

impl_variant_enum!(MouseButton, u32);
