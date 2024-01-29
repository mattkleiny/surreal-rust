//! Helpers for working with geometry

pub use csg::*;

mod csg;

// TODO: implement geometry helpers like ear cutting and triangulation

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

// Common triangle types
pub type Triangle2 = Triangle<Vec2>;
pub type DTriangle2 = Triangle<DVec2>;
pub type Triangle3 = Triangle<Vec3>;
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

// Common polygon types
pub type Polygon2 = Polygon<Vec2>;
pub type DPolygon2 = Polygon<DVec2>;
pub type Polygon3 = Polygon<Vec3>;
pub type DPolygon3 = Polygon<DVec3>;
