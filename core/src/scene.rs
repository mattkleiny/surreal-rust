use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

use anyhow::anyhow;

use crate::graphics::RenderContextManager;
use crate::maths::{Affine3A, FromRandom, Quat, Vec3};
use crate::utilities::unsafe_mutable_alias;

/// A unique identifier for a [`SceneNode`].
pub type NodeId = crate::maths::Guid;

/// The ID of the layer that a [`SceneNode`] inhabits.
pub type LayerId = u16;

/// A set of one or more [`Tag`]s.
pub type TagSet = HashSet<Tag>;

/// A tag that can be applied to a [`SceneNode`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

impl<'a> From<&'a str> for Tag {
  fn from(value: &'a str) -> Self {
    Self(value.to_string())
  }
}

/// A notification for some event that occurred in the scene.
pub enum SceneEvent<'a> {
  Awake,
  Start,
  Enable,
  Disable,
  Destroy,
  Update(f32),
  Render(&'a mut RenderContextManager),
  TransformChanged(&'a Transform),
}

/// A graph of [`SceneNode`]s that represent a scene in space.
///
/// Graphs are composed of [`SceneNode`]s in a recursive tree structure. Each
/// node provides transform information to the graph for use in rendering and logic.
///
/// Notifications are sent down the graph via the [`SceneEvent`] type, which can be
/// used to inform recursive operations on the graph and it's children.
pub struct SceneGraph {
  pub root: SceneNode,
  groups: HashMap<String, SceneGroup>,
}

/// A grouping of nodes in a [`SceneGraph`].
#[derive(Default)]
struct SceneGroup {
  members: HashSet<NodeId>,
}

impl SceneGraph {
  /// Creates a new [`SceneGraph`] with the given root [`SceneNode`].
  pub fn new(root: impl Into<SceneNode>) -> Self {
    Self {
      root: root.into(),
      groups: HashMap::new(),
    }
  }

  /// Notifies all nodes in the scene graph of a [`SceneEvent`].
  pub fn notify(&mut self, mut event: SceneEvent) {
    self.root.notify(&mut event);
  }

  /// Adds a [`SceneNode`] to a [`SceneGroup`], or creates the group anew.
  pub fn add_to_group(&mut self, name: impl Into<String>, node_id: NodeId) {
    let name = name.into();
    let group = self.groups.entry(name).or_default();

    group.members.insert(node_id);
  }

  /// Removes a [`SceneNode`] from a [`SceneGroup`].
  pub fn remove_from_group(&mut self, name: impl Into<String>, node_id: NodeId) {
    let name = name.into();

    if let Some(group) = self.groups.get_mut(&name) {
      group.members.retain(|id| *id != node_id);

      if group.members.is_empty() {
        self.groups.remove(&name);
      }
    }
  }

  /// Re-parents a [`SceneNode`] to a new parent.
  pub fn reparent_node(&mut self, node_to_move_id: NodeId, new_parent_id: NodeId) -> crate::Result<()> {
    let node_to_move = self
      .root
      .take_node_by_id(node_to_move_id)
      .ok_or(anyhow!("Unable to locate node to move"))?;

    let new_parent = self
      .root
      .find_by_id_mut(new_parent_id)
      .ok_or(anyhow!("Unable to find target node"))?;

    if node_to_move.children.iter().any(|node| node.id == new_parent_id) {
      return Err(anyhow!("Unable to reparent node to a child of itself"));
    }

    new_parent.children.push(node_to_move);
    new_parent.update_child_transforms();

    Ok(())
  }

  /// Destroys a [`SceneNode`] and all of it's children.
  pub fn delete_node(&mut self, node_to_delete_id: NodeId) -> crate::Result<()> {
    let mut node_to_delete = self
      .root
      .take_node_by_id(node_to_delete_id)
      .ok_or(anyhow!("Unable to find node to delete"))?;

    node_to_delete.notify(&mut SceneEvent::Destroy);
    drop(node_to_delete);

    Ok(())
  }
}

impl Debug for SceneGraph {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    // render a small tree view of the graph
    for (node, level) in self.root.iter_recursive() {
      writeln!(formatter, "{}{:?}", " ".repeat(level * 2), node.id)?;
    }

    Ok(())
  }
}

