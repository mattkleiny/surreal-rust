//! General-purpose editor support for Surreal

pub use properties::*;

use crate::ui::*;

mod properties;

/// Permits inspecting an object's properties.
pub trait Inspector {
  fn on_inspector_gui(&mut self, canvas: &mut impl Canvas, layout: &mut impl Layout);
}

/// Generic editor canvas extensions.
pub trait EditorCanvas: Canvas {
  fn property(&mut self, layout: &mut impl Layout, property: &mut impl PropertyEditor, label: &Content) {
    let position = layout.next_bounds();

    property.on_property_gui(self, &position, label);
  }
}

impl<C: Canvas> EditorCanvas for C {}

#[cfg(test)]
mod tests {
  use crate::maths::Range;

  use super::*;

  struct ExampleState {
    health: f32,
    chance: Range<f32>,
  }

  impl Inspector for ExampleState {
    fn on_inspector_gui(&mut self, canvas: &mut impl Canvas, layout: &mut impl Layout) {
      canvas.property(layout, &mut self.health, &"Health".into());
      canvas.property(layout, &mut self.chance, &"Chance".into());
    }
  }
}
