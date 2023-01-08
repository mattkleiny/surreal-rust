use super::*;

/// An [`EditorPanel`] that renders an editable view of the current scene.
#[derive(Default)]
pub struct SceneView {}

impl SceneView {
  pub fn show(&mut self, _ui: &mut egui::Ui, _context: &mut EditorContext) {
    surreal::diagnostics::profiling::profile_scope!("SceneView::show");
    // no-op
  }
}