/// A transform for use in [`SceneNode`] positioning.
#[derive(Clone, Debug)]
pub struct Transform {
  pub local_position: Vec3,
  pub local_rotation: Quat,
  pub local_scale: Vec3,
  pub global_position: Vec3,
  pub global_rotation: Quat,
  pub global_scale: Vec3,
}

impl Default for Transform {
  fn default() -> Self {
    Self {
      local_position: Vec3::ZERO,
      local_rotation: Quat::IDENTITY,
      local_scale: Vec3::ONE,
      global_position: Vec3::ZERO,
      global_rotation: Quat::IDENTITY,
      global_scale: Vec3::ONE,
    }
  }
}

impl Transform {
  /// Creates a [`Mat4`] that takes a point in local space and transforms it to global space.
  pub fn world_to_local(&self) -> Affine3A {
    Affine3A::from_scale_rotation_translation(self.global_scale, self.global_rotation, self.global_position)
  }

  /// Rebuilds this transform from the given other parent [`Transform`].
  pub fn rebuild(&mut self, _parent: &Transform) {
    // TODO: work this out
    // let affine = parent.world_to_local();

    // self.global_position = affine.transform_point3(self.local_position);
    // self.global_rotation = transform * self.local_rotation;
    // self.global_scale = affine.transform_vector3(self.local_scale);
  }
}

/// A flag that indicates what kind of [`Component`]s are present in a [`ComponentSet`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComponentKind {
  /// This component has standard 'update' behaviour, but doesn't need to render.
  Behaviour,
  /// This component needs to render and wants access to the [`RenderContextManager`].
  Renderer,
}

/// Represents a component in a scene.
///
/// Components receive callbacks in response to scene lifecycle events, and
/// can access information from their parent [`SceneNode`]s.
pub trait Component<N = SceneNode> {
  /// Invoked to handle dispatch of [`SceneEvent`]s.
  fn on_event(&mut self, node: &mut N, event: &mut SceneEvent) {
    match event {
      SceneEvent::Awake => self.on_awake(node),
      SceneEvent::Start => self.on_start(node),
      SceneEvent::Enable => self.on_enable(node),
      SceneEvent::Disable => self.on_disable(node),
      SceneEvent::Destroy => self.on_destroy(node),
      SceneEvent::Update(delta_time) => self.on_update(node, *delta_time),
      SceneEvent::Render(manager) => self.on_render(node, *manager),
      _ => {}
    }
  }

  fn on_awake(&mut self, _node: &mut N) {}
  fn on_start(&mut self, _node: &mut N) {}
  fn on_enable(&mut self, _node: &mut N) {}
  fn on_disable(&mut self, _node: &mut N) {}
  fn on_destroy(&mut self, _node: &mut N) {}
  fn on_update(&mut self, _node: &mut N, _delta_time: f32) {}
  fn on_render(&mut self, _node: &mut N, _manager: &mut RenderContextManager) {}

  /// Determines the [`ComponentKind`] of this component.
  ///
  /// The kind is used for determining which sub-trees have component types.
  fn get_kind(&self) -> ComponentKind {
    ComponentKind::Behaviour
  }
}

/// A set of [`Component`]s in a [`SceneNode`].
#[derive(Default)]
pub struct ComponentSet {
  // TODO: hierarchical bit mask over ComponentKind
  components: Vec<Box<dyn Component>>,
}

impl ComponentSet {
  /// Builds a [`ComponentSet`] from the given array.
  pub fn from_array<const S: usize>(components: [Box<dyn Component>; S]) -> Self {
    Self {
      components: Vec::from(components),
    }
  }

  /// Determines if the given [`ComponentKind`] is present in the set.
  pub fn has_kind(&self, kind: ComponentKind) -> bool {
    self.components.iter().any(|c| c.get_kind() == kind)
  }

  /// Adds a new [`Component`] to the set.
  pub fn push<C: Component + 'static>(&mut self, component: C) {
    self.components.push(Box::new(component));
  }

  /// Iterates the [`Component`]s in this set.
  pub fn iter(&self) -> impl Iterator<Item = &Box<dyn Component>> {
    self.components.iter()
  }

  /// Mutably iterates the [`Component`]s in this set.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Component>> {
    self.components.iter_mut()
  }
}

