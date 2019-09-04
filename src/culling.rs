//! A fast, 2d-only culling system for Surreal.
//!
//! This culling system is originally adapted from the following white-paper, but has since been adjusted
//! to make more sense within the context of Rust, and to honor different culling shapes, types and level
//! of detail: https://pdfs.semanticscholar.org/5622/a317d5a3696c8aade5d500e24120566cd300.pdf

use std::collections::LinkedList;

use crate::maths::{Rect, Vec2};

/// Describes a level of detail level.
///
/// Each LOD can exhibit different qualities based on distance to and visibility towards the camera,
/// and it's up to the consumer of the module to define said behaviours.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct LOD(u8);

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

/// An orthographic projection that can be used for culling.
pub trait OrthographicProjection {
  /// Gets the visible region of the projection in screen space.
  fn get_visible_region(&self) -> Rect;
}

/// A group of objects that may be culled relative to the game's camera.
///
/// This group is designed to be built once, and constantly updated.
///
/// The visibility calculations should be re-run prior to preparing each new frame of the game; if you wish to use
/// the culling information to adjust game-play systems, then performing the culling earlier on in the game loop is
/// recommended.
#[derive(Clone, Debug)]
pub struct CullingGroup<'a, T> {
  previous_snapshot: Option<CullingSnapshot<'a, T>>,
  current_snapshot: Option<CullingSnapshot<'a, T>>,
}

impl<'a, T> CullingGroup<'a, T> {
  pub fn new() -> Self {
    Self {
      previous_snapshot: None,
      current_snapshot: None,
    }
  }

  /// Culls objects are visible/not visible to the given projection.
  pub fn cull_scene(&mut self, projection: &impl OrthographicProjection) {
    let _visible_region = projection.get_visible_region();

    unimplemented!()
  }

  /// Resets the group's state.
  pub fn reset(&mut self) {
    self.previous_snapshot = None;
    self.current_snapshot = None;
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

/// A snapshot of the results of culling a particular culling group.
///
/// This snapshot is used to derive frame-by-frame culling deltas, and is core to the algorithm that we use.
#[derive(Clone, Debug)]
struct CullingSnapshot<'a, T> {
  elements: LinkedList<Cullable<T>>,
  first_visible: Option<&'a Cullable<T>>,
  last_visible: Option<&'a Cullable<T>>,
}

impl<'a, T> CullingSnapshot<'a, T> {
  pub fn new() -> Self {
    Self {
      elements: LinkedList::new(),
      first_visible: None,
      last_visible: None,
    }
  }
}