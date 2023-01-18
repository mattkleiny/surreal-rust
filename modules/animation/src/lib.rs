//! Animation support for Surreal

use surreal::{graphics::TextureRegion, maths::Lerp, utilities::TimeSpan};

/// A function for interpolating values in a [`KeyFrameTrack`].
pub type Interpolator<T> = fn(&T, &T, f32) -> T;

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
  fn advance(&mut self, duration: TimeSpan);
}

/// An [`AnimationTrack`] that evaluates [`KeyFrame`]s through interpolation.
pub struct KeyFrameTrack<T> {
  pub current_value: T,
  pub key_frames: Vec<KeyFrame<T>>,
  pub evaluator: Interpolator<T>,
}

/// A single key-frame in a [`KeyFrameTrack`].
pub struct KeyFrame<T> {
  pub value: T,
  pub normalised_time: f32,
}

impl<T: Lerp> AnimationTrack for KeyFrameTrack<T> {
  fn advance(&mut self, duration: TimeSpan) {
    let a = &self.key_frames[0].value;
    let b = &self.key_frames[1].value;

    let result = (self.evaluator)(a, b, duration.total_seconds());

    self.current_value = result;
  }
}

impl<'a> AnimationTrack for KeyFrameTrack<SpriteFrame<'a>> {
  fn advance(&mut self, _duration: TimeSpan) {
    todo!()
  }
}

/// A single frame in a [`KeyFrameTrack`] for use in sprite animations.
#[derive(Clone)]
pub struct SpriteFrame<'a> {
  pub texture: &'a TextureRegion,
  pub duration: TimeSpan,
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
        current_value: 0.,
        key_frames: vec![
          KeyFrame {
            normalised_time: 0.,
            value: 0.,
          },
          KeyFrame {
            normalised_time: 1.,
            value: 1.,
          },
        ],
        evaluator: |a, b, t| f32::lerp(*a, *b, t),
      })],
    };

    animation.advance(TimeSpan::from_seconds(0.5));
  }
}
