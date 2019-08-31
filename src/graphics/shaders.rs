//! GPU shader abstractions.

use glam::{Mat4, Vec2};
use glam::f32::Vec4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShaderKind {
  Vertex,
  Fragment,
}

#[derive(Clone, Copy)]
pub enum UniformData {
  Int(i32),
  Mat4([Mat4; 4]),
  Vec2(Vec2),
  Vec4(Vec4),
  TextureUnit(u32),
}