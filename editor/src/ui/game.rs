use surreal::diagnostics::profiling;

use super::*;

#[derive(Default)]
pub struct GameView {}

impl GameView {
  #[profiling::function]
  pub fn show(&mut self, _ui: &mut egui::Ui, _context: &mut EditorContext) {
    // no-op
  }
}
