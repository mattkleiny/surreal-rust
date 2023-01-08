use surreal::diagnostics::profiling;

use super::*;

/// An [`EditorPanel`] that renders a list of all known application assets.
#[derive(Default)]
pub struct ContentBrowser {}

impl ContentBrowser {
  #[profiling::function]
  pub fn show(&mut self, _ui: &mut egui::Ui, _context: &mut EditorContext) {
    // no-op
  }
}
