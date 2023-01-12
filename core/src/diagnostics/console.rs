//! In-process console with commands for diagnostics and changing state.

use crate::collections::RingBuffer;

/// A managed `egui` panel for the in-game console.
pub struct ConsolePanel {
  input_buffer: String,
  history: RingBuffer<String>,
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
    egui::ScrollArea::vertical().show(ui, |ui| {
      ui.vertical(|ui| {
        for line in self.history.iter() {
          ui.label(line);
        }

        let response = ui.text_edit_singleline(&mut self.input_buffer);
        response.request_focus();
        if response.has_focus() && response.ctx.input().key_pressed(egui::Key::Enter) {
          self.history.push(self.input_buffer.clone());
          self.input_buffer.clear();
        }
      });
    });
  }
}
