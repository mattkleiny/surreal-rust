use std::collections::{HashMap, HashSet};

use crate::maths::{FromRandom, Matrix4x4, Quaternion, Vector3};
use crate::utilities::unsafe_mutable_alias;

/// A unique identifier for a [`SceneNode`].
pub type SceneNodeId = crate::maths::Guid;

/// The ID of the layer that a [`SceneNode`] inhabits.
pub type LayerId = u16;

/// A list of one or more [`Tag`]s.
pub type TagSet = HashSet<Tag>;

/// A tag that can be applied to a [`SceneNode`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

impl<'a> From<&'a str> for Tag {
  fn from(value: &'a str) -> Self {
    Self(value.to_string())
  }
}

/// A graph of [`SceneNode`]s that represent a scene in space.
///
/// Graphs are composed of [`SceneNode`]s in a recursive tree structure. Each
/// node provides transform information to the graph for use in rendering and logic.
///
/// Notifications are sent down the graph via the [`SceneEvent`] type, which can be
/// used to inform recursive operations on the graph and it's children.
#[derive(Default)]
pub struct SceneGraph {
  /// The root [`SceneNode`] of this graph.
  pub root: SceneNode,

  /// Groups of [`SceneNode`]s by name.
  groups: HashMap<String, SceneGroup>,
}

/// A grouping of nodes in a [`SceneGraph`].
#[derive(Default)]
struct SceneGroup {
  members: HashSet<SceneNodeId>,
}

/// A node in a [`SceneGraph`].
///
/// A node is a sub-tree of [`SceneNode`]s that represent a scene in a [`SceneGraph`].
/// Each node can contain one or more [`Component`]s to build up logic from pieces.
///
/// A node has a position, orientation, and scale relative to its parent node.
pub struct SceneNode {
  id: SceneNodeId,
  is_visible: bool,
  is_enabled: bool,
  is_transform_dirty: bool,
  layer_id: LayerId,
  tags: TagSet,
  transform: SceneNodeTransform,
  components: Vec<Box<dyn Component>>,
  children: Vec<SceneNode>,
}

/// A transform for use in [`SceneNode`] positioning.
#[derive(Clone, Debug)]
pub struct SceneNodeTransform {
  pub local_position: Vector3<f32>,
  pub local_rotation: Quaternion<f32>,
  pub local_scale: Vector3<f32>,
  pub global_position: Vector3<f32>,
  pub global_rotation: Quaternion<f32>,
  pub global_scale: Vector3<f32>,
}

/// A notification for some event that occurred in the scene.
#[derive(Copy, Clone, Debug)]
pub enum SceneEvent<'a> {
  Awake,
  Start,
  Enable,
  Disable,
  Update(f32),
  Destroy,
  TransformChanged(&'a SceneNodeTransform),
}

/// Represents a component in a scene.
///
/// Components receive callbacks in response to scene lifecycle events, and
/// can access information from their parent [`SceneNode`]s.
pub trait Component<N = SceneNode> {
  /// Invoked to handle dispatch of [`SceneEvent`]s.
  fn on_event(&mut self, node: &mut N, event: SceneEvent) {
    match event {
      SceneEvent::Awake => self.on_awake(node),
      SceneEvent::Start => self.on_start(node),
      SceneEvent::Enable => self.on_enable(node),
      SceneEvent::Disable => self.on_disable(node),
      SceneEvent::Update(delta_time) => self.on_update(node, delta_time),
      SceneEvent::Destroy => self.on_destroy(node),
      _ => {}
    }
  }

  fn on_awake(&mut self, _node: &mut N) {}
  fn on_start(&mut self, _node: &mut N) {}
  fn on_enable(&mut self, _node: &mut N) {}
  fn on_disable(&mut self, _node: &mut N) {}
  fn on_update(&mut self, _node: &mut N, _delta_time: f32) {}
  fn on_destroy(&mut self, _node: &mut N) {}
}

impl Default for SceneNode {
  fn default() -> Self {
    Self {
      id: SceneNodeId::random(),
      is_visible: true,
      is_enabled: true,
      is_transform_dirty: true,
      layer_id: 0,
      tags: HashSet::new(),
      transform: Default::default(),
      components: Vec::with_capacity(0),
      children: Vec::with_capacity(0),
    }
  }
}

impl Default for SceneNodeTransform {
  fn default() -> Self {
    SceneNodeTransform::IDENTITY
  }
}

impl SceneNodeTransform {
  pub const IDENTITY: Self = Self {
    local_position: Vector3::ZERO,
    local_rotation: Quaternion::IDENTITY,
    local_scale: Vector3::ONE,
    global_position: Vector3::ZERO,
    global_rotation: Quaternion::IDENTITY,
    global_scale: Vector3::ONE,
  };

  pub fn local_to_world(&self) -> Matrix4x4<f32> {
    todo!()
  }

  pub fn world_to_local(&self) -> Matrix4x4<f32> {
    todo!()
  }

