use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use surreal::collections::FastHashSet;

use crate::{LayerId, SceneComponentSet, TagSet};

surreal::impl_guid!(SceneNodeId);

/// A node in a [`SceneGraph`].
///
/// A node is a sub-tree of [`SceneNode`]s that represent a scene in a
/// [`SceneGraph`]. Each node can contain one or more [`SceneComponent`]s to
/// build up logic from pieces.
pub struct SceneNode<'a> {
  id: SceneNodeId,
  name: Option<String>,
  flags: NodeFlags,
  is_visible: bool,
  is_enabled: bool,
  is_transform_dirty: bool,
  layer_id: LayerId,
  tags: TagSet<'a>,
  components: SceneComponentSet,
  children: Vec<SceneNode<'a>>,
}

bitflags! {
  /// Internal flags for a [`SceneNode`], indicating the current state of the
  /// node relative to internal scene graph lifecycle events.
  #[derive(Default)]
  struct NodeFlags: u8 {
    const NONE = 0b00000000;
    const AWAKE = 0b00000001;
    const STARTED = 0b00000010;
    const ENABLED = 0b00000100;
  }
}

impl<'a> Default for SceneNode<'a> {
  fn default() -> Self {
    Self {
      id: SceneNodeId::random(),
      name: None,
      flags: NodeFlags::NONE,
      is_visible: true,
      is_enabled: true,
      is_transform_dirty: true,
      layer_id: 0,
      tags: FastHashSet::default(),
      components: SceneComponentSet::default(),
      children: Vec::with_capacity(0),
    }
  }
}