/// A node in a [`SceneGraph`].
///
/// A node is a sub-tree of [`SceneNode`]s that represent a scene in a [`SceneGraph`].
/// Each node can contain one or more [`Component`]s to build up logic from pieces.
///
/// A node has a position, orientation, and scale relative to its parent node.
pub struct SceneNode {
  id: NodeId,
  is_visible: bool,
  is_enabled: bool,
  is_transform_dirty: bool,
  layer_id: LayerId,
  tags: TagSet,
  transform: Transform,
  components: ComponentSet,
  children: Vec<SceneNode>,
}

impl Default for SceneNode {
  fn default() -> Self {
    Self {
      id: NodeId::random(),
      is_visible: true,
      is_enabled: true,
      is_transform_dirty: true,
      layer_id: 0,
      tags: HashSet::new(),
      transform: Default::default(),
      components: ComponentSet::default(),
      children: Vec::with_capacity(0),
    }
  }
}

impl SceneNode {
  pub fn id(&self) -> NodeId {
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

  pub fn local_position(&self) -> Vec3 {
    self.transform.local_position
  }

  pub fn set_local_position(&mut self, position: Vec3) {
    self.transform.local_position = position;
    self.is_transform_dirty = true;
  }

  pub fn global_position(&self) -> Vec3 {
    self.transform.global_position
  }

  pub fn set_global_position(&mut self, position: Vec3) {
    self.transform.global_position = position;
  }

  pub fn local_rotation(&self) -> Quat {
    self.transform.local_rotation
  }

  pub fn set_local_rotation(&mut self, rotation: Quat) {
    self.transform.local_rotation = rotation;
    self.is_transform_dirty = true;
  }

  pub fn global_rotation(&self) -> Quat {
    self.transform.global_rotation
  }

  pub fn set_global_rotation(&mut self, rotation: Quat) {
    self.transform.global_rotation = rotation;
  }

  pub fn local_scale(&self) -> Vec3 {
    self.transform.local_scale
  }

  pub fn set_local_scale(&mut self, scale: Vec3) {
    self.transform.local_scale = scale;
    self.is_transform_dirty = true;
  }

  pub fn global_scale(&self) -> Vec3 {
    self.transform.global_scale
  }

  pub fn set_global_scale(&mut self, scale: Vec3) {
    self.transform.global_scale = scale;
  }

  /// Adds a child [`SceneNode`] to the node.
  pub fn add_child(&mut self, child: SceneNode) {
    self.children.push(child);
  }

  /// Adds a new [`Component`] to the node.
  pub fn add_component<C: Component + 'static>(&mut self, component: C) {
    self.components.push(component);
  }

  /// Notify this node's [`Component`] and all of it's child [`SceneNode`]s.
  fn notify(&mut self, event: &mut SceneEvent) {
    let node = unsafe_mutable_alias(self);

    // notify all components
    for component in &mut self.components.iter_mut() {
      component.on_event(node, event);
    }

    // propagate to child nodes
    match event {
      SceneEvent::Update(_) => {
        // if our transform is dirty, on the next update we need to notify all children
        if self.is_transform_dirty {
          self.update_child_transforms();
        }
        self.notify_children(event);
      }
      SceneEvent::TransformChanged(parent_transform) => {
        // propagate transform information down the hierarchy
        self.transform.rebuild(&parent_transform);
        self.update_child_transforms();
      }
      _ => self.notify_children(event),
    }
  }

  /// Updates the transform of all of this node's child [`SceneNode`]s.
  fn update_child_transforms(&mut self) {
    let node = unsafe_mutable_alias(self);

    self.is_transform_dirty = false;
    node.notify_children(&mut SceneEvent::TransformChanged(&self.transform));
  }

  /// Notifies this node's child [`SceneNode`]s of the given [`SceneEvent`].
  fn notify_children(&mut self, event: &mut SceneEvent) {
    for child in &mut self.children {
      child.notify(event);
    }
  }

  /// Tries to locate the node with the given [`NodeId`] in this hierarchy.
  pub fn find_by_id(&self, node_id: NodeId) -> Option<&SceneNode> {
    if self.id == node_id {
      return Some(self);
    }

    for child in &self.children {
      if let Some(node) = child.find_by_id(node_id) {
        return Some(node);
      }
    }

    None
  }

