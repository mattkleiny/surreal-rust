use egui::{Response, Ui, Widget};

/// A simple tab control that swaps between varying UI elements.
#[derive(Default)]
pub struct TabControl {
  _tabs: Vec<Tab>,
  _selected_tab: Option<usize>,
}

/// A single tab in a [`TabControl`].
pub struct Tab {
  _name: String,
  _widget: Box<dyn Fn(&mut Ui) -> Response>,
}

impl Widget for TabControl {
  fn ui(self, _ui: &mut Ui) -> Response {
    todo!()
  }
}
