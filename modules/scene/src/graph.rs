use std::fmt::{Debug, Formatter};

use bitflags::bitflags;
use common::{Camera, FastHashMap, FastHashSet, FromRandom, Frustum};

use super::*;

mod rendering;

common::impl_arena_index!(SceneNodeId, "Identifies a node in a scene graph.");

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

/// A possible error that can occur when working with a [`SceneGraph`].
#[derive(Debug)]
pub enum SceneError {
  CannotFindNode,
  CannotReparentNodeToSelf,
}

/// A graph of [`SceneNode`]s that represent a scene in space.
///
/// Graphs are composed of [`SceneNode`]s in a recursive tree structure. Each
/// node provides transform information to the graph for use in rendering and
/// logic.
///
/// Notifications are sent down the graph via the [`SceneEvent`] type, which can
/// be used to inform recursive operations on the graph and it's children.
#[derive(Default)]
pub struct SceneGraph<'a, T: Transform = ()> {
  pub root: SceneNode<'a, T>,
  groups: FastHashMap<String, FastHashSet<SceneNodeId>>,
  listeners: Vec<Box<dyn common::SceneListener>>,
}

impl<'a, T: Transform> common::Scene for SceneGraph<'a, T> {
  fn add_listener(&mut self, listener: Box<dyn common::SceneListener>) {
    self.listeners.push(listener);
  }

  fn remove_listener(&mut self, listener: Box<dyn common::SceneListener>) {
    self.listeners.retain(|x| !std::ptr::eq(x.as_ref(), listener.as_ref()));
  }
}

impl<'a, T: Transform> SceneGraph<'a, T> {
  /// Creates a new [`SceneGraph`] with the given root [`SceneNode`].
  pub fn new(root: impl Into<SceneNode<'a, T>>) -> Self {
    Self {
      root: root.into(),
      groups: FastHashMap::default(),
      listeners: Vec::default(),
    }
  }

  /// Updates the scene with a single time-step.
  pub fn update(&mut self, delta_time: f32) {
    self.root.notify(&mut SceneEvent::Update(delta_time));
  }

  /// Draws the scene to the given [`Renderer`].
  pub fn render(&mut self, renderer: &mut graphics::Renderer) {
    self.root.notify(&mut SceneEvent::Render(renderer));
  }

  /// Adds a [`SceneNode`] to a [`SceneGroup`], or creates the group anew.
  pub fn add_to_group(&mut self, name: impl Into<String>, node_id: SceneNodeId) {
    let name = name.into();
    let group = self.groups.entry(name).or_default();

    group.insert(node_id);
  }

  /// Removes a [`SceneNode`] from a [`SceneGroup`].
  pub fn remove_from_group(&mut self, name: impl Into<String>, node_id: SceneNodeId) {
    let name = name.into();

    if let Some(group) = self.groups.get_mut(&name) {
      group.retain(|id| *id != node_id);

      if group.is_empty() {
        self.groups.remove(&name);
      }
    }
  }

  /// Re-parents a [`SceneNode`] to a new parent.
  pub fn reparent_node(&mut self, node_to_move_id: SceneNodeId, new_parent_id: SceneNodeId) -> Result<(), SceneError> {
    let node_to_move = self
      .root
      .take_node_by_id(node_to_move_id)
      .ok_or(SceneError::CannotFindNode)?;

    let new_parent = self
      .root
      .find_by_id_mut(new_parent_id)
      .ok_or(SceneError::CannotFindNode)?;

    if node_to_move.children.iter().any(|node| node.id == new_parent_id) {
      return Err(SceneError::CannotReparentNodeToSelf);
    }

    new_parent.children.push(node_to_move);
    new_parent.update_child_transforms();

    Ok(())
  }

  /// Destroys a [`SceneNode`] and all of it's children.
  pub fn delete_node(&mut self, node_to_delete_id: SceneNodeId) -> Result<(), SceneError> {
    let mut node_to_delete = self
      .root
      .take_node_by_id(node_to_delete_id)
      .ok_or(SceneError::CannotFindNode)?;

    node_to_delete.notify(&mut SceneEvent::Destroy);
    drop(node_to_delete);

    Ok(())
  }