  /// Tries to locate the node with the given [`NodeId`] in this hierarchy.
  pub fn find_by_id_mut(&mut self, node_id: NodeId) -> Option<&mut SceneNode> {
    if self.id == node_id {
      return Some(self);
    }

    for child in &mut self.children {
      if let Some(node) = child.find_by_id_mut(node_id) {
        return Some(node);
      }
    }

    None
  }

  /// Tries to locate the [`SceneNode`] with the given [`NodeId`] in this hierarchy.
  /// If the node is found, remove it from it's parent and return it.
  fn take_node_by_id(&mut self, node_id: NodeId) -> Option<SceneNode> {
    for i in 0..self.children.len() {
      if self.children[i].id == node_id {
        return Some(self.children.remove(i));
      }
    }

    None
  }

  /// Iterates all child [`SceneNode`]s of this node.
  pub fn iter(&self) -> impl Iterator<Item = &SceneNode> {
    struct DirectIter<'a> {
      node: &'a SceneNode,
      index: Option<usize>,
    }

    impl<'a> Iterator for DirectIter<'a> {
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

    DirectIter {
      node: self,
      index: Some(0),
    }
  }

  /// Iterates all child [`SceneNode`]s of this node, recursively.
  pub fn iter_recursive(&self) -> impl Iterator<Item = (&SceneNode, usize)> {
    struct RecursiveIter<'a> {
      stack: Vec<(&'a SceneNode, usize)>,
    }

    impl<'a> Iterator for RecursiveIter<'a> {
      type Item = (&'a SceneNode, usize);

      fn next(&mut self) -> Option<Self::Item> {
        if let Some((node, level)) = self.stack.pop() {
          for child in &node.children {
            self.stack.push((child, level + 1));
          }

          Some((node, level))
        } else {
          None
        }
      }
    }

    RecursiveIter { stack: vec![(self, 0)] }
  }
}

/// A utility builder for [`SceneNode`]s.
#[derive(Default)]
pub struct SceneNodeBuilder {
  layer_id: LayerId,
  tags: TagSet,
  transform: Transform,
  components: ComponentSet,
  children: Vec<SceneNode>,
}

impl SceneNodeBuilder {
  pub fn with_layer_id(mut self, layer_id: LayerId) -> Self {
    self.layer_id = layer_id;
    self
  }

  pub fn with_tag(mut self, tag: impl Into<Tag>) -> Self {
    self.tags.insert(tag.into());
    self
  }

  pub fn with_local_position(mut self, position: Vec3) -> Self {
    self.transform.local_position = position;
    self
  }

  pub fn with_global_position(mut self, position: Vec3) -> Self {
    self.transform.global_position = position;
    self
  }

  pub fn with_local_rotation(mut self, rotation: Quat) -> Self {
    self.transform.local_rotation = rotation;
    self
  }

  pub fn with_global_rotation(mut self, rotation: Quat) -> Self {
    self.transform.global_rotation = rotation;
    self
  }

  pub fn with_local_scale(mut self, scale: Vec3) -> Self {
    self.transform.local_scale = scale;
    self
  }

  pub fn with_global_scale(mut self, scale: Vec3) -> Self {
    self.transform.global_scale = scale;
    self
  }

  pub fn with_component(mut self, component: impl Component + 'static) -> Self {
    self.components.push(component);
    self
  }

  pub fn with_child(mut self, child: impl Into<SceneNode>) -> Self {
    self.children.push(child.into());
    self
  }

