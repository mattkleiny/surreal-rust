//! Basic bone-mesh skinning support for Surreal

use surreal::{collections::FastHashMap, maths::Mat4};

use crate::{BufferUsage, GraphicsServer, Mesh, Vertex, VertexDescriptor, VertexKind};

/// A single bone in a skeleton.
#[derive(Debug, Clone)]
pub struct Bone {
  /// A unique name for this bone within the skeleton.
  pub name: String,
  /// Reference to the parent bone.
  pub parent: Option<usize>,
  /// The local transform of this bone.
  ///
  /// This is the transform that transforms vertices from the bone's local space
  /// to the space of its parent bone.
  pub transform: Mat4,
  /// The inverse bind matrix for this bone; used to transform vertices.
  ///
  /// This is the matrix that transforms vertices from the bone's local space
  /// to the space of the skeleton.
  pub inverse_bind: Mat4,
}

/// A skeleton is a collection of [`Bone`]s that can be used in a [`Skin`].
#[derive(Default, Debug, Clone)]
pub struct Skeleton {
  bones: Vec<Bone>,
  bones_by_name: FastHashMap<String, usize>,
}

impl Skeleton {
  /// Creates a new empty skeleton.
  pub fn empty() -> Self {
    Self::default()
  }

  /// Creates a new skeleton from a list of bones.
  pub fn from_bones(bones: Vec<Bone>) -> Self {
    let mut skeleton = Self {
      bones: Vec::with_capacity(bones.len()),
      bones_by_name: FastHashMap::with_capacity_and_hasher(bones.len(), Default::default()),
    };

    for bone in bones {
      skeleton.add_bone(bone);
    }

    skeleton
  }

  /// Adds a bone to this skeleton, and returns its index.
  pub fn add_bone(&mut self, bone: Bone) -> usize {
    let index = self.bones.len();
    let name = bone.name.clone();

    self.bones.push(bone);
    self.bones_by_name.insert(name, index);

    index
  }

  /// Tries to find a bone with the given name.
  ///
  /// N.B: The name must be unique, and the search is case-insensitive.
  pub fn find_bone(&self, name: &str) -> Option<usize> {
    self
      .bones
      .iter()
      .position(|bone| bone.name.eq_ignore_ascii_case(name))
  }

  /// Borrows a bone from this skeleton.
  pub fn bone(&self, index: usize) -> Option<&Bone> {
    self.bones.get(index)
  }

  /// Mutably borrows a bone from this skeleton.
  pub fn bone_mut(&mut self, index: usize) -> Option<&mut Bone> {
    self.bones.get_mut(index)
  }

  /// Updates all of the inverse bind matrices for this skeleton.
  ///
  /// Use this to after a change to the skeleton's bone transforms, to ensure
  /// that the inverse bind matrices are up to date.
  pub fn update_bind_matrices(&mut self) {
    let bones = self.bones.clone(); // TODO: remove this clone; might need some unsafe code

    for bone in self.bones.iter_mut() {
      let parent = bone.parent.map(|index| &bones[index]);

      bone.inverse_bind = parent
        .map(|parent| parent.inverse_bind * bone.transform)
        .unwrap_or(bone.transform);
    }
  }
}

/// A single keyframe for a [`Bone`] in an [`Animation`].
#[derive(Debug, Clone)]
pub struct Keyframe {
  /// The normalised time of this keyframe, in seconds.
  pub normalised_time: f32,
  /// The local transform of the bone at this keyframe.
  pub transform: Mat4,
}

impl Default for Keyframe {
  fn default() -> Self {
    Self {
      normalised_time: 0.0,
      transform: Mat4::IDENTITY,
    }
  }
}

/// A single animation for a [`SkinnedMesh`].
#[derive(Default, Debug, Clone)]
pub struct Animation {
  /// The total duration of this animation, in seconds.
  pub duration: f32,
  /// The individual keyframes of this animation, in chronological order.
  pub keyframes: Vec<Keyframe>,
}

impl Animation {
  /// Creates a new empty animation.
  pub fn empty() -> Self {
    Self::default()
  }

  /// Creates a new animation from a list of keyframes.
  pub fn from_keyframes(duration: f32, keyframes: Vec<Keyframe>) -> Self {
    let mut animation = Self {
      duration,
      keyframes,
    };

    animation.sort_keyframes();
    animation
  }

  /// Adds a new keyframe to this animation.
  ///
  /// N.B: The keyframe will be inserted into the animation in the correct
  /// order, so we'll need to sort the keyframes after adding a new one.
  pub fn add_keyframe(&mut self, keyframe: Keyframe) {
    self.keyframes.push(keyframe);
    self.sort_keyframes()
  }

  /// Sort keyframes by their normalised time.
  fn sort_keyframes(&mut self) {
    self
      .keyframes
      .sort_by(|a, b| a.normalised_time.partial_cmp(&b.normalised_time).unwrap());
  }
}

/// A skin is a [`Skeleton`] and it's [`Animation`]s.
///
/// It can be used to animate a [`SkinnedMesh`].
#[derive(Default, Debug, Clone)]
pub struct Skin {
  skeleton: Skeleton,
  animations: Vec<Animation>,
}

/// A skinned mesh is a custom type of mesh with a [`Skin`].
///
/// The [`Bone`]s of the skeleton define a linear hierarchy of bones that can be
/// used to blend vertex positions in the underlying vertex shader.
#[derive(Clone)]
pub struct SkinnedMesh {
  mesh: Mesh<SkinVertex>,
  skin: Skin,
}

impl SkinnedMesh {
  /// Creates a new skinned mesh.
  pub fn new(server: &GraphicsServer) -> surreal::Result<Self> {
    Ok(Self {
      mesh: Mesh::new(server, BufferUsage::Dynamic)?,
      skin: Skin::default(),
    })
  }
}

/// A single vertex in a [`SkinnedMesh`].
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SkinVertex {
  position: [f32; 3],
  normal: [f32; 3],
  uv: [f32; 2],
  bone_indices: [u32; 4],
  bone_weights: [f32; 4],
}

impl Vertex for SkinVertex {
  #[rustfmt::skip]
  const DESCRIPTORS: &'static [VertexDescriptor] = &[
    VertexDescriptor { count: 3, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 3, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 2, kind: VertexKind::F32, should_normalize: false },
    VertexDescriptor { count: 4, kind: VertexKind::U32, should_normalize: false },
    VertexDescriptor { count: 4, kind: VertexKind::F32, should_normalize: false },
  ];
}
