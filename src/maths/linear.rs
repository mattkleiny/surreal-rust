// TODO: implement a frustum?

pub type Point2<T> = cgmath::Point2<T>;
pub type Point3<T> = cgmath::Point3<T>;

pub type Vector2<T> = cgmath::Vector2<T>;
pub type Vector3<T> = cgmath::Vector3<T>;
pub type Vector4<T> = cgmath::Vector4<T>;

pub type Matrix2<T> = cgmath::Matrix2<T>;
pub type Matrix3<T> = cgmath::Matrix3<T>;
pub type Matrix4<T> = cgmath::Matrix4<T>;

pub type Euler<T> = cgmath::Euler<T>;
pub type Quaternion<T> = cgmath::Quaternion<T>;

/// Represents a plane in 3-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Plane<T> {
  distance: T,
  normal: Vector2<T>,
}

/// Represents a ray into 2-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ray2<T> {
  origin: Vector2<T>,
  direction: Vector2<T>,
}

impl<T> Ray2<T> {
  pub fn new(origin: Vector2<T>, direction: Vector2<T>) -> Self {
    Self { origin, direction }
  }
}

/// Represents a ray into 3-space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ray3<T> {
  origin: Vector3<T>,
  direction: Vector3<T>,
}

impl<T> Ray3<T> {
  pub fn new(origin: Vector3<T>, direction: Vector3<T>) -> Self {
    Self { origin, direction }
  }
}

/// Represents a half-space in 3d; usually results from a plane split of the space.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum HalfSpace {
  Behind,
  Inline,
  Front,
}

pub const fn vec2<T>(x: T, y: T) -> Vector2<T> { Vector2 { x, y } }
pub const fn vec3<T>(x: T, y: T, z: T) -> Vector3<T> { Vector3 { x, y, z } }
pub const fn point2<T>(x: T, y: T) -> Point2<T> { Point2 { x, y } }
pub const fn point3<T>(x: T, y: T, z: T) -> Point3<T> { Point3 { x, y, z } }
