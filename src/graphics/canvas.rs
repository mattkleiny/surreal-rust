pub use widgets::*;

use crate::graphics::Color;
use crate::input::Key;
use crate::maths::Vector2;

/// An immediate-mode canvas framework.
pub struct Canvas {
  widgets: Vec<WidgetBox<Box<dyn Widget>>>,
}

struct WidgetBox<W> {
  inner: W,
  state: WidgetState,
}

struct WidgetState {
  is_focused: bool
}

impl Canvas {
  pub fn new() -> Self {
    Self { widgets: Vec::new() }
  }

  pub fn add_widget(&mut self, widget: impl Widget + 'static) {
    self.widgets.push(WidgetBox {
      inner: Box::new(widget),
      state: WidgetState {
        is_focused: false
      },
    })
  }

  // TODO: implement me

  pub fn input(&mut self, delta_time: f64) {}
  pub fn update(&mut self, delta_time: f64) {}
  pub fn draw(&mut self, delta_time: f64) {}
}

/// Size of a widget.
pub struct Size {}

/// Constraints for a widget.
pub struct Constraints {}

/// Represents a widget in a canvas.
pub trait Widget {
  fn on_event(&mut self, event: &Event);
  fn on_layout(&mut self, constaints: &Constraints) -> Size;
  fn on_update(&mut self);
  fn on_draw(&mut self);
}

/// Represents an event on the canvas.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Event {
  KeyDown {
    key: Key,
  },
  KeyUp {
    key: Key
  },
  MouseDown {
    position: Vector2<f32>,
    button: usize,
  },
  MouseUp {
    position: Vector2<f32>,
    button: usize,
  },
}

/// Styling for a canvas item.
pub struct Style {
  foreground_color: Color,
  background_color: Color,
}

impl Default for Style {
  fn default() -> Self {
    Self {
      foreground_color: Color::WHITE,
      background_color: Color::BLACK,
    }
  }
}

/// A brush for rendering operations.
pub struct Brush {}

mod widgets {
  use super::*;

  pub struct Button {
    label: String,
  }

  impl Button {
    pub fn new(label: impl AsRef<str>) -> Self {
      Self {
        label: label.as_ref().to_string()
      }
    }
  }

  impl Widget for Button {
    fn on_event(&mut self, event: &Event) {
      unimplemented!()
    }

    fn on_layout(&mut self, constaints: &Constraints) -> Size {
      unimplemented!()
    }

    fn on_update(&mut self) {
      unimplemented!()
    }

    fn on_draw(&mut self) {
      unimplemented!()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_build_a_canvas_and_attach_widgets() {
    let mut canvas = Canvas::new();

    canvas.add_widget(Button::new("Click Me!"));
  }
}
