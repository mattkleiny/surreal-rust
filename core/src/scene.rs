//! Scene graph and scene node management.

use std::fmt::{Debug, Formatter};

use anyhow::anyhow;

use crate::{
  collections::{FastHashMap, FastHashSet},
  graphics::Renderer,
  maths::{Affine3A, Quat, Vec3},
  utilities::{unsafe_mutable_alias, Object, ServiceContainer},
};

// TODO: embed hecs and use a mixed model scene graph/ecs?

// A unique identifier for a [`SceneNode`].
crate::impl_guid!(SceneNodeId);

/// The ID of the layer that a [`SceneNode`] inhabits.
pub type LayerId = u16;

/// A set of one or more [`Tag`]s.
pub type TagSet = FastHashSet<Tag>;

/// A tag that can be applied to a [`SceneNode`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

impl<'a> From<&'a str> for Tag {
  fn from(value: &'a str) -> Self {
    Self(value.to_string())
  }
}

/// A notification for some event that occurred in the scene.
#[non_exhaustive]
pub enum SceneEvent<'a> {
  Awake,
  Start,
  Enable,
  Disable,
  Destroy,
  Update(f32),
  Render(&'a mut Renderer),
  TransformChanged(&'a Transform),
}

/// A graph of [`SceneNode`]s that represent a scene in space.
///
/// Graphs are composed of [`SceneNode`]s in a recursive tree structure. Each
/// node provides transform information to the graph for use in rendering and
/// logic.
///
/// Notifications are sent down the graph via the [`SceneEvent`] type, which can
/// be used to inform recursive operations on the graph and it's children.
pub struct SceneGraph {
  pub root: SceneNode,
  services: ServiceContainer,
  groups: FastHashMap<String, SceneGroup>,
}

/// A grouping of nodes in a [`SceneGraph`].
#[derive(Default)]
struct SceneGroup {
  members: FastHashSet<SceneNodeId>,
}

impl SceneGraph {
  /// Creates a new [`SceneGraph`] with the given root [`SceneNode`].
  pub fn new(root: impl Into<SceneNode>) -> Self {
    Self {
      root: root.into(),
      groups: FastHashMap::default(),
      services: ServiceContainer::default(),
    }
  }

  /// Notifies all nodes in the scene graph of a [`SceneEvent`].
  pub fn notify(&mut self, mut event: SceneEvent) {
    self.root.notify(&mut self.services, &mut event);
  }

  /// Adds a [`SceneNode`] to a [`SceneGroup`], or creates the group anew.
  pub fn add_to_group(&mut self, name: impl Into<String>, node_id: SceneNodeId) {
    let name = name.into();
    let group = self.groups.entry(name).or_default();

    group.members.insert(node_id);
  }

  /// Removes a [`SceneNode`] from a [`SceneGroup`].
  pub fn remove_from_group(&mut self, name: impl Into<String>, node_id: SceneNodeId) {
    let name = name.into();

    if let Some(group) = self.groups.get_mut(&name) {
      group.members.retain(|id| *id != node_id);

      if group.members.is_empty() {
        self.groups.remove(&name);
      }
    }
  }

  /// Re-parents a [`SceneNode`] to a new parent.
  pub fn reparent_node(&mut self, node_to_move_id: SceneNodeId, new_parent_id: SceneNodeId) -> crate::Result<()> {
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
    new_parent.update_child_transforms(&mut self.services);

    Ok(())
  }

  /// Destroys a [`SceneNode`] and all of it's children.
  pub fn delete_node(&mut self, node_to_delete_id: SceneNodeId) -> crate::Result<()> {
    let mut node_to_delete = self
      .root
      .take_node_by_id(node_to_delete_id)
      .ok_or(anyhow!("Unable to find node to delete"))?;

    node_to_delete.notify(&mut self.services, &mut SceneEvent::Destroy);
    drop(node_to_delete);

    Ok(())
  }
}

impl Drop for SceneGraph {
  fn drop(&mut self) {
    self.root.notify(&mut self.services, &mut SceneEvent::Destroy);
  }
}

impl Debug for SceneGraph {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    for (node, level) in self.root.iter_recursive() {
      let indent = if level > 0 {
        " ".repeat(level * 2) + "⤷"
      } else {
        " ".repeat(level * 2)
      };

      writeln!(formatter, "{}{:?}", indent, node)?;
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
  /// Creates a [`Mat4`] that takes a point in local space and transforms it to
  /// global space.
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

/// Context for a [`SceneEvent`].
pub struct SceneContext<'a> {
  /// The [`SceneNode`] being updated.
  pub node: &'a mut SceneNode,
  /// The [`ServiceContainer`] for the scene graph.
  pub services: &'a mut ServiceContainer,
}

/// Represents a component in a scene.
///
/// Components receive callbacks in response to scene lifecycle events, and
/// can access information from their parent [`SceneNode`]s.
#[allow(unused_variables)]
pub trait SceneComponent: Object {
  /// Returns a friendly name for this component, for debugging/editor/etc.
  fn name(&self) -> &'static str;

  /// Invoked to handle dispatch of [`SceneEvent`]s.
  fn on_event(&mut self, context: SceneContext, event: &mut SceneEvent) {
    match event {
      SceneEvent::Awake => self.on_awake(context),
      SceneEvent::Start => self.on_start(context),
      SceneEvent::Enable => self.on_enable(context),
      SceneEvent::Disable => self.on_disable(context),
      SceneEvent::Destroy => self.on_destroy(context),
      SceneEvent::Update(delta_time) if context.node.is_enabled() => self.on_update(context, *delta_time),
      SceneEvent::Render(manager) if context.node.is_visible() => self.on_render(context, *manager),
      _ => {}
    }
  }

  fn on_awake(&mut self, context: SceneContext) {}
  fn on_start(&mut self, context: SceneContext) {}
  fn on_enable(&mut self, context: SceneContext) {}
  fn on_disable(&mut self, context: SceneContext) {}
  fn on_destroy(&mut self, context: SceneContext) {}
  fn on_update(&mut self, context: SceneContext, delta_time: f32) {}
  fn on_render(&mut self, context: SceneContext, renderer: &mut Renderer) {}
}

/// A set of [`SceneComponent`]s in a [`SceneNode`].
#[derive(Default)]
pub struct SceneComponentSet {
  // TODO: hierarchical bit mask over ComponentKind
  components: Vec<Box<dyn SceneComponent>>,
}

impl SceneComponentSet {
  /// Builds a [`SceneComponentSet`] from the given array.
  pub fn from_array<const S: usize>(components: [Box<dyn SceneComponent>; S]) -> Self {
    Self {
      components: Vec::from(components),
    }
  }

  /// Adds a new [`SceneComponent`] to the set.
  pub fn push<C: SceneComponent + 'static>(&mut self, component: C) {
    self.components.push(Box::new(component));
  }

  /// Iterates the [`SceneComponent`]s in this set.
  pub fn iter(&self) -> impl Iterator<Item = &Box<dyn SceneComponent>> {
    self.components.iter()
  }

  /// Mutably iterates the [`SceneComponent`]s in this set.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn SceneComponent>> {
    self.components.iter_mut()
  }
}

