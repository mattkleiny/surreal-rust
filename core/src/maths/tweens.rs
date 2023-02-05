use super::{Curve, Lerp};
use crate::{
  fibers::yield_fiber,
  graphics::{Color, Color32},
  utilities::TimeSpan,
};

/// Defines how to evaluate a tween effect over time with a curve and duration.
pub struct TweenAnimation {
  duration: TimeSpan,
  curve: Box<dyn Curve>,
}

impl Default for TweenAnimation {
  fn default() -> Self {
    Self {
      duration: TimeSpan::from_seconds(1.0),
      curve: Box::new(|t| t), // linear
    }
  }
}

impl TweenAnimation {
  /// Evaluates this animation by invoking the given function with a normalised
  /// value between 0 and 1 over the duration of the animation.
  ///
  /// The function will be invoked once per frame, and will yield after each update.
  pub async fn evaluate(&self, mut body: impl FnMut(f32)) {
    let mut time = TimeSpan::ZERO;

    while time < self.duration {
      let normal = time.total_seconds() / self.duration.total_seconds();
      let normal = self.curve.evaluate(normal);

      body(normal);

      time += TimeSpan::from_seconds(0.016);

      yield_fiber().await
    }
  }
}

/// A type that can be animated over time and can establish dependencies with other signals.
pub trait Animatable: Lerp + Copy {
  /// Tweens the value of this object to the given value over the given duration.
  async fn tween_to(&mut self, value: Self, tween: TweenAnimation) {
    let start = *self; // make a copy of the starting value
    tween.evaluate(move |normal| *self = Self::lerp(start, value, normal)).await
  }
}

macro_rules! impl_animatable {
  ($type:ty) => {
    impl Animatable for $type {}
  };
}

impl_animatable!(u8);
impl_animatable!(u16);
impl_animatable!(u32);
impl_animatable!(u64);
impl_animatable!(i8);
impl_animatable!(i16);
impl_animatable!(i32);
impl_animatable!(i64);
impl_animatable!(f32);
impl_animatable!(f64);
impl_animatable!(Color);
impl_animatable!(Color32);

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fibers::block_on;

  #[test]
  fn animatable_values_should_work() {
    let mut test = 0f32;

    block_on(test.tween_to(1.0, TweenAnimation::default()));

    assert!(test - 1.0 < f32::EPSILON);
  }
}
