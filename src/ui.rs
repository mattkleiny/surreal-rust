//! Immediate-mode UI framework for Surreal.

use crate::assets::Asset;
use crate::graphics::Texture;
use crate::maths::Rect;

pub type Bounds = Rect<f32>;

/// A canvas for immediate-mode rendering.
pub trait Canvas: Sized {
  fn label(&mut self, position: &Bounds, content: &Content) -> f32 {
    unimplemented!()
  }

  fn button(&mut self, position: &Bounds, content: &Content) -> bool {
    unimplemented!()
  }

  fn toggle(&mut self, position: &Bounds, content: &Content, value: bool) -> bool {
    unimplemented!()
  }

  fn int_field(&mut self, position: &Bounds, content: &Content, value: i64) -> i64 {
    unimplemented!()
  }

  fn float_field(&mut self, position: &Bounds, content: &Content, value: f64) -> f64 {
    unimplemented!()
  }

  fn slider(&mut self, position: &Bounds) -> f64 {
    unimplemented!()
  }

  fn min_max_slider(&mut self, position: &Bounds) -> (f64, f64) {
    unimplemented!()
  }

  fn property(
    &mut self,
    layout: &mut impl Layout,
    property: &mut impl crate::editor::Property,
    label: &Content,
  ) {
    let position = layout.next_bounds();

    property.on_property_gui(self, &position, label);
  }
}

/// Permit immediate-mode layouts on some canvas.
pub trait Layout {
  /// Calculates the next logical bounds within the layout.
  fn next_bounds(&mut self) -> Bounds;
}

/// A vertically-oriented layout.
pub struct VerticalLayout {}

impl Layout for VerticalLayout {
  fn next_bounds(&mut self) -> Bounds {
    unimplemented!()
  }
}

/// A horizontally-oriented layout.
pub struct HorizontalLayout {}

impl Layout for HorizontalLayout {
  fn next_bounds(&mut self) -> Bounds {
    unimplemented!()
  }
}

/// Renderable content (with optional icon).
pub enum Content {
  Label(&'static str),
  Icon(Option<Asset<Texture>>),
}

impl From<&'static str> for Content {
  fn from(text: &'static str) -> Self {
    Self::Label(text)
  }
}