impl Debug for SceneComponentSet {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_list().entries(self.components.iter().map(|c| c.name())).finish()
  }
}

impl<'a> IntoIterator for &'a SceneComponentSet {
  type Item = &'a Box<dyn SceneComponent>;
  type IntoIter = impl Iterator<Item = &'a Box<dyn SceneComponent>>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut SceneComponentSet {
  type Item = &'a mut Box<dyn SceneComponent>;
  type IntoIter = impl Iterator<Item = &'a mut Box<dyn SceneComponent>>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

/// A node in a [`SceneGraph`].
///
/// A node is a sub-tree of [`SceneNode`]s that represent a scene in a
/// [`SceneGraph`]. Each node can contain one or more [`SceneComponent`]s to
/// build up logic from pieces.
///
/// A node has a position, orientation, and scale relative to its parent node.
pub struct SceneNode {
  id: SceneNodeId,
  name: Option<String>,
  is_visible: bool,
  is_enabled: bool,
  is_transform_dirty: bool,
  layer_id: LayerId,
  tags: TagSet,
  transform: Transform,
  components: SceneComponentSet,
  children: Vec<SceneNode>,
}

impl Default for SceneNode {
  fn default() -> Self {
    Self {
      id: SceneNodeId::random(),
      name: None,
      is_visible: true,
      is_enabled: true,
      is_transform_dirty: true,
      layer_id: 0,
      tags: FastHashSet::default(),
      transform: Transform::default(),
      components: SceneComponentSet::default(),
      children: Vec::with_capacity(0),
    }
  }
}

impl SceneNode {
  pub fn id(&self) -> SceneNodeId {
    self.id
  }

