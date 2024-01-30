use bitflags::bitflags;
use common::Frustum;

use super::*;

// TODO: flesh out the culling system and add some tests

bitflags! {
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

/// A key that can be used to uniquely identify the kind of material.
///
/// This is used to sort materials into batches for efficient rendering,
/// minimizing state changes between draw calls.
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct MaterialSortingKey {
  flags: MaterialFlags,
}

impl From<&Material> for MaterialSortingKey {
  /// Gets the sorting key for the given material.
  fn from(material: &Material) -> Self {
    let mut flags = MaterialFlags::empty();

    let shader = material.shader();
    let metadata = shader.flags();

    if material.blend_state() != BlendState::Disabled {
      flags.insert(MaterialFlags::ALPHA_BLENDING);
    }

    if material.culling_mode() != CullingMode::Disabled {
      flags.insert(MaterialFlags::BACKFACE_CULLING);
    }

    if material.scissor_mode() != ScissorMode::Disabled {
      flags.insert(MaterialFlags::SCISSOR_TESTING);
    }

    if metadata.contains(ShaderFlags::ALPHA_TESTING) {
      flags.insert(MaterialFlags::ALPHA_TESTING);
    }

    if metadata.contains(ShaderFlags::DEPTH_TESTING) {
      flags.insert(MaterialFlags::DEPTH_TESTING);
    }

    if metadata.contains(ShaderFlags::DEPTH_WRITING) {
      flags.insert(MaterialFlags::DEPTH_WRITING);
    }

    Self { flags }
  }
}

/// Represents an object that is visible to a camera, along with it's material
/// properties that are used to render it.
pub struct VisibleObject<'a, I> {
  /// The identifier of the object.
  pub identifier: I,
  /// The sorting key for the material of the object.
  pub material: &'a Material,
}

/// A set of visible objects that can be rendered in a scene.
///
/// This is a subset of the objects in a scene that are visible to a specific
/// camera, and can be used to optimize rendering by only rendering the objects
/// that are visible to the camera.
pub struct VisibleObjectSet<'a, I> {
  /// The frustum of the camera that was used to cull the objects.
  pub frustum: Frustum,
  /// The objects that are visible to the camera.
  pub objects: Vec<VisibleObject<'a, I>>,
}

impl<'a, I> VisibleObjectSet<'a, I> {
  /// Gets an iterator over the objects in the set.
  pub fn group_by_material(&self) -> impl Iterator<Item = (&'a Material, &[VisibleObject<'a, I>])> {
    self
      .objects
      .chunk_by(|a, b| MaterialSortingKey::from(a.material) == MaterialSortingKey::from(b.material))
      .map(|chunk| (chunk[0].material, chunk))
  }
}
