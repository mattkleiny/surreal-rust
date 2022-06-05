//! Mouse input management.

use std::collections::HashSet;

use crate::maths::{range, Vector2, vec2};
use winit::event::ElementState;

pub use winit::event::MouseButton;

/// Represents a mouse device on the system.
pub struct MouseDevice {
  position: Vector2<f32>,
  normalised_position: Vector2<f32>,
  previous_buttons: HashSet<MouseButton>,
  current_buttons: HashSet<MouseButton>,
}

impl MouseDevice {
  /// Creates a new mouse device.
  pub fn new() -> Self {
    Self {
      position: Vector2::new(0., 0.),
      normalised_position: Vector2::new(0., 0.),
      previous_buttons: HashSet::new(),
      current_buttons: HashSet::new(),
    }
  }

  /// Ticks the mouse.
  pub fn tick(&mut self) {
    self.previous_buttons.clear();
  }

  /// Handles mouse movement.
  pub fn on_mouse_moved(&mut self, new_position: Vector2<f32>, window_size: Vector2<f32>) {
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

  /// Gets the screen position of the mouse, origin (0, 0) in the top-left corner.
  pub fn position(&self) -> Vector2<f32> {
    self.position
  }

  /// Gets the normalised screen position of the mouse, origin (0, 0) in the top-left corner.
  pub fn normalised_position(&self) -> Vector2<f32> {
    self.normalised_position.clamp(range(0., 1.))
  }

  /// Gets the normalised screen position of the mouse, origin (0, 0) in the top-left corner.
  pub fn normalised_position_rescaled(&self) -> Vector2<f32> {
    (self.normalised_position.clamp(range(0., 1.)) * 2. - vec2(1., 1.)) / 2.
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