  pub fn recalculate(&mut self, _other: &SceneNodeTransform) {
    // TODO: implement me
  }
}

impl SceneGraph {
  /// Notifies all nodes in the scene graph of a [`SceneEvent`].
  pub fn notify(&mut self, event: SceneEvent) {
    self.root.notify(event);
  }

  /// Adds a [`SceneNode`] to a group.
  pub fn add_to_group(&mut self, name: impl Into<String>, node_id: SceneNodeId) {
    let name = name.into();
    let group = self.groups.entry(name).or_default();

    group.members.insert(node_id);
  }

  /// Removes a [`SceneNode`] from a group.
  pub fn remove_from_group(&mut self, name: impl Into<String>, node_id: SceneNodeId) {
    let name = name.into();

    if let Some(group) = self.groups.get_mut(&name) {
      group.members.retain(|id| *id != node_id);

      if group.members.is_empty() {
        self.groups.remove(&name);
      }
    }
  }
}

impl SceneNode {
  pub fn id(&self) -> SceneNodeId {
    self.id
  }

  pub fn is_visible(&self) -> bool {
    self.is_visible
  }

  pub fn set_visible(&mut self, visible: bool) {
    self.is_visible = visible;
  }

  pub fn set_visible_recursive(&mut self, visible: bool) {
    self.set_visible(visible);

    for child in self.children.iter_mut() {
      child.set_visible_recursive(visible);
    }
  }

  pub fn is_enabled(&self) -> bool {
    self.is_enabled
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.is_enabled = enabled;
  }

  pub fn set_enabled_recursive(&mut self, enabled: bool) {
    self.is_enabled = enabled;

    for child in &mut self.children {
      child.set_enabled_recursive(enabled);
    }
  }

  pub fn layer(&self) -> LayerId {
    self.layer_id
  }

  pub fn set_layer(&mut self, layer_id: LayerId) {
    self.layer_id = layer_id;
  }

  pub fn set_layer_recursive(&mut self, layer_id: LayerId) {
    self.layer_id = layer_id;

    for child in &mut self.children {
      child.set_layer_recursive(layer_id);
    }
  }

  pub fn tags(&self) -> impl Iterator<Item = &Tag> {
    self.tags.iter()
  }

  pub fn has_tag(&self, tag: impl Into<Tag>) -> bool {
    self.tags.contains(&tag.into())
  }

  pub fn has_tag_recursive(&self, tag: impl Into<Tag> + Copy) -> bool {
    if self.has_tag(tag) {
      return true;
    }

    for child in &self.children {
      if child.has_tag_recursive(tag) {
        return true;
      }
    }

    false
  }

  pub fn add_tag(&mut self, tag: impl Into<Tag>) {
    self.tags.insert(tag.into());
  }

  pub fn add_tag_recursive(&mut self, tag: impl Into<Tag> + Copy) {
    self.add_tag(tag);

    for child in &mut self.children {
      child.add_tag_recursive(tag);
    }
  }

  pub fn remove_tag(&mut self, tag: impl Into<Tag>) {
    self.tags.remove(&tag.into());
  }

  pub fn remove_tag_recursive(&mut self, tag: impl Into<Tag> + Copy) {
    self.remove_tag(tag);

    for child in &mut self.children {
      child.remove_tag_recursive(tag);
    }
  }

  pub fn local_position(&self) -> Vector3<f32> {
    self.transform.local_position
  }

  pub fn set_local_position(&mut self, position: Vector3<f32>) {
    self.transform.local_position = position;
    self.is_transform_dirty = true;
  }

  pub fn global_position(&self) -> Vector3<f32> {
    self.transform.global_position
  }

  pub fn set_global_position(&mut self, position: Vector3<f32>) {
    self.transform.global_position = position;
  }

  pub fn local_rotation(&self) -> Quaternion<f32> {
    self.transform.local_rotation
  }

  pub fn set_local_rotation(&mut self, rotation: Quaternion<f32>) {
    self.transform.local_rotation = rotation;
    self.is_transform_dirty = true;
  }

  pub fn global_rotation(&self) -> Quaternion<f32> {
    self.transform.global_rotation
  }

  pub fn set_global_rotation(&mut self, rotation: Quaternion<f32>) {
    self.transform.global_rotation = rotation;
  }

  pub fn local_scale(&self) -> Vector3<f32> {
    self.transform.local_scale
  }

  pub fn set_local_scale(&mut self, scale: Vector3<f32>) {
    self.transform.local_scale = scale;
    self.is_transform_dirty = true;
  }

  pub fn global_scale(&self) -> Vector3<f32> {
    self.transform.global_scale
  }

  pub fn set_global_scale(&mut self, scale: Vector3<f32>) {
    self.transform.global_scale = scale;
  }

  /// Adds a child [`SceneNode`] to the node.
  pub fn add_child(&mut self, child: SceneNode) {
    self.children.push(child);
  }

