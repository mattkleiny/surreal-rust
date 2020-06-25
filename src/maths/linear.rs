pub type Vector2<T> = cgmath::Vector2<T>;
pub type Vector3<T> = cgmath::Vector3<T>;
pub type Vector4<T> = cgmath::Vector4<T>;
pub type Matrix2<T> = cgmath::Matrix2<T>;
pub type Matrix3<T> = cgmath::Matrix3<T>;
pub type Matrix4<T> = cgmath::Matrix4<T>;
pub type Euler<T> = cgmath::Euler<T>;
pub type Quaternion<T> = cgmath::Quaternion<T>;

#[inline]
pub fn vec2<T>(x: T, y: T) -> Vector2<T> {
  Vector2::new(x, y)
}

#[inline]
pub fn vec3<T>(x: T, y: T, z: T) -> Vector3<T> {
  Vector3::new(x, y, z)
}