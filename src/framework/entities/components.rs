//! Commonly used ECS components.

use crate::graphics::Color;
use crate::maths::{Quat, Vec2};

use super::*;

/// Represents a transformed position in 2-space.
#[derive(Debug)]
pub struct Transform {
  pub position: Vec2,
  pub rotation: Quat,
  pub scale: Vec2,
}

impl Component for Transform {
  type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct SpriteRender {
  pub pivot: Vec2,
  pub offset: Vec2,
  pub color: Color,
}

impl Component for SpriteRender {
  type Storage = DenseVecStorage<Self>;
}