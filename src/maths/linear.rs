//! Mathematical utilities for linear algebra.

pub use matrices::*;
pub use planes::*;
pub use quaternions::*;
pub use rays::*;
pub use vectors::*;

mod vectors;
mod planes;
mod matrices;
mod rays;
mod quaternions;

/// Shorthand to construct a `Vector2`
#[inline(always)]
pub const fn vec2<T>(x: T, y: T) -> Vector2<T> {
  Vector2 { x, y }
}

/// Shorthand to construct a `Vector3`
#[inline(always)]
pub const fn vec3<T>(x: T, y: T, z: T) -> Vector3<T> {
  Vector3 { x, y, z }
}