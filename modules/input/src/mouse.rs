use common::{FastHashSet, Vec2};

use super::*;

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
pub struct MouseDevice {
  _cursor_pos: Vec2,
  _cursor_delta: Vec2,
  _pressed_buttons: FastHashSet<MouseButton>,
  queued_events: Vec<MouseEvent>,
}

impl InputDevice for MouseDevice {
  fn update(&mut self, _delta_time: f32) {
    todo!()
  }

  fn drain_events(&mut self) -> Vec<InputEvent> {
    let mut events = Vec::with_capacity(self.queued_events.len());

    for event in self.queued_events.drain(..) {
      events.push(InputEvent::MouseEvent(event));
    }

    events
  }
}
