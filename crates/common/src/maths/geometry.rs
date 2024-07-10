//! Helpers for working with geometry

use super::{DVec2, DVec3, Vec2, Vec3, Vector};

/// A triangle in a vector space V.
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Triangle<V: Vector> {
  pub a: V,
  pub b: V,
  pub c: V,
}

/// A 2D triangle.
pub type Triangle2 = Triangle<Vec2>;

/// A 2D triangle with double precision vertices.
pub type DTriangle2 = Triangle<DVec2>;

/// A 3D triangle.
pub type Triangle3 = Triangle<Vec3>;

/// A 3D triangle with double precision vertices.
pub type DTriangle3 = Triangle<DVec3>;

/// A polygon in a vector space V.
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polygon<V: Vector> {
  pub vertices: Vec<V>,
}

impl<V: Vector> Polygon<V> {
  /// Creates a new polygon from a set of vertices.
  pub fn from_vertices(vertices: &[V]) -> Self {
    Self {
      vertices: Vec::from(vertices),
    }
  }

  /// Creates a new polygon from a set of triangles.
  pub fn from_triangles(triangles: &[Triangle<V>]) -> Self {
    let mut vertices = Vec::new();

    for triangle in triangles {
      vertices.push(triangle.a);
      vertices.push(triangle.b);
      vertices.push(triangle.c);
    }

    Self { vertices }
  }
}

/// A 2D polygon.
pub type Polygon2 = Polygon<Vec2>;

/// A 2D polygon with double precision vertices.
pub type DPolygon2 = Polygon<DVec2>;

/// A 3D polygon.
pub type Polygon3 = Polygon<Vec3>;

/// A 3D polygon with double precision vertices.
pub type DPolygon3 = Polygon<DVec3>;
