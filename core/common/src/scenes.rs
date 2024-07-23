use crate::Guid;

/// A scene graph that contains all the nodes in the scene.
#[derive(Default)]
pub struct SceneGraph {
  nodes: Vec<SceneNode>,
}

impl SceneGraph {
  /// Adds a new node to the scene graph.
  pub fn add_node(&mut self, node: SceneNode) {
    self.nodes.push(node);
  }
}

/// A node in the scene graph.
#[derive(Default)]
pub struct SceneNode {
  id: Option<Guid>,
  name: Option<String>,
  components: Vec<Box<dyn SceneComponent>>,
}

impl SceneNode {
  /// Create a new scene node.
  pub fn new() -> Self {
    SceneNode {
      id: None,
      name: None,
      components: Vec::new(),
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

  /// Update the scene node.
  pub fn update(&mut self) {
    todo!()
  }
}

/// An event that can be sent to a scene node.
pub enum SceneEvent {
  Awake,
  Start,
  Update,
  Render,
  Destroy,
}

/// A component that can be attached to a scene node.
pub trait SceneComponent {
  fn on_awake(&mut self, _node: &SceneNode) {}
  fn on_start(&mut self, _node: &SceneNode) {}
  fn on_update(&mut self, _node: &SceneNode) {}
  fn on_render(&mut self, _node: &SceneNode) {}
  fn on_destroy(&mut self, _node: &SceneNode) {}

  /// Handle an event from the scene tree.
  fn on_event(&mut self, node: &SceneNode, event: SceneEvent) {
    match event {
      SceneEvent::Awake => self.on_awake(node),
      SceneEvent::Start => self.on_start(node),
      SceneEvent::Update => self.on_update(node),
      SceneEvent::Render => self.on_render(node),
      SceneEvent::Destroy => self.on_destroy(node),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::FromRandom;

  struct SpriteComponent {
    texture: String,
  }

  impl SceneComponent for SpriteComponent {
    fn on_render(&mut self, _node: &SceneNode) {
      todo!()
    }
  }

  #[test]
  fn it_should_create_a_simple_graph() {
    let mut graph = SceneGraph::default();

    graph.add_node(
      SceneNode::default()
        .with_name("Root")
        .with_id(Guid::random())
        .with_component(SpriteComponent {
          texture: "player.png".to_string(),
        }),
    )
  }
}
