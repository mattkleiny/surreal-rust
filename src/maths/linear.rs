//! Linear algebra abstractions, mainly using `euclid`.

use euclid::*;

/// A representation of screen space.
pub struct ScreenSpace;

pub type ScreenPoint<T> = Point2D<T, ScreenSpace>;
pub type ScreenSize<T> = Size2D<T, ScreenSpace>;
pub type ScreenRect<T> = Rect<T, ScreenSpace>;

/// A representation of world space.
pub struct WorldSpace;

pub type WorldPoint<T> = Point3D<T, WorldSpace>;
pub type WorldSize<T> = Size3D<T, WorldSpace>;
pub type WorldRect<T> = Rect<T, WorldSpace>;

pub type ProjectionMatrix = Transform3D<f32, WorldSpace, ScreenSpace>;

// Convenience factories
#[inline] pub fn vec2<T : Default>(x: T, y: T) -> WorldPoint<T> { WorldPoint::new(x, y, T::default()) }
#[inline] pub fn vec3<T : Default>(x: T, y: T, z: T) -> WorldPoint<T> { WorldPoint::new(x, y, z) }