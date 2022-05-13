use super::*;

/// The server for input management.
pub struct DesktopInputServer {
  // mouse
  mouse_position: Vector2<f64>,
  mouse_delta: Vector2<f64>,
  pressed_buttons: HashSet<MouseButton>,
  released_buttons: HashSet<MouseButton>,

  // keyboard
  pressed_keys: HashSet<Key>,
  released_keys: HashSet<Key>,
}

impl DesktopInputServer {
  pub fn new() -> Self {
    Self {
      mouse_position: vec2(0., 0.),
      mouse_delta: vec2(0., 0.),
      pressed_buttons: HashSet::new(),
      released_buttons: HashSet::new(),
      pressed_keys: HashSet::new(),
      released_keys: HashSet::new(),
    }
  }
}