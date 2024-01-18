//! Helpers for working with geometry

pub use convexhull::*;
pub use delaunay::*;
pub use earclipping::*;

mod convexhull;
mod delaunay;
mod earclipping;

use super::{DVec2, DVec3, Vec2, Vec3};

/// A triangle in a vector space V.
#[derive(Clone, Debug)]
pub struct Triangle<V = Vec2> {
  pub a: V,
  pub b: V,
  pub c: V,
}

pub type Triangle2 = Triangle<Vec2>;
pub type DTriangle2 = Triangle<DVec2>;
pub type Triangle3 = Triangle<Vec3>;
pub type DTriangle3 = Triangle<DVec3>;

/// A polygon in a vector space V.
#[derive(Clone, Debug)]
pub struct Polygon<V = Vec2> {
  pub vertices: Vec<V>,
}

pub type Polygon2 = Polygon<Vec2>;
pub type DPolygon2 = Polygon<DVec2>;
pub type Polygon3 = Polygon<Vec3>;
pub type DPolygon3 = Polygon<DVec3>;
