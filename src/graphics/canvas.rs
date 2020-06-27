use crate::graphics::Color;
use crate::maths::Bounds2;

/// An immediate-mode canvas framework.
pub struct Canvas {
  active_style: Style,
}

impl Canvas {
  pub fn new() -> Self {
    Self { active_style: Style::default() }
  }

  pub fn draw_bounds(&mut self, bounds: Bounds2<f32>) {
    unimplemented!()
  }
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
