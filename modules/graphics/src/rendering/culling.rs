use bitflags::bitflags;
use common::Frustum;

use super::*;

bitflags! {
  /// Flags that can be used to control the behavior of a material.
  #[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
  struct MaterialFlags: u32 {
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

/// A key that can be used to uniquely identify the kind of material.
///
/// This is used to sort materials into batches for efficient rendering,
/// minimizing state changes between draw calls.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
struct MaterialSortingKey {
  flags: MaterialFlags,
}

impl From<&Material> for MaterialSortingKey {
  /// Gets the sorting key for the given material.
  fn from(_value: &Material) -> Self {
    todo!()
  }
}

/// Represents an object that is visible to a camera, along with it's material
/// properties that are used to render it.
pub struct VisibleObject {
  object: Box<dyn RenderObject>,
  material_sort_key: MaterialSortingKey,
}

/// A set of visible objects that can be rendered in a scene.
///
/// This is a subset of the objects in a scene that are visible to a specific
/// camera, and can be used to optimize rendering by only rendering the objects
/// that are visible to the camera.
pub struct VisibleObjectSet {
  /// The frustum of the camera that was used to cull the objects.
  pub frustum: Frustum,
  /// The objects that are visible to the camera.
  pub objects: Vec<VisibleObject>,
}

impl<'a> IntoIterator for &'a VisibleObjectSet {
  type Item = &'a VisibleObject;
  type IntoIter = VisibleObjectIterator<'a>;

  fn into_iter(self) -> Self::IntoIter {
    VisibleObjectIterator {
      visible_objects: &self.objects,
      current_index: 0,
    }
  }
}

/// Allows iterating over the [`VisibleObject`]s in a [`VisibleObjectSet`] by
/// batching them by [`MaterialSortingKey`].
///
/// This is useful for efficiently rendering the objects in a set by minimizing
/// state changes between draw calls.
pub struct VisibleObjectIterator<'a> {
  visible_objects: &'a [VisibleObject],
  current_index: usize,
}

impl<'a> Iterator for VisibleObjectIterator<'a> {
  type Item = &'a VisibleObject;

  fn next(&mut self) -> Option<Self::Item> {
    todo!()
  }
}
