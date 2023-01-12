use super::*;

/// An [`EditorPanel`] that renders a view of the actively running game and scene.
#[derive(Default)]
pub struct GameView {}

impl GameView {
  pub fn show(&mut self, _ui: &mut egui::Ui, _context: &mut EditorContext) {
    surreal::diagnostics::profile_scope!("GameView::show");

    // no-op
  }
}