  /// Adds a new [`Component`] to the node.
  pub fn add_component(&mut self, component: Box<dyn Component>) {
    self.components.push(component);
  }

  /// Notify this node's [`Component`] and all of it's child [`SceneNode`]s.
  pub fn notify(&mut self, event: SceneEvent) {
    let node = unsafe_mutable_alias(self);

    for component in &mut self.components {
      component.on_event(node, event);
    }

    match event {
      SceneEvent::Update(_) => {
        if self.is_transform_dirty {
          // propagate our local transform changes to children
          self.update_child_transforms();
        }
        self.notify_children(event);
      }
      // propagate transform information down the hierarchy when parent updates
      SceneEvent::TransformChanged(parent_transform) => {
        self.transform.recalculate(&parent_transform);
        self.update_child_transforms();
      }
      _ => self.notify_children(event),
    }
  }

  /// Updates the transform of all of this node's child [`SceneNode`]s.
  fn update_child_transforms(&mut self) {
    let node = unsafe_mutable_alias(self);

    self.is_transform_dirty = false;
    node.notify_children(SceneEvent::TransformChanged(&self.transform));
  }

  /// Notifies this node's child [`SceneNode`]s of the given [`SceneEvent`].
  fn notify_children(&mut self, event: SceneEvent) {
    for child in &mut self.children {
      child.notify(event);
    }
  }

  /// Iterates all child [`SceneNode`]s of this node.
  pub fn children(&self) -> impl Iterator<Item = &SceneNode> {
    /// Allows iteration of the node's children.
    struct ChildrenIter<'a> {
      node: &'a SceneNode,
      index: Option<usize>,
    }

    impl<'a> Iterator for ChildrenIter<'a> {
      type Item = &'a SceneNode;

      fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.index {
          if index < self.node.children.len() {
            self.index = Some(index + 1);
            return Some(&self.node.children[index]);
          }
        }

        None
      }
    }

    ChildrenIter {
      node: self,
      index: Some(0),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::maths::vec3;

  #[test]
  pub fn scene_node_should_iterate_child_nodes() {
    let node = SceneNode {
      children: vec![
        SceneNode::default(),
        SceneNode::default(),
        SceneNode::default(),
        SceneNode::default(),
        SceneNode::default(),
      ],
      ..Default::default()
    };

    for child in node.children() {
      println!("Child: {:?}", child.local_position());
    }
  }

  #[test]
  pub fn scene_node_should_notify_child_nodes() {
    struct TestComponent {}

    impl Component for TestComponent {
      fn on_update(&mut self, node: &mut SceneNode, delta_time: f32) {
        println!("Update node id {} delta_time {}", node.id, delta_time);
      }
    }

    let mut node = SceneNode {
      children: vec![
        SceneNode::default(),
        SceneNode::default(),
        SceneNode::default(),
        SceneNode::default(),
        SceneNode {
          components: vec![Box::new(TestComponent {})],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    node.notify(SceneEvent::Update(0.16));
  }

  #[test]
  fn scene_node_should_propagate_transform_changes() {
    let mut node = SceneNode {
      children: vec![
        SceneNode::default(),
        SceneNode::default(),
        SceneNode::default(),
        SceneNode::default(),
        SceneNode::default(),
      ],
      ..Default::default()
    };

    node.set_local_position(vec3(1.0, 2.0, 3.0));
    node.notify(SceneEvent::Update(0.16));
  }

  #[test]
  fn scene_node_should_manage_tag_lists() {
    let mut node = SceneNode::default();

    node.add_tag("foo");
    node.add_tag("foo");
    node.add_tag("bar");

    assert!(node.has_tag("foo"));
    assert!(node.has_tag("bar"));

    node.remove_tag("foo");

    assert!(!node.has_tag("foo"));
    assert!(node.has_tag("bar"));
  }

  #[test]
  fn scene_graph_should_notify_all_child_nodes_recursively() {
    struct TestComponent {}

    impl Component for TestComponent {
      fn on_update(&mut self, node: &mut SceneNode, delta_time: f32) {
        println!("Update node id {} delta_time {}", node.id, delta_time);
      }
    }

    let mut scene = SceneGraph {
      root: SceneNode {
        components: vec![
          Box::new(TestComponent {}),
          Box::new(TestComponent {}),
          Box::new(TestComponent {}),
          Box::new(TestComponent {}),
        ],
        children: vec![
          SceneNode {
            components: vec![
              Box::new(TestComponent {}),
              Box::new(TestComponent {}),
              Box::new(TestComponent {}),
              Box::new(TestComponent {}),
            ],
            ..Default::default()
          },
          SceneNode {
            components: vec![
              Box::new(TestComponent {}),
              Box::new(TestComponent {}),
              Box::new(TestComponent {}),
              Box::new(TestComponent {}),
            ],
            ..Default::default()
          },
        ],
        ..Default::default()
      },
      groups: HashMap::new(),
    };

    scene.notify(SceneEvent::Update(0.16));
  }
}
