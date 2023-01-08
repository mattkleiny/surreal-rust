use surreal::diagnostics::profiling;

use super::*;

/// An [`EditorPanel`] that renders an editable view of the current scene.
#[derive(Default)]
pub struct SceneView {}

impl SceneView {
  #[profiling::function]
  pub fn show(&mut self, _ui: &mut egui::Ui, _context: &mut EditorContext) {
    // no-op
  }
}
