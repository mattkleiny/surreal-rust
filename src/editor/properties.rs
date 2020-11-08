use crate::maths::Range;
use crate::ui::*;

/// Permits rendering of mutable properties in a canvas.
pub trait Property {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content);
}

impl Property for bool {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    *self = canvas.toggle(&position, label, *self);
  }
}

impl Property for i32 {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    *self = canvas.int_field(&position, label, *self as i64) as i32;
  }
}

impl Property for i64 {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    *self = canvas.int_field(&position, label, *self);
  }
}

impl Property for f64 {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    *self = canvas.float_field(&position, label, *self);
  }
}

impl Property for f32 {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    *self = canvas.float_field(&position, label, *self as f64) as f32;
  }
}

impl Property for Range<f32> {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    let (min, max) = canvas.min_max_slider(&position);

    self.min = min as f32;
    self.max = max as f32;
  }
}

impl Property for Range<f64> {
  fn on_property_gui(&mut self, canvas: &mut impl Canvas, position: &Bounds, label: &Content) {
    let (min, max) = canvas.min_max_slider(&position);

    self.min = min;
    self.max = max;
  }
}