  pub fn name(&self) -> Option<&str> {
    self.name.as_deref()
  }

  pub fn set_name(&mut self, name: impl Into<String>) {
    self.name = Some(name.into());
  }

  pub fn is_visible(&self) -> bool {
    self.is_visible
  }

  pub fn set_visible(&mut self, visible: bool) {
    self.is_visible = visible;
  }

  pub fn set_visible_recursive(&mut self, visible: bool) {
    self.set_visible(visible);

    for child in &mut self.children {
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

  /// Adds a new [`SceneComponent`] to the node.
  pub fn add_component<C: SceneComponent + 'static>(&mut self, component: C) {
    self.components.push(component);
  }

  /// Notify this node's [`SceneComponent`] and all of it's child
  /// [`SceneNode`]s.
  fn notify(&mut self, services: &mut ServiceContainer, event: &mut SceneEvent) {
    let node = unsafe_mutable_alias(self);

    // notify all components
    for component in &mut self.components {
      let context = SceneContext { node, services };

      component.on_event(context, event);
    }

    // propagate to child nodes
    match event {
      SceneEvent::Update(_) => {
        // if our transform is dirty, on the next update we need to notify all children
        if self.is_transform_dirty {
          self.update_child_transforms(services);
        }
        self.notify_children(event, services);
      }
      SceneEvent::TransformChanged(parent_transform) => {
        // propagate transform information down the hierarchy
        self.transform.rebuild(&parent_transform);
        self.update_child_transforms(services);
      }
      _ => self.notify_children(event, services),
    }
  }

  /// Updates the transform of all of this node's child [`SceneNode`]s.
  fn update_child_transforms(&mut self, services: &mut ServiceContainer) {
    let node = unsafe_mutable_alias(self);

    self.is_transform_dirty = false;
    node.notify_children(&mut SceneEvent::TransformChanged(&self.transform), services);
  }

  /// Notifies this node's child [`SceneNode`]s of the given [`SceneEvent`].
  fn notify_children(&mut self, event: &mut SceneEvent, services: &mut ServiceContainer) {
    for child in &mut self.children {
      child.notify(services, event);
    }
  }

  /// Tries to locate the node with the given [`SceneNodeId`] in this hierarchy.
  pub fn find_by_id(&self, node_id: SceneNodeId) -> Option<&SceneNode> {
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

  /// Tries to locate the node with the given [`SceneNodeId`] in this hierarchy.
  pub fn find_by_id_mut(&mut self, node_id: SceneNodeId) -> Option<&mut SceneNode> {
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

  /// Tries to locate a node in this hierarchy by it's [`NodePath`].
  pub fn find_by_path(&self, node_path: impl Into<NodePath>) -> Option<&SceneNode> {
    fn find_recursive<'a>(node: &'a SceneNode, node_path: impl Into<NodePath>) -> Option<&'a SceneNode> {
      if let Some((first, rest)) = node_path.into().split_first() {
        if node.name() == Some(first) {
          for child in &node.children {
            return find_recursive(child, rest);
          }
        }

        return None;
      }

      return Some(node);
    }

    find_recursive(self, node_path)
  }

  /// Tries to mutably locate a node in this hierarchy by it's [`NodePath`].
  pub fn find_by_path_mut(&mut self, node_path: impl Into<NodePath>) -> Option<&mut SceneNode> {
    fn find_recursive<'a>(node: &'a mut SceneNode, node_path: impl Into<NodePath>) -> Option<&'a mut SceneNode> {
      if let Some((first, rest)) = node_path.into().split_first() {
        if node.name() == Some(first) {
          for child in &mut node.children {
            return find_recursive(child, rest);
          }
        }

        return None;
      }

      return Some(node);
    }

