#![allow(unused_imports)]

use common::{Asset, FastHashMap, StringName, TimeSpan, Vec2};

use crate::{Color, Texture};

/// A persistent representation of an AnimationTree.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimationTreeDescriptor<T> {
  state: T,
  clips: Vec<AnimationStateDescriptor>,
}

/// A persistent representation of a single AnimationState.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimationStateDescriptor {
  name: String,
  speed: f32,
  clip: AnimationClipDescriptor,
}

/// A persistent representation of a single AnimationClip.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimationClipDescriptor {
  duration_in_seconds: f32,
}

/// An animation tree that can be used to drive animation state changes.
///
/// The animation tree is a directed acyclic graph (DAG) where each node is an
/// animation state. The root node is the current animation state, and the leaf
/// nodes are the animation states that are being dynamically selected.
#[derive(Default)]
pub struct AnimationTree<T = ()> {
  state: T,
  nodes: FastHashMap<StringName, AnimationState<T>>,
  current: Option<StringName>,
}

/// A single animation state in an animation tree.
pub struct AnimationState<T> {
  name: StringName,
  clip: AnimationClip,
  transitions: Vec<AnimationTransition<T>>,
  time_elapsed: TimeSpan,
  speed: f32,
}

/// A condition that must be met for a transition to occur.
type Condition<T> = Box<dyn Fn(&AnimationState<T>, &T) -> bool>;

/// A transition between two animation states.
pub struct AnimationTransition<T> {
  target: StringName,
  condition: Condition<T>,
}

/// A single clip of animation data.
#[derive(Default)]
pub struct AnimationClip {
  duration: TimeSpan,
  tracks: Vec<AnimationTrack>,
}

/// A single track of animation data.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnimationTrack {
  Position(AnimationTrackData<Vec2>),
  Rotation(AnimationTrackData<f32>),
  Scale(AnimationTrackData<Vec2>),
  Color(AnimationTrackData<Color>),
  Texture(AnimationTrackData<Asset<Texture>>),
}

/// Data for a single animation track.
pub type AnimationTrackData<T> = Vec<AnimationKeyFrame<T>>;

/// A single keyframe of animation data.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimationKeyFrame<T> {
  time: f32,
  value: T,
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

  /// Builds a descriptor from this animation tree.
  pub fn to_descriptor(&self) -> AnimationTreeDescriptor<T>
  where
    T: Clone,
  {
    AnimationTreeDescriptor {
      state: self.state.clone(),
      clips: self
        .nodes
        .values()
        .map(|it| AnimationStateDescriptor {
          name: it.name.to_string(),
          speed: it.speed,
          clip: AnimationClipDescriptor {
            duration_in_seconds: it.clip.duration.total_seconds(),
          },
        })
        .collect(),
    }
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
  fn it_should_support_basic_animations() {
    let mut tree = AnimationTree::new(AnimationParams::default());

    tree.add_state(AnimationState {
      name: "idle".to_string_name(),
      clip: AnimationClip {
        duration: TimeSpan::from_seconds(1.0),
        tracks: vec![
          AnimationTrack::Position(vec![
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
}
