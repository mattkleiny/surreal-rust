//! Animation support.

use common::{Color, Color32, FastHashMap, Lerp, Quat, StringName, TimeSpan, Vec2, Vec3};

/// Represents a type that can be animated by an animation tree.
pub trait Animatable<V> {
  /// Applies the given value to the animatable type.
  fn apply(&mut self, track: AnimationTrack, time: f32);
}

/// An animation tree that can be used to drive animation state changes.
///
/// The animation tree is a directed acyclic graph (DAG) where each node is an
/// animation state. The root node is the current animation state, and the leaf
/// nodes are the animation states that are being dynamically selected.
pub struct AnimationTree<T = ()> {
  /// The state used to drive the animation tree.
  pub state: T,

  nodes: FastHashMap<StringName, AnimationState<T>>,
  current: Option<StringName>,
}

/// A single animation state in an animation tree.
pub struct AnimationState<T> {
  pub name: StringName,
  pub clip: AnimationClip,
  pub transitions: Vec<AnimationTransition<T>>,
  pub time_elapsed: TimeSpan,
  pub speed: f32,
}

/// A condition that must be met for a transition to occur.
type AnimationCondition<T> = Box<dyn Fn(&AnimationState<T>, &T) -> bool>;

/// A transition between two animation states.
pub struct AnimationTransition<T> {
  pub target: StringName,
  pub condition: AnimationCondition<T>,
}

/// A single clip of animation data.
#[derive(Default)]
pub struct AnimationClip {
  pub duration: TimeSpan,
  pub tracks: Vec<AnimationTrack>,
}

/// Data for a single animation track.
pub type AnimationTrackData<T> = Vec<AnimationKeyFrame<T>>;

/// A single track of animation data.
#[derive(Clone)]
pub enum AnimationTrack {
  Scalar(AnimationTrackData<f32>),
  Vec2(AnimationTrackData<Vec2>),
  Vec3(AnimationTrackData<Vec3>),
  Quat(AnimationTrackData<Quat>),
  Color(AnimationTrackData<Color>),
  Color32(AnimationTrackData<Color32>),
}

/// A single keyframe of animation data.
#[derive(Clone, Debug)]
pub struct AnimationKeyFrame<T> {
  pub time: f32,
  pub value: T,
}

impl<T> AnimationTree<T> {
  /// Creates a new animation tree.
  pub fn new(state: T) -> Self {
    AnimationTree {
      state,
      current: None,
      nodes: FastHashMap::default(),
    }
  }

  /// Adds a new animation state to the animation tree.
  pub fn add_state(&mut self, state: AnimationState<T>) {
    self.nodes.insert(state.name, state);
  }

  /// Removes an animation state from the animation tree.
  pub fn remove_state(&mut self, name: StringName) {
    self.nodes.remove(&name);
  }

  /// Gets the current animation state.
  pub fn current(&self) -> Option<&AnimationState<T>> {
    self.current.and_then(|it| self.nodes.get(&it))
  }

  /// Sets the current animation state.
  pub fn set_current(&mut self, name: StringName) {
    self.current = Some(name);
  }

  /// Gets the state of the animation tree.
  pub fn state(&self) -> &T {
    &self.state
  }

  /// Sets the state of the animation tree.
  pub fn set_state(&mut self, state: T) {
    self.state = state;
  }

  /// Applies a function to the state of the animation tree.
  pub fn modify_state(&mut self, body: impl FnOnce(&mut T)) {
    body(&mut self.state);
  }

  /// Updates the animation tree.
  pub fn update(&mut self, delta_time: f32) {
    if let Some(state) = self.current.and_then(|it| self.nodes.get_mut(&it)) {
      state.time_elapsed += TimeSpan::from_seconds(state.speed * delta_time);

      // loop the animation if it's finished
      if state.time_elapsed > state.clip.duration {
        state.time_elapsed = TimeSpan::ZERO;
      }

      // evaluate all transitions each tick
      for transition in &state.transitions {
        let AnimationTransition { condition, target } = transition;

        if condition(state, &self.state) {
          self.current = Some(*target);
          break;
        }
      }
    }
  }
}

