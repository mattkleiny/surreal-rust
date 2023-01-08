use surreal::diagnostics::profiling;

use super::*;

/// An [`EditorPanel`] that renders a view of the actively running game and scene.
#[derive(Default)]
pub struct GameView {}

impl GameView {
  #[profiling::function]
  pub fn show(&mut self, _ui: &mut egui::Ui, _context: &mut EditorContext) {
    // no-op
  }
}
