use std::alloc::Allocator;

use common::{Array, Camera, Frustum, Sphere, AABB};

use super::*;

/// Represents an object that can be culled from the scene.
pub trait CullableObject {
  /// Gets the bounds of the object.
  fn compute_bounds(&self) -> ObjectBounds;
}

/// Represents a scene that can be culled.
pub trait CullableScene {
  /// Culls the objects that are visible to the given camera.
  fn cull_visible_objects<T>(&self, camera: &dyn Camera) -> VisibleObjectSet<T>;
}

/// Possible bounds of an object.
pub enum ObjectBounds {
  AABB(AABB),
  Sphere(Sphere),
}

impl ObjectBounds {
  /// Determines if the bounds are visible to the given frustum.
  pub fn is_visible_to_frustum(&self, frustum: &Frustum) -> bool {
    match self {
      ObjectBounds::AABB(aabb) => frustum.intersects_aabb(aabb),
      ObjectBounds::Sphere(sphere) => frustum.intersects_sphere(sphere),
    }
  }
}

/// A set of visible objects that can be rendered in a scene.
///
/// This is a subset of the objects in a scene that are visible to a specific
/// camera, and can be used to optimize rendering by only rendering the objects
/// that are visible to the camera.
pub struct VisibleObjectSet<'a, T: ?Sized> {
  /// The objects that are visible to the camera.
  array: Array<'a, VisibleObject<'a, T>>,
}

/// A visible object that can be rendered in a scene.
struct VisibleObject<'a, T: ?Sized> {
  object: &'a T,
  bounds: ObjectBounds,
  sorting_key: Option<MaterialSortingKey>,
}

impl<'a, T: ?Sized + CullableObject + RenderObject + 'static> VisibleObjectSet<'a, T> {
  /// Creates a new set of visible objects from the iterator.
  pub fn from_iter(frustum: &Frustum, objects: impl IntoIterator<Item = &'a T>) -> Self {
    Self::from_iter_in(frustum, &std::alloc::Global, objects)
  }

  /// Creates a new set of visible objects from the iterator in the allocator.
  pub fn from_iter_in(
    frustum: &Frustum,
    allocator: &'a dyn Allocator,
    objects: impl IntoIterator<Item = &'a T>,
  ) -> Self {
    let mut iter = objects.into_iter();
    let (capacity, _) = iter.size_hint();
    let mut array = Array::with_capacity_in(allocator, capacity);

    while let Some(object) = iter.next() {
      let bounds = object.compute_bounds();

      // is this object visible to the camera?
      if bounds.is_visible_to_frustum(frustum) {
        array.push(VisibleObject {
          object,
          bounds,
          sorting_key: object.material().map(MaterialSortingKey::for_material),
        });
      }
    }

    Self { array }
  }
}

impl<'a, T: ?Sized> IntoIterator for &'a VisibleObjectSet<'a, T> {
  type Item = &'a T;
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.array.iter().map(|it| &it.object).copied()
  }
}

/// A key that can be used to uniquely identify the kind of material.
///
/// This is used to sort materials into batches for efficient rendering,
/// minimizing state changes between draw calls.
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct MaterialSortingKey(u64);

impl MaterialSortingKey {
  /// Gets the sorting key for the given material.
  ///
  /// A sorting key is defined as a 64-bit integer, where the first 32 bits
  /// represent the flags that indicate the required state of the graphics
  /// pipeline, and the last 32 bits represent the ID of the shader that should
  /// be used to render the material.
  pub fn for_material(material: &Material) -> Self {
    let shader = material.shader();
    let flags = material.flags();

    let flags = u64::from(flags.bits());
    let shader = u64::from(shader.id());

    Self(flags << 32 | shader)
  }
}

bitflags::bitflags! {
  /// Flags that indicate the required state of the graphics pipeline for a material.
  #[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
  pub struct MaterialFlags: u32 {
    const ALPHA_TESTING = 0b00000001;
    const DEPTH_TESTING = 0b00000010;
    const STENCIL_TESTING = 0b00000100;
  }
}
