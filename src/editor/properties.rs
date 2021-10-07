use crate::maths::Range;
use crate::ui::*;

/// Permits rendering of mutable properties in a canvas.
pub trait PropertyEditor {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content);
}

impl PropertyEditor for bool {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    *self = canvas.toggle(&position, label, *self);
  }
}

impl PropertyEditor for i32 {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    *self = canvas.int_field(&position, label, *self as i64) as i32;
  }
}

impl PropertyEditor for i64 {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    *self = canvas.int_field(&position, label, *self);
  }
}

impl PropertyEditor for f64 {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    *self = canvas.float_field(&position, label, *self);
  }
}

impl PropertyEditor for f32 {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    *self = canvas.float_field(&position, label, *self as f64) as f32;
  }
}

impl PropertyEditor for Range<f32> {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    let (min, max) = canvas.min_max_slider(&position);

    self.min = min as f32;
    self.max = max as f32;
  }
}

impl PropertyEditor for Range<f64> {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    let (min, max) = canvas.min_max_slider(&position);

    self.min = min;
    self.max = max;
  }
}
