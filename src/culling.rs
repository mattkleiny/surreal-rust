//! A fast, 2d-only culling system for Surreal.
//!
//! This culling system is originally adapted from the following white-paper, but has since been adjusted to make more
//! sense within the context of Rust, and to honor different culling shapes, types and level of detail:
//!
//! See https://pdfs.semanticscholar.org/5622/a317d5a3696c8aade5d500e24120566cd300.pdf

use std::collections::LinkedList;

use glam::Vec2;
use crate::maths::Rect;

/// A camera that can be used for culling.
pub trait CullingCamera {
  /// Gets the visible area of the camera, relative to it's screen space.
  fn get_visible_rect(&self) -> Rect;
}

/// Describes a level of detail level.
///
/// Different levels of detail can exhibit different qualities based on distance to and visibility towards the camera.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct LOD(u8);

/// A group of objects that may be culled relative to the game's camera.
///
/// This group is designed to be built once, and constantly updated.
///
/// The visibility calculations should be re-run prior to preparing each new frame of the game; if you wish to use
/// the culling information to adjust game-play systems, then performing the culling earlier on in the game loop is
/// recommended.
#[derive(Clone, Debug)]
pub struct CullingGroup<'a, T> {
  elements: LinkedList<Cullable<T>>,
  first_visible: Option<&'a Cullable<T>>,
  last_visible: Option<&'a Cullable<T>>,
}

impl<'a, T> CullingGroup<'a, T> {
  pub fn new() -> Self {
    Self {
      elements: LinkedList::new(),
      first_visible: None,
      last_visible: None,
    }
  }

  /// Adds a new element to the group.
  pub fn add(&mut self, element: T, shape: CullingShape) {
    self.elements.push_back(Cullable {
      element,
      is_visible: false,
      current_lod: LOD(0),
      shape,
    })
  }

  /// Culls objects are visible/not visible to the given camera.
  pub fn recalculate_visible_objects(&mut self, camera: &impl CullingCamera) {
    let _visible_rect = camera.get_visible_rect();

    unimplemented!()
  }

  /// Clears all elements from teh group.
  pub fn clear(&mut self) {
    self.elements.clear();
    self.first_visible = None;
    self.last_visible = None;
  }
}

/// Represents some object that may be culled in a culling group.
#[derive(Clone, Debug)]
struct Cullable<T> {
  /// The underlying element that we're culling.
  element: T,
  /// True if the element is visible as of the last culling pass.
  is_visible: bool,
  /// The current level of detail that this object should exhibit.
  current_lod: LOD,
  /// The shape used to perform visibility-based trivial rejection.
  shape: CullingShape,
}

/// A shape that can be culled relative to the camera.
#[derive(Clone, Debug)]
pub enum CullingShape {
  /// A simple circular shape in 2-space.
  Circle {
    radius: f32,
    center: Vec2,
  },
  /// An axially aligned bounding box.
  AABB(Rect),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn culling_api_should_be_easy_to_work_with() {
    let mut group = CullingGroup::new();

    group.add("Sprite 1", CullingShape::Circle {
      radius: 3.0,
      center: Vec2::new(0.5, 0.5),
    });
  }
}