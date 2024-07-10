use super::*;

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

/// A set of visible objects that can be rendered in a scene.
///
/// This is a subset of the objects in a scene that are visible to a specific
/// camera, and can be used to optimize rendering by only rendering the objects
/// that are visible to the camera.
pub struct VisibleObjectSet<'a> {
  /// The objects that are visible to the camera.
  objects: Vec<VisibleObject<'a>>,
}

/// Represents an object that is visible to a camera.
///
/// This is a reference to an object in a scene, along with the material that
/// should be used to render it.
///
/// This is used to sort objects into batches for efficient rendering,
/// minimizing state changes between draw calls.
pub struct VisibleObject<'a> {
  /// The object itself.
  pub object: &'a dyn RenderObject,
  /// The material of the object.
  pub material: &'a Material,
}

impl<'a> VisibleObjectSet<'a> {
  /// An empty set of objects.
  pub const EMPTY: VisibleObjectSet<'static> = VisibleObjectSet { objects: Vec::new() };

  /// Creates a new set of visible objects from the given objects.
  pub fn new(mut objects: Vec<VisibleObject<'a>>) -> Self {
    objects.sort_by(|a, b| {
      let a = MaterialSortingKey::for_material(a.material);
      let b = MaterialSortingKey::for_material(b.material);

      a.cmp(&b)
    });

    Self { objects }
  }

  /// Groups the objects by material sorting key.
  ///
  /// The flags parameter can be used to filter the materials that are included
  /// in the result. Only materials that have all the specified flags will be
  /// included in the result.
  pub fn group_by_material(&self, flags: MaterialFlags) -> impl Iterator<Item = (&'a Material, &[VisibleObject<'a>])> {
    self
      .objects
      .chunk_by(|a, b| {
        let a = MaterialSortingKey::for_material(a.material);
        let b = MaterialSortingKey::for_material(b.material);

        a == b
      })
      .filter(move |it| flags.contains(it[0].material.flags()))
      .map(|it| (it[0].material, it))
  }
}
