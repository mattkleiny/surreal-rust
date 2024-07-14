use common::{Camera, Sphere, AABB};

use super::*;

/// Possible bounds of an object.
pub enum ObjectBounds {
  AABB(AABB),
  Sphere(Sphere),
}

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

/// A set of visible objects that can be rendered in a scene.
///
/// This is a subset of the objects in a scene that are visible to a specific
/// camera, and can be used to optimize rendering by only rendering the objects
/// that are visible to the camera.
pub struct VisibleObjectSet<'a, T> {
  /// The objects that are visible to the camera.
  objects: Vec<&'a T>,
}

impl<'a, T: 'static> VisibleObjectSet<'a, T> {
  /// An empty set of objects.
  pub const EMPTY: VisibleObjectSet<'static, T> = VisibleObjectSet { objects: Vec::new() };

  /// Creates a new set of visible objects from the given objects.
  pub fn new(objects: Vec<&'a T>) -> Self {
    Self { objects }
  }
}

/// A key that can be used to uniquely identify the kind of material.
///
/// This is used to sort materials into batches for efficient rendering,
/// minimizing state changes between draw calls.
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct MaterialSortingKey(u64);

bitflags::bitflags! {
  /// Flags that indicate the required state of the graphics pipeline for a material.
  #[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
  pub struct MaterialFlags: u32 {
    const ALPHA_BLENDING = 0b00000001;
    const ALPHA_TESTING = 0b00000010;
    const BACKFACE_CULLING = 0b00000100;
    const DEPTH_TESTING = 0b00001000;
    const DEPTH_WRITING = 0b00010000;
    const SCISSOR_TESTING = 0b00100000;
    const STENCIL_TESTING = 0b01000000;
    const WIREFRAME = 0b10000000;
  }
}

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
