//! The user interface for the Surreal editor.

/// The main window for the editor.
#[must_use]
pub struct EditorWindow {
  name: &'static str,
}

impl Default for EditorWindow {
  fn default() -> Self {
    Self::new("Surreal Editor")
  }
}

impl EditorWindow {
  /// Creates a new [`EditorWindow`].
  pub fn new(name: &'static str) -> Self {
    Self { name }
  }

  /// Renders the [`EditorWindow`] in the given context.
  pub fn show(&mut self, egui: &egui::Context) {
    egui::Window::new(self.name).show(egui, |ui| {
      ui.label("Hello World!");
    });
  }
}
