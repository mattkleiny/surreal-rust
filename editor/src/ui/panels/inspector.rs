use egui::Ui;

use super::*;

/// An [`EditorPanel`] that renders a read/write view of selected object
/// properties.
#[derive(Default)]
pub struct Inspector {}

impl EditorPanel for Inspector {
  fn show(&mut self, ui: &mut Ui, _context: &mut EditorContext) {
    surreal::diagnostics::profile_scope!("Inspector::show");

    ui.heading("Inspector");
    ui.label("Inspecting objects");
  }
}

/// Represents a type that can be inspected by an [`Inspector`].
pub trait Inspectable {
  fn inspect(&mut self, inspector: &mut Inspector);
}
