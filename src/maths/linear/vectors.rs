pub use vector2::*;
pub use vector3::*;
pub use vector4::*;

use super::*;

mod vector2;
mod vector3;
mod vector4;

/// Shorthand to construct a [`Vector2`].
pub const fn vec2<T>(x: T, y: T) -> Vector2<T> where T: Numeric {
  Vector2::new(x, y)
}

/// Shorthand to construct a [`Vector3`].
pub const fn vec3<T>(x: T, y: T, z: T) -> Vector3<T> where T: Numeric {
  Vector3::new(x, y, z)
}

/// Shorthand to construct a [`Vector4`].
pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vector4<T> where T: Numeric {
  Vector4::new(x, y, z, w)
}
