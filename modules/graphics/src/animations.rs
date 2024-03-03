//! Animation support for Surreal

use common::{FastHashMap, Ref, StringName, Vec2};

use crate::{Color, Texture};

/// An animation tree that can be used to drive animation state changes.
///
/// The animation tree is a directed acyclic graph (DAG) where each node is an
/// animation state. The root node is the current animation state, and the leaf
/// nodes are the animation states that are being dynamically selected.
pub struct AnimationTree<T = ()> {
  current: Option<StringName>,
  parameters: T,
  nodes: FastHashMap<StringName, AnimationState<T>>,
}

/// A single animation state in an animation tree.
pub struct AnimationState<T> {
  name: StringName,
  clip: AnimationClip,
  transitions: Vec<AnimationTransition<T>>,
  time: f32,
  speed: f32,
}

/// A transition between two animation states.
pub struct AnimationTransition<T> {
  to: StringName,
  condition: Box<dyn Fn(&AnimationState<T>, &T) -> bool>,
}

/// A single clip of animation data.
pub struct AnimationClip {
  duration: f32,
  tracks: Vec<AnimationTrack>,
}

/// A single track of animation data.
pub enum AnimationTrack {
  Position { keyframes: Vec<Keyframe<Vec2>> },
  Rotation { keyframes: Vec<Keyframe<f32>> },
  Scale { keyframes: Vec<Keyframe<Vec2>> },
  Color { keyframes: Vec<Keyframe<Color>> },
  Texture { keyframes: Vec<Keyframe<Ref<Texture>>> },
}

/// A single keyframe of animation data.
#[derive(Clone, Debug)]
pub struct Keyframe<T> {
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
      state.time += state.speed * delta_time;

      if state.time > state.clip.duration {
        state.time = state.time % state.clip.duration;
      }

      for AnimationTransition { condition, to } in &state.transitions {
        if condition(state, &self.parameters) {
          self.current = Some(to.clone());
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

    let idle = tree.add_state(AnimationState {
      name: "idle".to_string_name(),
      clip: AnimationClip {
        duration: 1.0,
        tracks: vec![AnimationTrack::Color {
          keyframes: vec![
            Keyframe {
              time: 0.0,
              value: Color::WHITE,
            },
            Keyframe {
              time: 0.5,
              value: Color::BLACK,
            },
            Keyframe {
              time: 1.0,
              value: Color::WHITE,
            },
          ],
        }],
      },
      transitions: vec![AnimationTransition {
        to: "walk".to_string_name(),
        condition: Box::new(|_, p| p.is_walking),
      }],
      time: 0.0,
      speed: 1.0,
    });

    let walk = tree.add_state(AnimationState {
      name: "walk".to_string_name(),
      clip: AnimationClip {
        duration: 1.0,
        tracks: vec![AnimationTrack::Color {
          keyframes: vec![
            Keyframe {
              time: 0.0,
              value: Color::WHITE,
            },
            Keyframe {
              time: 0.5,
              value: Color::RED,
            },
            Keyframe {
              time: 1.0,
              value: Color::WHITE,
            },
          ],
        }],
      },
      transitions: vec![AnimationTransition {
        to: "idle".to_string_name(),
        condition: Box::new(|_, p| !p.is_walking),
      }],
      time: 0.0,
      speed: 1.0,
    });

    tree.update(0.5);
  }
}