/// Evaluates the final value of the given keyframes by interpolation.
pub fn evaluate_keyframes<T: Default + Lerp + Copy>(time: f32, keyframes: &[AnimationKeyFrame<T>]) -> T {
  // Handle empty keyframes case
  if keyframes.is_empty() {
    return T::default();
  }

  // If time is before first keyframe, return first value
  if time <= keyframes[0].time {
    return keyframes[0].value;
  }

  // If time is after last keyframe, return last value
  if time >= keyframes[keyframes.len() - 1].time {
    return keyframes[keyframes.len() - 1].value;
  }

  // Find the keyframes to interpolate between
  for i in 1..keyframes.len() {
    let prev = &keyframes[i - 1];
    let next = &keyframes[i];

    if next.time >= time {
      let t = (time - prev.time) / (next.time - prev.time);
      return T::lerp(prev.value, next.value, t);
    }
  }

  // Fallback (should never happen given above checks)
  T::default()
}

#[cfg(test)]
mod tests {
  use common::ToStringName;

  use super::*;

  /// Parameters for the animation state machine.
  #[derive(Default, Clone)]
  struct AnimationParams {
    pub is_walking: bool,
    pub is_jumping: bool,
  }

  #[test]
  fn it_should_evaluate_track_data() {
    let keyframes = vec![
      AnimationKeyFrame {
        time: 0.0,
        value: Vec2::ZERO,
      },
      AnimationKeyFrame {
        time: 1.0,
        value: Vec2::ONE,
      },
      AnimationKeyFrame {
        time: 2.0,
        value: Vec2::ZERO,
      },
    ];

    println!("{:?}", evaluate_keyframes(0.5, &keyframes));
    println!("{:?}", evaluate_keyframes(1.0, &keyframes));
    println!("{:?}", evaluate_keyframes(1.5, &keyframes));
    println!("{:?}", evaluate_keyframes(2.0, &keyframes));
    println!("{:?}", evaluate_keyframes(2.5, &keyframes));
  }

  #[test]
  fn it_should_support_basic_animations() {
    let mut tree = AnimationTree::new(AnimationParams::default());

    tree.add_state(AnimationState {
      name: "idle".to_string_name(),
      clip: AnimationClip {
        duration: TimeSpan::from_seconds(1.0),
        tracks: vec![
          AnimationTrack::Vec2(vec![
            AnimationKeyFrame {
              time: 0.0,
              value: Vec2::ZERO,
            },
            AnimationKeyFrame {
              time: 1.0,
              value: Vec2::ZERO,
            },
          ]),
          AnimationTrack::Color(vec![
            AnimationKeyFrame {
              time: 0.0,
              value: Color::BLACK,
            },
            AnimationKeyFrame {
              time: 1.0,
              value: Color::WHITE,
            },
          ]),
        ],
      },
      transitions: vec![
        AnimationTransition {
          target: "walk".to_string_name(),
          condition: Box::new(|_, p| p.is_walking),
        },
        AnimationTransition {
          target: "jump".to_string_name(),
          condition: Box::new(|_, p| p.is_jumping),
        },
      ],
      time_elapsed: TimeSpan::ZERO,
      speed: 1.0,
    });

    tree.update(0.5);
  }

  #[test]
  fn it_should_evaluate_keyframes() {
    let keyframes = vec![
      AnimationKeyFrame {
        time: 0.0,
        value: 0.0f32,
      },
      AnimationKeyFrame { time: 1.0, value: 1.0 },
      AnimationKeyFrame { time: 2.0, value: 0.0 },
    ];

    // Test exact keyframe times
    assert_eq!(evaluate_keyframes(0.0, &keyframes), 0.0);
    assert_eq!(evaluate_keyframes(1.0, &keyframes), 1.0);
    assert_eq!(evaluate_keyframes(2.0, &keyframes), 0.0);

    // Test interpolation between keyframes
    assert_eq!(evaluate_keyframes(0.5, &keyframes), 0.5);
    assert_eq!(evaluate_keyframes(1.5, &keyframes), 0.5);

    // Test clamping before first and after last keyframe
    assert_eq!(evaluate_keyframes(-1., &keyframes), 0.0);
    assert_eq!(evaluate_keyframes(3.0, &keyframes), 0.0);
  }
}
