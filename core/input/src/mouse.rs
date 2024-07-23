use common::Vec2;

/// A mouse event.
#[derive(Debug, Clone)]
pub enum MouseEvent {
  MouseMove { position: Vec2, delta: Vec2 },
  MouseDown { button: MouseButton },
  MouseUp { button: MouseButton },
}

/// Possible mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
  Left,
  Right,
  Middle,
  Extra(u8),
}

/// A mouse input device.
pub trait MouseDevice {}