  /// Builds the resultant [`SceneNode`].
  pub fn build(self) -> SceneNode {
    let mut node = SceneNode {
      id: NodeId::random(),
      is_visible: true,
      is_enabled: true,
      is_transform_dirty: false,
      layer_id: self.layer_id,
      tags: self.tags.clone(),
      transform: self.transform,
      components: self.components,
      children: self.children,
    };

    // initial transform propagation
    node.update_child_transforms();
    node
  }
}

impl Into<SceneNode> for SceneNodeBuilder {
  fn into(self) -> SceneNode {
    self.build()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn scene_node_should_iterate_child_nodes() {
    let node = SceneNodeBuilder::default()
      .with_child(SceneNode::default())
      .with_child(SceneNode::default())
      .with_child(SceneNode::default())
      .with_child(SceneNode::default())
      .with_child(SceneNode::default())
      .build();

    for child in node.iter() {
      println!("Child: {:?}", child.local_position());
    }
  }

  #[test]
  pub fn scene_node_should_notify_child_nodes() {
    struct TestComponent1;
    struct TestComponent2;

    impl Component for TestComponent1 {
      fn on_update(&mut self, node: &mut SceneNode, delta_time: f32) {
        println!("Update component 1 on node id {} delta_time {}", node.id, delta_time);
      }
    }

    impl Component for TestComponent2 {
      fn on_update(&mut self, node: &mut SceneNode, delta_time: f32) {
        println!("Update component 2 on node id {} delta_time {}", node.id, delta_time);
      }
    }

    let mut node = SceneNodeBuilder::default()
      .with_component(TestComponent1)
      .with_component(TestComponent2)
      .build();

    node.notify(&mut SceneEvent::Update(0.16));
  }

  // #[test]
  // fn scene_node_should_propagate_transform_changes() {
  //   let node = SceneNodeBuilder::default()
  //     .with_global_position(vec3(1.0, 2.0, 3.0))
  //     .with_child(
  //       SceneNodeBuilder::default()
  //         .with_global_position(vec3(1.0, 0.0, 1.0))
  //         .with_child(
  //           SceneNodeBuilder::default()
  //             .with_global_position(vec3(1.0, 0.0, 1.0))
  //             .with_local_rotation(Quat::IDENTITY)
  //             .build(),
  //         )
  //         .build(),
  //     )
  //     .build();
  //
  //   assert_eq!(node.children[0].transform.global_position, vec3(2.0, 2.0, 4.0));
  //   assert_eq!(node.children[0].children[0].transform.global_position, vec3(3.0, 2.0, 5.0));
  // }

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
    struct TestComponent;

    impl Component for TestComponent {
      fn on_update(&mut self, node: &mut SceneNode, delta_time: f32) {
        println!("Update node id {} delta_time {}", node.id, delta_time);
      }
    }

    let mut scene = SceneGraph::new(
      SceneNodeBuilder::default()
        .with_component(TestComponent)
        .with_component(TestComponent)
        .with_component(TestComponent)
        .with_component(TestComponent)
        .with_component(TestComponent)
        .with_child(
          SceneNodeBuilder::default()
            .with_component(TestComponent)
            .with_component(TestComponent)
            .with_component(TestComponent)
            .with_component(TestComponent),
        )
        .with_child(
          SceneNodeBuilder::default()
            .with_component(TestComponent)
            .with_component(TestComponent)
            .with_component(TestComponent)
            .with_component(TestComponent),
        ),
    );

    scene.notify(SceneEvent::Update(0.16));
  }

  #[test]
  fn scene_graph_should_reparent_nodes() {
    let mut scene = SceneGraph::new(
      SceneNodeBuilder::default()
        .with_child(SceneNodeBuilder::default())
        .with_child(SceneNodeBuilder::default()),
    );

    let from_id = scene.root.children[0].id;
    let to_id = scene.root.children[1].id;

    println!("Before: {:?}", scene);

    scene.reparent_node(from_id, to_id).unwrap();

    println!("After: {:?}", scene);
  }

  #[test]
  fn scene_graph_should_destroy_nodes() {
    let mut scene = SceneGraph::new(
      SceneNodeBuilder::default()
        .with_child(SceneNodeBuilder::default())
        .with_child(SceneNodeBuilder::default())
        .with_child(
          SceneNodeBuilder::default()
            .with_child(SceneNodeBuilder::default())
            .with_child(SceneNodeBuilder::default())
            .with_child(
              SceneNodeBuilder::default()
                .with_child(SceneNodeBuilder::default())
                .with_child(SceneNodeBuilder::default())
                .with_child(SceneNodeBuilder::default()),
            ),
        ),
    );

    println!("Before: {:?}", scene);

    scene.delete_node(scene.root.children[2].id).unwrap();

    println!("After: {:?}", scene);
  }
}