    find_recursive(self, node_path)
  }

  /// Tries to locate the [`SceneNode`] with the given [`SceneNodeId`] in this
  /// hierarchy. If the node is found, remove it from it's parent and return
  /// it.
  fn take_node_by_id(&mut self, node_id: SceneNodeId) -> Option<SceneNode> {
    for i in 0..self.children.len() {
      if self.children[i].id == node_id {
        return Some(self.children.remove(i));
      }

      if let Some(node) = self.children[i].take_node_by_id(node_id) {
        return Some(node);
      }
    }

    None
  }

  /// Iterates all child [`SceneNode`]s of this node.
  pub fn iter(&self) -> impl Iterator<Item = &SceneNode> {
    struct Iter<'a> {
      node: &'a SceneNode,
      index: usize,
    }

    impl<'a> Iterator for Iter<'a> {
      type Item = &'a SceneNode;

      fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.node.children.len() {
          self.index += 1;
          return Some(&self.node.children[self.index]);
        }

        None
      }

      fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.node.children.len() - self.index;
        (remaining, Some(remaining))
      }
    }

    Iter { node: self, index: 0 }
  }

  /// Mutably iterates all child [`SceneNode`]s of this node.
  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SceneNode> {
    struct IterMut<'a> {
      node: &'a mut SceneNode,
      index: usize,
    }

    impl<'a> Iterator for IterMut<'a> {
      type Item = &'a mut SceneNode;

      fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.node.children.len() {
          self.index += 1;
          let item = &mut self.node.children[self.index];

          return Some(unsafe_mutable_alias(item));
        }

        None
      }

      fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.node.children.len() - self.index;
        (remaining, Some(remaining))
      }
    }

    IterMut { node: self, index: 0 }
  }

  /// Iterates all child [`SceneNode`]s of this node, recursively.
  pub fn iter_recursive(&self) -> impl Iterator<Item = (&SceneNode, usize)> {
    struct IterRecursive<'a> {
      stack: Vec<(&'a SceneNode, usize)>,
    }

    impl<'a> Iterator for IterRecursive<'a> {
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

    IterRecursive { stack: vec![(self, 0)] }
  }
}

impl Debug for SceneNode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SceneNode")
      .field("id", &self.id)
      .field("name", &self.name)
      .field("layer", &self.layer_id)
      .field("tags", &self.tags)
      .field("components", &self.components)
      .finish()
  }
}

impl<'a> IntoIterator for &'a SceneNode {
  type Item = &'a SceneNode;
  type IntoIter = impl Iterator<Item = &'a SceneNode>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut SceneNode {
  type Item = &'a mut SceneNode;
  type IntoIter = impl Iterator<Item = &'a mut SceneNode>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

/// A path to a [`SceneNode`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodePath<'a>(&'a str);

impl<'a> NodePath<'a> {
  /// Splits the path into it's first component and the rest of the path as two
  /// pieces.
  pub fn split_first(&self) -> Option<(&'a str, &'a str)> {
    let mut split = self.0.splitn(2, '/');

    let first = split.next()?;
    let rest = split.next()?;

    Some((first, rest))
  }
}

impl<'a> From<&'a str> for NodePath<'a> {
  fn from(value: &'a str) -> Self {
    Self(value)
  }
}

/// A utility builder for [`SceneNode`]s.
#[must_use]
#[derive(Default)]
pub struct SceneNodeBuilder {
  name: Option<String>,
  layer_id: LayerId,
  tags: TagSet,
  transform: Transform,
  components: SceneComponentSet,
  children: Vec<SceneNode>,
}

impl SceneNodeBuilder {
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.name = Some(name.into());
    self
  }

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

  pub fn with_component(mut self, component: impl SceneComponent + 'static) -> Self {
    self.components.push(component);
    self
  }

  pub fn with_child(mut self, child: impl Into<SceneNode>) -> Self {
    self.children.push(child.into());
    self
  }

  /// Builds the resultant [`SceneNode`].
  pub fn build(self) -> SceneNode {
    SceneNode {
      id: SceneNodeId::random(),
      name: self.name,
      is_visible: true,
      is_enabled: true,
      is_transform_dirty: false,
      layer_id: self.layer_id,
      tags: self.tags.clone(),
      transform: self.transform,
      components: self.components,
      children: self.children,
    }
  }
}

