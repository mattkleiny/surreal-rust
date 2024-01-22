use bitflags::bitflags;
use common::Frustum;

use super::*;

bitflags! {
  /// Flags that indicate the required state of the graphics pipeline for a material.
  #[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct MaterialSortingKey {
  flags: MaterialFlags,
}

impl From<MaterialFlags> for MaterialSortingKey {
  /// Gets the sorting key for the given material flags.
  fn from(flags: MaterialFlags) -> Self {
    Self { flags }
  }
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
pub struct VisibleObject {
  /// The object that is visible.
  pub object: Box<dyn RenderObject>,
  /// The sorting key for the material of the object.
  pub material_sort_key: MaterialSortingKey,
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

/// Allows iterating over the [`VisibleObject`]s in a [`VisibleObjectSet`] by
/// batching them by [`MaterialSortingKey`].
///
/// This is useful for efficiently rendering the objects in a set by minimizing
/// state changes between draw calls.
pub struct VisibleObjectIterator<'a> {
  visible_objects: Vec<&'a [VisibleObject]>,
  current_index: usize,
}

impl<'a> IntoIterator for &'a VisibleObjectSet {
  type Item = &'a [VisibleObject];
  type IntoIter = VisibleObjectIterator<'a>;

  fn into_iter(self) -> Self::IntoIter {
    let visible_objects = self
      .objects
      .group_by(|a, b| a.material_sort_key == b.material_sort_key)
      .collect();

    VisibleObjectIterator {
      visible_objects,
      current_index: 0,
    }
  }
}

impl<'a> Iterator for VisibleObjectIterator<'a> {
  type Item = &'a [VisibleObject];

  fn next(&mut self) -> Option<Self::Item> {
    if self.current_index >= self.visible_objects.len() {
      return None;
    }

    let index = self.current_index;
    let batch = self.visible_objects[index];

    self.current_index += 1;

    Some(batch)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Default)]
  struct TestObject;

  impl RenderObject for TestObject {
    fn render(&self, _renderer: &mut Renderer) {
      // no-op
    }
  }

  #[test]
  fn test_iteration_over_group_by_key() {
    let objects = VisibleObjectSet {
      frustum: Frustum::default(),
      objects: vec![
        VisibleObject {
          object: Box::new(TestObject::default()),
          material_sort_key: MaterialSortingKey::from(MaterialFlags::ALPHA_BLENDING),
        },
        VisibleObject {
          object: Box::new(TestObject::default()),
          material_sort_key: MaterialSortingKey::from(MaterialFlags::DEPTH_TESTING),
        },
        VisibleObject {
          object: Box::new(TestObject::default()),
          material_sort_key: MaterialSortingKey::from(MaterialFlags::ALPHA_BLENDING),
        },
      ],
    };

    for batch in &objects {
      for object in batch {
        println!("{:?}", object.material_sort_key);
      }
    }
  }
}
