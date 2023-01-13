//! Mathematical utilities for linear algebra.

pub use aabb::*;
/// Swizzle operations for [`Vec2`].
pub use glam::swizzles::Vec2Swizzles;
/// Swizzle operations for [`Vec3`].
pub use glam::swizzles::Vec3Swizzles;
/// Swizzle operations for [`Vec4`].
pub use glam::swizzles::Vec4Swizzles;
pub use glam::{
  ivec2, ivec3, ivec4, uvec2, uvec3, uvec4, vec2, vec3, vec4, Affine2, Affine3A, EulerRot, IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, Quat,
  UVec2, UVec3, UVec4, Vec2, Vec3, Vec4,
};
pub use planes::*;
pub use rays::*;

mod aabb;
mod planes;
mod rays;