impl Into<SceneNode> for SceneNodeBuilder {
  fn into(self) -> SceneNode {
    self.build()
  }
}

#[cfg(test)]
mod tests {
  use macros::Object;

  use super::*;
  use crate as surreal;

  #[test]
  pub fn scene_node_should_iterate_child_nodes() {
    let graph = SceneGraph::new(
      SceneNodeBuilder::default()
        .with_child(SceneNode::default())
        .with_child(SceneNode::default())
        .with_child(SceneNode::default())
        .with_child(SceneNode::default())
        .with_child(SceneNode::default()),
    );

    println!("{:?}", graph);
  }

  #[test]
  pub fn scene_node_should_notify_child_nodes() {
    #[derive(Object)]
    struct TestComponent1;

    impl SceneComponent for TestComponent1 {
      fn name(&self) -> &'static str {
        "TestComponent1"
      }

      fn on_update(&mut self, context: SceneContext, delta_time: f32) {
        println!("Update component 1 on node id {} delta_time {}", context.node.id, delta_time);
      }
    }

    #[derive(Object)]
    struct TestComponent2;

    impl SceneComponent for TestComponent2 {
      fn name(&self) -> &'static str {
        "TestComponent2"
      }

      fn on_update(&mut self, context: SceneContext, delta_time: f32) {
        println!("Update component 2 on node id {} delta_time {}", context.node.id, delta_time);
      }
    }

    let mut graph = SceneGraph::new(
      SceneNodeBuilder::default()
        .with_component(TestComponent1)
        .with_component(TestComponent2),
    );

    graph.notify(SceneEvent::Update(0.16));

    println!("{:?}", graph);
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
  //   assert_eq!(node.children[0].transform.global_position, vec3(2.0, 2.0,
  // 4.0));   assert_eq!(node.children[0].children[0].transform.global_position,
  // vec3(3.0, 2.0, 5.0)); }

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

    println!("{:?}", node);
  }

  #[test]
  fn scene_graph_should_notify_all_child_nodes_recursively() {
    #[derive(Object)]
    struct TestComponent;

    impl SceneComponent for TestComponent {
      fn name(&self) -> &'static str {
        "TestComponent"
      }

      fn on_update(&mut self, context: SceneContext, delta_time: f32) {
        println!("Update node {:?} delta_time {}", context.node, delta_time);
      }
    }

    let mut scene = SceneGraph::new(
      SceneNodeBuilder::default()
        .with_component(TestComponent)
        .with_child(
          SceneNodeBuilder::default()
            .with_component(TestComponent)
            .with_component(TestComponent),
        )
        .with_child(
          SceneNodeBuilder::default()
            .with_component(TestComponent)
            .with_component(TestComponent)
            .with_component(TestComponent),
        ),
    );

    scene.notify(SceneEvent::Update(0.16));

    println!("{:?}", scene);
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

    println!("Before reparent:\n{:?}", scene);

    scene.reparent_node(from_id, to_id).unwrap();

    println!("After reparent:\n{:?}", scene);
  }

  #[test]
  fn scene_graph_should_find_by_node_path() {
    #[rustfmt::skip]
    let scene = SceneGraph::new(
      SceneNodeBuilder::default()
        .with_name("Parent")
        .with_child(
          SceneNodeBuilder::default()
            .with_name("Child1")
            .with_child(
              SceneNodeBuilder::default()
                .with_name("Child2")
                .with_child(SceneNodeBuilder::default()
                  .with_name("Child3")
                )
            ),
        ),
    );

    println!("{:?}", scene);

    let result = scene.root.find_by_path("Parent/Child1/Child2/Child3").unwrap();

    println!("{:?}", result);
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

    println!("Before delete:\n{:?}", scene);

    scene.delete_node(scene.root.children[2].id).unwrap();

    println!("After delete:\n{:?}", scene);
  }
}
