use super::*;

/// An [`EditorPanel`] that renders an editable view of the current scene.
#[derive(Default)]
pub struct SceneView {}

impl EditorPanelContents for SceneView {
  fn show(&mut self, _ui: &mut egui::Ui, _context: &mut EditorContext) {
    surreal::diagnostics::profile_scope!("SceneView::show");
    // no-op
  }
}
