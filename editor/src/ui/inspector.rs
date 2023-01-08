use super::EditorContext;

/// Represents a type that can be inspected by an [`Inspector`].
pub trait Inspectable {
  fn inspect(&mut self, inspector: &mut Inspector);
}

/// A widget for editing objects in-situ.
#[derive(Default)]
pub struct Inspector {}

impl Inspector {
  /// Shows the [`Inspector`] in the given context.
  pub fn show(&mut self, ui: &mut egui::Ui, _context: &mut EditorContext) {
    ui.heading("Inspector");
    ui.label("Inspecting objects");
    // no-op
  }
}