  /// Iterates all [`SceneNode`]s in the scene graph.
  pub fn nodes(&'a self) -> impl Iterator<Item = &SceneNode<'a, T>> {
    self.root.iter_recursive().map(|(node, _)| node)
  }
}

impl<'a, T: Transform> Drop for SceneGraph<'a, T> {
  fn drop(&mut self) {
    self.root.notify(&mut SceneEvent::Destroy);
  }
}

/// A node in a [`SceneGraph`].
///
/// A node is a sub-tree of [`SceneNode`]s that represent a scene in a
/// [`SceneGraph`]. Each node can contain one or more [`SceneComponent`]s to
/// build up logic from pieces.
///
/// The transform type, `T`, is used to represent the position of the node
/// relative to it's parent, but the type of transform is generic across the
/// scene graph. This allows for 2D and 3D scenes to be represented using the
/// same types.
///
/// See the [`SceneTransform`] type for more information.
pub struct SceneNode<'a, T: Transform> {
  id: SceneNodeId,
  name: Option<String>,
  flags: NodeFlags,
  is_visible: bool,
  is_enabled: bool,
  is_transform_dirty: bool,
  layer_id: LayerId,
  tags: TagSet<'a>,
  transform: T,
  components: SceneComponentSet,
  children: Vec<SceneNode<'a, T>>,
}

impl<'a, T: Transform> Default for SceneNode<'a, T> {
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
      transform: T::default(),
      components: SceneComponentSet::default(),
      children: Vec::with_capacity(0),
    }
  }
}

impl<'a, T: Transform> SceneNode<'a, T> {
  /// Gets the ID of this [`SceneNode`].
  pub fn id(&self) -> SceneNodeId {
    self.id
  }

  /// Returns `true` if this [`SceneNode`] has been awakened.
  pub fn is_awake(&self) -> bool {
    self.flags.contains(NodeFlags::AWAKE)
  }

  /// Returns `true` if this [`SceneNode`] has been started.
  pub fn is_started(&self) -> bool {
    self.flags.contains(NodeFlags::STARTED)
  }

  /// Gets the name of this [`SceneNode`].
  pub fn name(&self) -> Option<&str> {
    self.name.as_deref()
  }

  /// Sets the name of this [`SceneNode`].
  pub fn set_name(&mut self, name: impl Into<String>) {
    self.name = Some(name.into());
  }

  /// Returns `true` if this [`SceneNode`] is visible.
  pub fn is_visible(&self) -> bool {
    self.is_visible
  }

  /// Returns `true` if this [`SceneNode`] is visible to the given frustum.
  ///
  /// We do this by asking all components if they're visible, and if any of them
  /// are, we're visible.
  pub fn is_visible_to(&self, frustum: &Frustum) -> bool {
    if !self.is_visible() {
      return false;
    }

    // if any of our components are visible, we're visible
    for component in &self.components {
      if component.is_visible_to(frustum) {
        return true;
      }
    }

    false
  }

  /// Sets whether or not this [`SceneNode`] is visible.
  pub fn set_visible(&mut self, visible: bool) {
    self.is_visible = visible;
  }

  /// Sets whether or not this [`SceneNode`] is visible, recursively.
  pub fn set_visible_recursive(&mut self, visible: bool) {
    self.set_visible(visible);

    for child in &mut self.children {
      child.set_visible_recursive(visible);
    }
  }

  /// Returns `true` if this [`SceneNode`] is enabled.
  pub fn is_enabled(&self) -> bool {
    self.is_enabled && self.flags.contains(NodeFlags::ENABLED)
  }

  /// Returns `true` if this [`SceneNode`] is disabled.
  pub fn is_disabled(&self) -> bool {
    !self.is_enabled()
  }

  /// Sets whether or not this [`SceneNode`] is enabled.
  pub fn set_enabled(&mut self, enabled: bool) {
    self.is_enabled = enabled;
  }

