use common::maths::{Quat, Vec2, Vec3};

/// Represents a transform used in the hierarchy of a [`SceneGraph`].
///
/// This trait is used by [`SceneNode`]s and [`SceneComponent`]s to convey
/// position, orientation, and scale information to the scene graph.
pub trait SceneTransform: Default + Sized {}

/// A transform in 2D space.
#[derive(Default)]
pub struct Transform2D {
  position: Vec2,
  rotation: f32,
  scale: Vec2,
}

impl SceneTransform for Transform2D {}

/// A transform in 3D space.
#[derive(Default)]
pub struct Transform3D {
  position: Vec3,
  rotation: Quat,
  scale: Vec3,
}

impl SceneTransform for Transform3D {}
