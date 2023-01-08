use egui::{Response, Ui, Widget};

/// A simple tab control that swaps between varying UI elements.
pub struct TabControl {
  tabs: Vec<Tab>,
  selected_tab: usize,
}

/// A single tab in a [`TabControl`].
pub struct Tab {
  name: String,
  widget: Box<dyn Fn(&mut Ui) -> Response>,
}

impl Widget for TabControl {
  fn ui(self, _ui: &mut Ui) -> Response {
    todo!()
  }
}