  /// Sets whether or not this [`SceneNode`] is enabled, recursively.
  pub fn set_enabled_recursive(&mut self, enabled: bool) {
    self.is_enabled = enabled;

    for child in &mut self.children {
      child.set_enabled_recursive(enabled);
    }
  }

  /// Gets the layer that this [`SceneNode`] is on.
  pub fn layer(&self) -> LayerId {
    self.layer_id
  }

  /// Sets the layer that this [`SceneNode`] is on.
  pub fn set_layer(&mut self, layer_id: LayerId) {
    self.layer_id = layer_id;
  }

  /// Sets the layer that this [`SceneNode`] is on, recursively.
  pub fn set_layer_recursive(&mut self, layer_id: LayerId) {
    self.layer_id = layer_id;

    for child in &mut self.children {
      child.set_layer_recursive(layer_id);
    }
  }

  /// Gets the tags that are applied to this [`SceneNode`].
  pub fn tags(&self) -> impl Iterator<Item = &Tag> {
    self.tags.iter()
  }

  /// Determines if this [`SceneNode`] has the given tag.
  pub fn has_tag(&self, tag: impl Into<Tag<'a>>) -> bool {
    self.tags.contains(&tag.into())
  }

  /// Determines if this [`SceneNode`] has the given tag, recursively.
  pub fn has_tag_recursive(&self, tag: impl Into<Tag<'a>> + Copy) -> bool {
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

  /// Adds a tag to this [`SceneNode`].
  pub fn add_tag(&mut self, tag: impl Into<Tag<'a>>) {
    self.tags.insert(tag.into());
  }

  /// Adds a tag to this [`SceneNode`], recursively.
  pub fn add_tag_recursive(&mut self, tag: impl Into<Tag<'a>> + Copy) {
    self.add_tag(tag);

    for child in &mut self.children {
      child.add_tag_recursive(tag);
    }
  }

  /// Removes a tag from this [`SceneNode`].
  pub fn remove_tag(&mut self, tag: impl Into<Tag<'a>>) {
    self.tags.remove(&tag.into());
  }

  /// Removes a tag from this [`SceneNode`], recursively.
  pub fn remove_tag_recursive(&mut self, tag: impl Into<Tag<'a>> + Copy) {
    self.remove_tag(tag);

    for child in &mut self.children {
      child.remove_tag_recursive(tag);
    }
  }

  /// Adds a child [`SceneNode`] to the node.
  pub fn add_child(&mut self, child: SceneNode<'a, T>) {
    self.children.push(child);
  }

  /// Adds a new [`SceneComponent`] to the node.
  pub fn add_component<C: SceneComponent + 'static>(&mut self, component: C) {
    self.components.add(component);
  }

