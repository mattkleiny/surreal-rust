use egui::{Response, Sense, Ui, Widget};
use std::borrow::Cow;

/// A simple tab control that swaps between varying UI elements.
#[derive(Default)]
pub struct TabControl {
  tabs: Vec<Tab>,
}

/// A single tab in a [`TabControl`].
pub struct Tab {
  name: Cow<'static, str>,
}

impl TabControl {
  pub fn selected_tab(&self) -> Option<u32> {
    todo!()
  }

  pub fn with_tab(mut self, name: &'static str) -> Self {
    self.tabs.push(Tab { name: Cow::Borrowed(name) });
    self
  }
}

impl Widget for TabControl {
  fn ui(self, _ui: &mut Ui) -> Response {
    todo!()
  }
}
