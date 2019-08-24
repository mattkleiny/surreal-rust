//! Commonly used ECS components.

use crate::maths::{Quat, Vec2};

use super::*;

/// Represents a transformed position in 2-space.
#[derive(Debug)]
pub struct Transform2d {
  pub position: Vec2,
  pub rotation: Quat,
  pub scale: Vec2,
}

impl Default for Transform2d {
  fn default() -> Self {
    Self {
      position: Vec2::new(0., 0.),
      rotation: Quat::identity(),
      scale: Vec2::new(1., 1.),
    }
  }
}

impl Component for Transform2d {
  type Storage = VecStorage<Self>;
}