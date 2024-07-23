pub use canvas::*;
pub use spatial::*;

mod canvas;
mod spatial;

use common::Guid;

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
  pub node: &'a SceneNode,
}

/// An event that can be sent to a scene node.
pub enum SceneEvent {
  Awake,
  Start,
  Update,
  Render,
  Destroy,
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
  /// Create a new scene node.
  pub fn new() -> Self {
    SceneNode {
      id: None,
      name: None,
      components: Vec::new(),
      children: Vec::new(),
    }
  }

  /// Create a new scene node with a specific ID.
  pub fn with_id(self, node_id: Guid) -> Self {
    SceneNode {
      id: Some(node_id),
      ..self
    }
  }

  /// Create a new scene node with a specific name.
  pub fn with_name(self, name: &str) -> Self {
    SceneNode {
      name: Some(name.to_string()),
      ..self
    }
  }

  /// Add a component to the scene node.
  pub fn with_component(mut self, component: impl SceneComponent + 'static) -> Self {
    self.components.push(Box::new(component));
    self
  }

  /// Adds a new node to the scene graph.
  pub fn add_child(&mut self, node: SceneNode) {
    self.children.push(node);
  }

  /// Find a child node by its ID.
  pub fn find_child_by_id(&self, id: Guid) -> Option<&SceneNode> {
    for child in &self.children {
      if child.id == Some(id) {
        return Some(child);
      }

      if let Some(found) = child.find_child_by_id(id) {
        return Some(found);
      }
    }

    None
  }

  /// Find a child node by its name.
  pub fn find_child_by_name(&self, name: &str) -> Option<&SceneNode> {
    for child in &self.children {
      if child.name.as_deref() == Some(name) {
        return Some(child);
      }

      if let Some(found) = child.find_child_by_name(name) {
        return Some(found);
      }
    }

    None
  }

  /// Update the scene node.
  pub fn update(&mut self) {
    todo!()
  }
}

impl<'a> SceneContext<'a> {
  /// Get a service from the scene context.
  pub fn resolve<T: ?Sized>(&self) -> Option<&T> {
    todo!()
  }
}
