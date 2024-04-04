use super::*;

/// Displays the main game window.
#[derive(Default)]
pub struct GameWindow {}

impl EditorWindow for GameWindow {
  fn update(&mut self) -> bool {
    true
  }

  fn present(&mut self) {}
}
