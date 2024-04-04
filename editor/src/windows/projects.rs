use super::*;

/// Displays all projects in a directory.
#[derive(Default)]
pub struct ProjectWindow {}

impl EditorWindow for ProjectWindow {
  fn update(&mut self) -> bool {
    true
  }

  fn present(&mut self) {}
}
