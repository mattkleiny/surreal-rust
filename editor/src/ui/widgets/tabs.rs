/// A control of [`Tab`]s.
#[derive(Default)]
pub struct TabControl {
  tabs: Vec<Tab>,
}

/// A single tab in a [`TabControl`].
struct Tab {
  _name: &'static str,
}

impl TabControl {
  pub fn with_tab(mut self, name: &'static str) -> Self {
    self.tabs.push(Tab { _name: name });
    self
  }
}
