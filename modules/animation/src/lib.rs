//! Animation support for Surreal

use core::maths::Lerp;
use core::utilities::TimeSpan;

/// An animation that evaluates one or more [`AnimationTrack`]s over time.
pub struct Animation {
  pub name: String,
  pub duration: TimeSpan,
  pub tracks: Vec<Box<dyn AnimationTrack>>,
}

impl Animation {
  pub fn advance(&mut self, time: TimeSpan) {
    for track in &mut self.tracks {
      track.advance(time);
    }
  }
}

/// A single track in an [`Animation`].
pub trait AnimationTrack {
  fn advance(&mut self, time: TimeSpan);
}

/// An [`AnimationTrack`] that evaluates [`KeyFrame`]s through interpolation.
pub struct KeyFrameTrack<T: Lerp> {
  pub keyframes: Vec<KeyFrame<T>>,
  pub evaluator: Interpolator<T>,
}

/// A single key-frame in a [`KeyFrameTrack`].
pub struct KeyFrame<T> {
  pub normalised_time: f32,
  pub value: T,
}

/// A function for interpolating values in a [`KeyFrameTrack`].
pub type Interpolator<T> = fn(&T, &T, f32) -> T;

impl<T: Lerp> AnimationTrack for KeyFrameTrack<T> {
  fn advance(&mut self, time: TimeSpan) {
    let a = &self.keyframes[0].value;
    let b = &self.keyframes[1].value;

    (self.evaluator)(a, b, time.total_seconds() as f32);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn keyframe_track_should_evaluate() {
    let mut animation = Animation {
      name: "Test".to_string(),
      duration: TimeSpan::from_seconds(1.0),
      tracks: vec![Box::new(KeyFrameTrack {
        keyframes: vec![
          KeyFrame {
            normalised_time: 0.0,
            value: 0.0,
          },
          KeyFrame {
            normalised_time: 1.0,
            value: 1.0,
          },
        ],
        evaluator: |a, b, t| f32::lerp(*a, *b, t),
      })],
    };

    animation.advance(TimeSpan::from_seconds(0.5));
  }
}
