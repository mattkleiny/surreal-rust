use common::maths::{Quat, Vec2, Vec3};

use crate::SceneNodeBuilder;

/// Represents a transform used in the hierarchy of a [`SceneGraph`].
///
/// This trait is used by [`SceneNode`]s and [`SceneComponent`]s to convey
/// position, orientation, and scale information to the scene graph.
pub trait SceneTransform: Default + Sized {}

/// A no-op transform.
impl SceneTransform for () {}

/// A transform in 2D space.
#[derive(Default)]
pub struct Transform2D {
  position: Vec2,
  rotation: f32,
  scale: Vec2,
}

impl SceneTransform for Transform2D {}

/// Helper methods for building [`SceneNode`]s with [`Transform2D`]s.
impl<'a> SceneNodeBuilder<'a, Transform2D> {
  /// Sets the position of the node.
  pub fn position(mut self, position: Vec2) -> Self {
    self.transform.position = position;
    self
  }

  /// Sets the rotation of the node.
  pub fn rotation(mut self, rotation: f32) -> Self {
    self.transform.rotation = rotation;
    self
  }

  /// Sets the scale of the node.
  pub fn scale(mut self, scale: Vec2) -> Self {
    self.transform.scale = scale;
    self
  }
}

/// A transform in 3D space.
#[derive(Default)]
pub struct Transform3D {
  position: Vec3,
  rotation: Quat,
  scale: Vec3,
}

impl SceneTransform for Transform3D {}

/// Helper methods for building [`SceneNode`]s with [`Transform3D`]s.
impl<'a> SceneNodeBuilder<'a, Transform3D> {
  /// Sets the position of the node.
  pub fn position(mut self, position: Vec3) -> Self {
    self.transform.position = position;
    self
  }

  /// Sets the rotation of the node.
  pub fn rotation(mut self, rotation: Quat) -> Self {
    self.transform.rotation = rotation;
    self
  }

  /// Sets the scale of the node.
  pub fn scale(mut self, scale: Vec3) -> Self {
    self.transform.scale = scale;
    self
  }
}
