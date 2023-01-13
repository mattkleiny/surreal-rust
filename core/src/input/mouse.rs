//! Mouse input management.

use winit::event::ElementState;
pub use winit::event::MouseButton;

use crate::{collections::FastHashSet, maths::Vec2};

/// Represents a mouse device on the system.
#[derive(Default)]
pub struct MouseDevice {
  position: Vec2,
  normalised_position: Vec2,
  previous_buttons: FastHashSet<MouseButton>,
  current_buttons: FastHashSet<MouseButton>,
}

impl MouseDevice {
  /// Ticks the mouse.
  pub fn tick(&mut self) {
    self.previous_buttons.clear();
  }

  /// Handles mouse movement.
  pub fn on_mouse_moved(&mut self, new_position: Vec2, window_size: Vec2) {
    self.position = new_position;
    self.normalised_position = new_position / window_size;
  }

  /// Handles mouse click events.
  pub fn on_mouse_button(&mut self, button: MouseButton, state: ElementState) {
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

  /// Gets the screen position of the mouse, origin (0, 0) in the top-left
  /// corner.
  pub fn position(&self) -> Vec2 {
    self.position
  }

  /// Gets the normalised screen position of the mouse, origin (0, 0) in the
  /// top-left corner.
  pub fn normalised_position(&self) -> Vec2 {
    self.normalised_position
  }

  /// Is the given mouse button currently up?
  pub fn is_button_up(&self, button: MouseButton) -> bool {
    !self.current_buttons.contains(&button)
  }

  /// Is the given mouse button currently down?
  pub fn is_button_down(&self, button: MouseButton) -> bool {
    self.current_buttons.contains(&button)
  }

  /// Is the given mouse button pressed this frame?
  pub fn is_button_pressed(&self, button: MouseButton) -> bool {
    self.previous_buttons.contains(&button)
  }
}