  /// Gets the [`SceneComponent`] of the given type in this node.
  pub fn get_component<C: SceneComponent + 'static>(&self) -> Option<&C> {
    self.components.get()
  }

  /// Mutably gets the [`SceneComponent`] of the given type in this node.
  pub fn get_component_mut<C: SceneComponent + 'static>(&mut self) -> Option<&mut C> {
    self.components.get_mut()
  }

  /// Removes the [`SceneComponent`] of the given type from this node.
  pub fn remove_component<C: SceneComponent + 'static>(&mut self) -> Option<()> {
    self.components.remove::<C>()
  }

  /// Tries to locate the node with the given [`SceneNodeId`] in this hierarchy.
  pub fn find_by_id(&self, node_id: SceneNodeId) -> Option<&SceneNode<'a, T>> {
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
  pub fn find_by_id_mut(&mut self, node_id: SceneNodeId) -> Option<&mut SceneNode<'a, T>> {
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
  pub fn find_by_path(&self, _path: impl Into<NodePath<'a>>) -> Option<&SceneNode<'a, T>> {
    todo!()
  }

  /// Tries to mutably locate a node in this hierarchy by it's [`NodePath`].
  pub fn find_by_path_mut(&mut self, _path: impl Into<NodePath<'a>>) -> Option<&mut SceneNode<'a, T>> {
    todo!()
  }

  /// Iterates all child [`SceneNode`]s of this node.
  pub fn iter(&'a self) -> impl Iterator<Item = &SceneNode<'a, T>> {
    struct Iter<'a, T: Transform> {
      node: &'a SceneNode<'a, T>,
      index: usize,
    }

    impl<'a, T: Transform> Iterator for Iter<'a, T> {
      type Item = &'a SceneNode<'a, T>;

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

  /// Iterates all child [`SceneNode`]s of this node, recursively.
  pub fn iter_recursive(&'a self) -> impl Iterator<Item = (&SceneNode<'a, T>, usize)> {
    struct IterRecursive<'a, T: Transform> {
      stack: Vec<(&'a SceneNode<'a, T>, usize)>,
    }

    impl<'a, T: Transform> Iterator for IterRecursive<'a, T> {
      type Item = (&'a SceneNode<'a, T>, usize);

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

  /// Recursively walks the [`SceneNode`] hierarchy, calling the given callback
  pub fn walk_recursive(&self, mut callback: impl FnMut(&SceneNode<'a, T>) -> bool) {
    if !callback(self) {
      return;
    }

    for child in &self.children {
      child.walk_recursive(&mut callback);
    }
  }

  /// Notifies this node that it has been added to a scene.
  fn on_awake(&mut self) {
    for component in &mut self.components {
      component.on_awake();
    }
  }

  /// Notifies this node that it has been started.
  fn on_start(&mut self) {
    for component in &mut self.components {
      component.on_start();
    }
  }

  /// Notifies this node that it has been enabled.
  fn on_enable(&mut self) {
    for component in &mut self.components {
      component.on_enable();
    }
  }

  /// Notifies this node that it has been disabled.
  fn on_disable(&mut self) {
    for component in &mut self.components {
      component.on_disable();
    }
  }

  /// Notifies this node that it has been destroyed.
  fn on_destroy(&mut self) {
    for component in &mut self.components {
      component.on_destroy();
    }
  }

  /// Notifies this node that it has been added to a scene.
  fn awake_if_necessary(&mut self) {
    if !self.flags.contains(NodeFlags::AWAKE) {
      self.notify_children(&mut SceneEvent::Awake);
      self.on_awake();

      self.flags |= NodeFlags::AWAKE;
    }
  }

  /// Notifies this node that it has been started.
  fn start_if_necessary(&mut self) {
    if !self.flags.contains(NodeFlags::STARTED) {
      self.notify(&mut SceneEvent::Start);
      self.on_start();

      self.flags |= NodeFlags::STARTED;
    }
  }

  /// Notifies this node that it has been enabled.
  fn enable_if_necessary(&mut self) {
    if !self.flags.contains(NodeFlags::ENABLED) {
      self.notify(&mut SceneEvent::Enable);
      self.on_enable();

      self.flags |= NodeFlags::ENABLED;
    }
  }

  /// Notifies this node that it has been disabled.
  fn disable_if_necessary(&mut self) {
    if self.flags.contains(NodeFlags::ENABLED) {
      self.notify(&mut SceneEvent::Disable);
      self.on_disable();

      self.flags.remove(NodeFlags::ENABLED);
    }
  }

  /// Notifies this node that it has been destroyed.
  fn destroy_if_necessary(&mut self) {
    if self.flags.contains(NodeFlags::AWAKE) {
      self.notify(&mut SceneEvent::Destroy);
      self.on_destroy();

      self.flags.remove(NodeFlags::AWAKE);
    }
  }

  /// Tries to locate the [`SceneNode`] with the given [`SceneNodeId`] in this
  /// hierarchy. If the node is found, remove it from it's parent and return
  /// it.
  fn take_node_by_id(&mut self, node_id: SceneNodeId) -> Option<SceneNode<'a, T>> {
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

  /// Updates the transform of this node relative to it's parent.
  fn update_transform(&mut self, parent: &T) {
    self.transform.update_transform(parent);

    self.update_child_transforms();
  }

  /// Updates the transform of all of this node's child [`SceneNode`]s.
  fn update_child_transforms(&mut self) {
    for child in &mut self.children {
      child.update_transform(&self.transform);
    }

    self.is_transform_dirty = false;
  }

  /// Notifies this node's child [`SceneNode`]s of the given [`SceneEvent`].
  fn notify_children(&mut self, event: &mut SceneEvent) {
    for child in &mut self.children {
      child.notify(event);
    }

    for component in &mut self.components {
      component.notify(event);
    }
  }

  /// Notify this node of the given event.
  fn notify(&mut self, event: &mut SceneEvent) {
    match event {
      SceneEvent::Awake => self.awake_if_necessary(),
      SceneEvent::Enable => self.enable_if_necessary(),
      SceneEvent::Disable => self.disable_if_necessary(),
      SceneEvent::Start => self.start_if_necessary(),
      SceneEvent::Destroy => self.destroy_if_necessary(),

      SceneEvent::Update(_) if self.is_enabled => {
        // if our transform is dirty, on the next update we need to notify all children
        if self.is_transform_dirty {
          self.update_child_transforms();
        }

        self.notify_children(event);
      }
      SceneEvent::Render(_) if self.is_visible => {
        self.notify_children(event);
      }
      SceneEvent::TransformChanged => {
        self.notify_children(event);
      }
      _ => {} // discard this event
    }
  }
}

impl<'a, T: Transform> Debug for SceneNode<'a, T> {
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

impl<'a, T: Transform> IntoIterator for &'a SceneNode<'a, T> {
  type Item = &'a SceneNode<'a, T>;
  type IntoIter = impl Iterator<Item = Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

/// A utility builder for [`SceneNode`]s.
#[must_use]
#[derive(Default)]
pub struct SceneNodeBuilder<'a, T: Transform = ()> {
  pub name: Option<String>,
  pub layer_id: LayerId,
  pub tags: TagSet<'a>,
  pub transform: T,
  pub components: SceneComponentSet,
  pub children: Vec<SceneNode<'a, T>>,
}

impl<'a, T: Transform> SceneNodeBuilder<'a, T> {
  /// Sets the name of the [`SceneNode`].
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.name = Some(name.into());
    self
  }

  /// Sets the layer ID of the [`SceneNode`].
  pub fn with_layer_id(mut self, layer_id: LayerId) -> Self {
    self.layer_id = layer_id;
    self
  }

  /// Adds a tag to the [`SceneNode`].
  pub fn with_tag(mut self, tag: impl Into<Tag<'a>>) -> Self {
    self.tags.insert(tag.into());
    self
  }

  /// Adds a component to the [`SceneNode`].
  pub fn with_component(mut self, component: impl SceneComponent + 'static) -> Self {
    self.components.add(component);
    self
  }

  /// Adds a child [`SceneNode`] to the [`SceneNode`].
  pub fn with_child(mut self, child: impl Into<SceneNode<'a, T>>) -> Self {
    self.children.push(child.into());
    self
  }

  /// Builds the resultant [`SceneNode`].
  pub fn build(self) -> SceneNode<'a, T> {
    SceneNode {
      id: SceneNodeId::random(),
      name: self.name,
      is_visible: true,
      is_enabled: true,
      is_transform_dirty: true,
      layer_id: self.layer_id,
      tags: self.tags.clone(),
      transform: self.transform,
      components: self.components,
      children: self.children,
      ..Default::default()
    }
  }
}

impl<'a, T: Transform> From<SceneNodeBuilder<'a, T>> for SceneNode<'a, T> {
  fn from(value: SceneNodeBuilder<'a, T>) -> Self {
    value.build()
  }
}

#[cfg(test)]
mod tests {
  use common::{Scene, SceneListener};

  use super::*;

  #[test]
  fn test_add_listener_to_scene() {
    struct TestListener {}

    impl SceneListener for TestListener {}

    let mut graph = SceneGraph2D::default();

    graph.add_listener(Box::new(TestListener {}));
    graph.add_listener(Box::new(TestListener {}));

    assert_eq!(graph.listeners.len(), 2);
  }
}
