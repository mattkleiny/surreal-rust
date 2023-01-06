//! Inspector widget for object editing.

/// Represents a type that can be inspected by an [`InspectorWidget`].
pub trait Inspectable {
  fn inspect(&mut self, inspector: &mut InspectorWidget);
}

/// A widget for editing objects in-situ.
#[derive(Default)]
pub struct InspectorWidget {}

impl InspectorWidget {
  pub fn show(&mut self, _ui: &mut egui::Ui, _inspectable: &mut impl Inspectable) {
    todo!()
  }
}
