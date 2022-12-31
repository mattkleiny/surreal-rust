//! Mathematical utilities for linear algebra.

pub use glam::{
  ivec2, ivec3, ivec4, swizzles::*, uvec2, uvec3, uvec4, vec2, vec3, vec4, Affine2, Affine3A, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat,
  UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};

pub use aabb::*;
pub use planes::*;
pub use rays::*;

mod aabb;
mod planes;
mod rays;
