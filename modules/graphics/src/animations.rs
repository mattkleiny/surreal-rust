//! Animation support for Surreal

use common::{AssetRef, FastHashMap, StringName, TimeSpan, Vec2};

use crate::{Color, Texture};

/// An animation tree that can be used to drive animation state changes.
///
/// The animation tree is a directed acyclic graph (DAG) where each node is an
/// animation state. The root node is the current animation state, and the leaf
/// nodes are the animation states that are being dynamically selected.
#[derive(Default)]
pub struct AnimationTree<T = ()> {
  parameters: T,
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
pub struct AnimationClip {
  duration: TimeSpan,
  tracks: Vec<AnimationTrack>,
}

/// A single track of animation data.
pub enum AnimationTrack {
  Position { keyframes: Vec<AnimationKeyFrame<Vec2>> },
  Rotation { keyframes: Vec<AnimationKeyFrame<f32>> },
  Scale { keyframes: Vec<AnimationKeyFrame<Vec2>> },
  Color { keyframes: Vec<AnimationKeyFrame<Color>> },
  Texture { keyframes: Vec<AnimationKeyFrame<AssetRef<Texture>>> },
}

/// A single keyframe of animation data.
#[derive(Clone, Debug)]
pub struct AnimationKeyFrame<T> {
  time: f32,
  value: T,
}

impl<T> AnimationTree<T> {
  /// Creates a new animation tree.
  pub fn new(parameters: T) -> Self {
    AnimationTree {
      parameters,
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

  /// Updates the animation tree.
  pub fn update(&mut self, delta_time: f32) {
    if let Some(state) = self.current.and_then(|it| self.nodes.get_mut(&it)) {
      state.time_elapsed += TimeSpan::from_seconds(state.speed * delta_time);

      // loop the animation if it's finished
      if state.time_elapsed > state.clip.duration {
        state.time_elapsed = TimeSpan::ZERO;
      }

      // evaluate transitions
      for transition in &state.transitions {
        let AnimationTransition { condition, target } = transition;

        if condition(state, &self.parameters) {
          self.current = Some(target.clone());
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
  struct AnimationParams {
    pub is_walking: bool,
    pub is_running: bool,
    pub is_falling: bool,
  }

  #[test]
  fn it_should_support_basic_animations() {
    let mut tree = AnimationTree::new(AnimationParams {
      is_walking: false,
      is_running: false,
      is_falling: false,
    });

    tree.add_state(AnimationState {
      name: "idle".to_string_name(),
      clip: AnimationClip {
        duration: TimeSpan::from_seconds(1.0),
        tracks: vec![AnimationTrack::Color {
          keyframes: vec![
            AnimationKeyFrame {
              time: 0.0,
              value: Color::WHITE,
            },
            AnimationKeyFrame {
              time: 1.0,
              value: Color::RED,
            },
          ],
        }],
      },
      transitions: vec![AnimationTransition {
        target: "walk".to_string_name(),
        condition: Box::new(|_, p| p.is_walking),
      }],
      time_elapsed: TimeSpan::ZERO,
      speed: 1.0,
    });

    tree.add_state(AnimationState {
      name: "walk".to_string_name(),
      clip: AnimationClip {
        duration: TimeSpan::from_seconds(1.0),
        tracks: vec![AnimationTrack::Color {
          keyframes: vec![
            AnimationKeyFrame {
              time: 0.0,
              value: Color::WHITE,
            },
            AnimationKeyFrame {
              time: 1.0,
              value: Color::BLACK,
            },
          ],
        }],
      },
      transitions: vec![],
      time_elapsed: TimeSpan::ZERO,
      speed: 1.0,
    });

    tree.update(0.5);
  }
}
