//! Commonly used ECS components.

use crate::framework::AssetRef;
use crate::graphics::{Color, Sprite};
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
pub struct Render {
  pub pivot: Vec2,
  pub offset: Vec2,
  pub color: Color,
  pub sprite: AssetRef<Sprite>,
}

impl Component for Render {
  type Storage = DenseVecStorage<Self>;
}