use super::*;

/// An [`EditorPanel`] that renders a list of all known application assets.
#[derive(Default)]
pub struct ContentBrowser {}

impl ContentBrowser {
  pub fn show(&mut self, _ui: &mut egui::Ui, _context: &mut EditorContext) {
    surreal::diagnostics::profile_scope!("ContentBrowser::show");

    // no-op
  }
}
