//! The user interface for the Surreal editor.

/// The main window for the editor.
#[derive(Default)]
pub struct EditorWindow {}

impl EditorWindow {
  /// Renders the [`EditorWindow`] in the given context.
  pub fn show(&mut self, egui: &egui::Context) {
    egui::CentralPanel::default().show(egui, |ui| {
      ui.heading("Surreal Editor");
      ui.label("Hello World!");
    });
  }
}
