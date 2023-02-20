use crate::collections::RingBuffer;

/// A managed `egui` panel for the in-game console.
pub struct ConsolePanel {
  input_buffer: String,
  history: RingBuffer<String>,
}

impl Default for ConsolePanel {
  fn default() -> Self {
    Self::new()
  }
}

impl ConsolePanel {
  /// Creates a new [`ConsolePanel`].
  pub fn new() -> Self {
    Self {
      input_buffer: "".to_string(),
      history: RingBuffer::new(100),
    }
  }

  /// Shows the [`ConsolePanel`] in the given [`egui::Ui`].
  pub fn show(&mut self, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |_i| {
      // TODO: implement me
    });
  }
}
