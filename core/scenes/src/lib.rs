//! A scene graph system for managing game objects and components.
//!
//! The scene graph system is a way to organize game objects and components in a
//! tree-like structure. Each node in the tree can have a name, an ID, and a
//! list of components attached to it.

pub use canvas::*;
pub use shared::*;
pub use spatial::*;
pub use templates::*;

mod canvas;
mod shared;
mod spatial;
mod templates;

use common::{unsafe_mutable_alias, Guid, ServiceProvider};

/// An event that can be sent to a scene node.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SceneEvent {
  Awake,
  Start,
  Update,
  Render,
  Destroy,
}

/// A node in the scene graph.
#[derive(Default)]
pub struct SceneNode {
  id: Option<Guid>,
  name: Option<String>,
  children: Vec<SceneNode>,
  components: Vec<Box<dyn SceneComponent>>,
}

/// Context for scene events.
pub struct SceneContext<'a> {
  /// A reference to the scene node.
  pub node: &'a SceneNode,
  /// A service provider for the entire scene.
  pub services: &'a ServiceProvider,
}

/// A component that can be attached to a [`SceneNode`].
#[allow(unused_variables)]
pub trait SceneComponent {
  fn on_awake(&mut self, context: &SceneContext) {}
  fn on_start(&mut self, context: &SceneContext) {}
  fn on_update(&mut self, context: &SceneContext) {}
  fn on_render(&mut self, context: &SceneContext) {}
  fn on_destroy(&mut self, context: &SceneContext) {}

  /// Handle an event from the scene tree.
  fn on_event(&mut self, context: &SceneContext, event: SceneEvent) {
    match event {
      SceneEvent::Awake => self.on_awake(context),
      SceneEvent::Start => self.on_start(context),
      SceneEvent::Update => self.on_update(context),
      SceneEvent::Render => self.on_render(context),
      SceneEvent::Destroy => self.on_destroy(context),
    }
  }
}

impl SceneNode {
  /// Create a new [`SceneNode`].
  pub fn new() -> Self {
    SceneNode {
      id: None,
      name: None,
      components: Vec::new(),
      children: Vec::new(),
    }
  }

  /// Create a new [`SceneNode`] with a specific ID.
  pub fn with_id(self, node_id: Guid) -> Self {
    SceneNode {
      id: Some(node_id),
      ..self
    }
  }

  /// Create a new [`SceneNode`] with a specific name.
  pub fn with_name(self, name: &str) -> Self {
    SceneNode {
      name: Some(name.to_string()),
      ..self
    }
  }

  /// Add a component to the [`SceneNode`].
  pub fn with_component(mut self, component: impl SceneComponent + 'static) -> Self {
    self.components.push(Box::new(component));
    self
  }

  /// Adds a child node to the current node.
  pub fn add_child(&mut self, node: SceneNode) {
    self.children.push(node);
  }

  /// Removes a child node from the current node.
  pub fn remove_child(&mut self, node: &SceneNode) {
    self.children.retain(|child| child.id != node.id);
  }

  /// Clears all children from the current node.
  pub fn clear_children(&mut self) {
    self.children.clear();
  }

  /// Sends an event to the scene node and all its children.
  pub fn notify(&mut self, event: SceneEvent, services: &ServiceProvider) {
    let context = unsafe {
      SceneContext {
        node: unsafe_mutable_alias(self),
        services,
      }
    };

    // notify components first
    for component in self.components.iter_mut() {
      component.on_event(&context, event);
    }

    // notify children
    for child in self.children.iter_mut() {
      child.notify(event, services);
    }
  }
}

#[cfg(test)]
mod tests {
  use common::FromRandom;

  use super::*;

  #[derive(Default)]
  struct TestComponent {}

  impl SceneComponent for TestComponent {
    fn on_awake(&mut self, context: &SceneContext) {
      println!(
        "I'm awake! Node id = {:?}, Node name = {:?}",
        context.node.id, context.node.name
      );
    }
  }

  #[test]
  fn test_scene_node_construction() {
    let node = SceneNode::new()
      .with_id(Guid::random())
      .with_name("Root")
      .with_component(TestComponent::default())
      .with_component(TestComponent::default());

    assert_eq!(node.id.is_some(), true);
    assert_eq!(node.name.is_some(), true);
    assert_eq!(node.components.len(), 2);
  }

  #[test]
  fn test_scene_node_notification() {
    let services = ServiceProvider::default();

    let mut root = SceneNode::new()
      .with_id(Guid::random())
      .with_name("Root")
      .with_component(TestComponent::default());

    let child = SceneNode::new()
      .with_id(Guid::random())
      .with_name("Child")
      .with_component(TestComponent::default());

    root.add_child(child);

    root.notify(SceneEvent::Awake, &services);
    root.notify(SceneEvent::Start, &services);
    root.notify(SceneEvent::Update, &services);
    root.notify(SceneEvent::Render, &services);
    root.notify(SceneEvent::Destroy, &services);
  }
}
