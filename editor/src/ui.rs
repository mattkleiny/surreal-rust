//! The user interface for the Surreal editor.

pub use graphs::*;
use surreal::graphs::Graph;

mod graphs;

/// The main window for the editor.
pub struct EditorWindow {
  graph_editor: GraphEditor<u32>,
}

impl EditorWindow {
  /// Builds a new [`EditorWindow`].
  pub fn new() -> Self {
    Self {
      graph_editor: GraphEditor::from_graph(Graph::default()),
    }
  }
}

impl EditorWindow {
  /// Renders the [`EditorWindow`] in the given context.
  pub fn show(&mut self, egui: &egui::Context) {
    egui::CentralPanel::default()
      .frame(egui::Frame::default().inner_margin(0.0))
      .show(egui, |ui| {
        self.graph_editor.show(ui, ui.available_rect_before_wrap());
      });
  }
}
