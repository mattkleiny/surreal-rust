use common::maths::{Mat4, Quat, Vec2, Vec3};

use super::SceneNodeBuilder;

/// Represents a transform used in the hierarchy of a [`SceneGraph`].
///
/// This trait is used by [`SceneNode`]s and [`SceneComponent`]s to convey
/// position, orientation, and scale information to the scene graph.
pub trait Transform: Default + Sized {
  /// Updates the transform relative to its parent.
  fn update_transform(&mut self, parent: &Self);
}

/// A no-op transform.
impl Transform for () {
  fn update_transform(&mut self, _parent: &Self) {
    // no-op
  }
}

/// A transform in 2D space.
#[derive(Default)]
pub struct Transform2D {
  position: Vec2,
  rotation: f32,
  scale: Vec2,
  local_to_world: Mat4,
}

impl Transform for Transform2D {
  fn update_transform(&mut self, parent: &Self) {
    let trs = Mat4::from_scale_rotation_translation(
      self.scale.extend(1.0),
      Quat::from_rotation_z(self.rotation),
      self.position.extend(0.0),
    );

    self.local_to_world = parent.local_to_world * trs;
  }
}

/// Helper methods for building [`SceneNode`]s with [`Transform2D`]s.
impl<'a> SceneNodeBuilder<'a, Transform2D> {
  /// Sets the position of the node.
  pub fn with_position(mut self, position: Vec2) -> Self {
    self.transform.position = position;
    self
  }

  /// Sets the rotation of the node.
  pub fn with_rotation(mut self, rotation: f32) -> Self {
    self.transform.rotation = rotation;
    self
  }

  /// Sets the scale of the node.
  pub fn with_scale(mut self, scale: Vec2) -> Self {
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
  local_to_world: Mat4,
}

impl Transform for Transform3D {
  fn update_transform(&mut self, parent: &Self) {
    let trs = Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position);

    self.local_to_world = parent.local_to_world * trs;
  }
}

/// Helper methods for building [`SceneNode`]s with [`Transform3D`]s.
impl<'a> SceneNodeBuilder<'a, Transform3D> {
  /// Sets the position of the node.
  pub fn with_position(mut self, position: Vec3) -> Self {
    self.transform.position = position;
    self
  }

  /// Sets the rotation of the node.
  pub fn with_rotation(mut self, rotation: Quat) -> Self {
    self.transform.rotation = rotation;
    self
  }

  /// Sets the scale of the node.
  pub fn with_scale(mut self, scale: Vec3) -> Self {
    self.transform.scale = scale;
    self
  }
}
