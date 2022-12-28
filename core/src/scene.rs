use crate::maths::{FromRandom, Matrix4x4, Quaternion, Vector3};

/// A unique identifier for a [`SceneNode`].
pub type SceneNodeId = crate::maths::Guid;

/// A graph of [`SceneNode`]s that represent a scene in space.
#[derive(Default)]
pub struct SceneGraph {
  /// The root [`SceneNode`] of this graph.
  pub root: SceneNode,
}

/// A node in a [`SceneGraph`].
pub struct SceneNode {
  id: SceneNodeId,
  is_visible: bool,
  is_enabled: bool,
  transform: AffineTransform,
  components: Vec<Box<dyn Component>>,
  children: Vec<SceneNode>,
}

impl Default for SceneNode {
  fn default() -> Self {
    Self {
      id: SceneNodeId::random(),
      is_visible: true,
      is_enabled: true,
      transform: Default::default(),
      components: vec![],
      children: vec![],
    }
  }
}

/// A notification for some event that occurred in the scene.
#[derive(Copy, Clone, Debug)]
pub enum SceneEvent {
  Awake,
  Start,
  Enable,
  Disable,
  Update(f32),
  Destroy,
}

/// Represents a component in a scene.
///
/// Components receive callbacks in response to scene lifecycle events, and
/// can access information from their parent [`SceneNode`]s.
pub trait Component {
  /// Invoked to handle dispatch of [`SceneEvent`]s.
  fn on_event(&mut self, event: SceneEvent) {
    match event {
      SceneEvent::Awake => self.on_awake(),
      SceneEvent::Start => self.on_start(),
      SceneEvent::Enable => self.on_enable(),
      SceneEvent::Disable => self.on_disable(),
      SceneEvent::Update(delta_time) => self.on_update(delta_time),
      SceneEvent::Destroy => self.on_destroy(),
    }
  }

  fn on_awake(&mut self) {}
  fn on_start(&mut self) {}
  fn on_enable(&mut self) {}
  fn on_disable(&mut self) {}
  fn on_update(&mut self, _delta_time: f32) {}
  fn on_destroy(&mut self) {}
}

/// An affine transform for use in [`SceneNode`] positioning.
#[derive(Debug)]
struct AffineTransform {
  pub position: Vector3<f32>,
  pub rotation: Quaternion<f32>,
  pub scale: Vector3<f32>,
}

impl SceneGraph {
  /// Notifies all nodes in the scene graph of a [`SceneEvent`].
  pub fn notify(&mut self, event: SceneEvent) {
    self.root.notify(event);
  }

  pub fn awake(&mut self) {
    self.notify(SceneEvent::Awake);
  }

  pub fn start(&mut self) {
    self.notify(SceneEvent::Start);
  }

  pub fn enable(&mut self) {
    self.notify(SceneEvent::Enable);
  }

  pub fn disable(&mut self) {
    self.notify(SceneEvent::Disable);
  }

  pub fn update(&mut self, delta_time: f32) {
    self.notify(SceneEvent::Update(delta_time));
  }

  pub fn destroy(&mut self) {
    self.notify(SceneEvent::Destroy);
  }
}

impl SceneNode {
  pub fn is_visible(&self) -> bool {
    self.is_visible
  }

  pub fn set_visible(&mut self, visible: bool) {
    self.is_visible = visible;
  }

  pub fn is_enabled(&self) -> bool {
    self.is_enabled
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.is_enabled = enabled;
  }

  pub fn position(&self) -> Vector3<f32> {
    self.transform.position
  }

  pub fn set_position(&mut self, position: Vector3<f32>) {
    self.transform.position = position
  }

  pub fn rotation(&self) -> Quaternion<f32> {
    self.transform.rotation
  }

  pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
    self.transform.rotation = rotation
  }

  pub fn scale(&self) -> Vector3<f32> {
    self.transform.scale
  }

  pub fn set_scale(&mut self, scale: Vector3<f32>) {
    self.transform.scale = scale
  }

  /// Adds a child node to this node.
  pub fn add_child(&mut self, child: SceneNode) {
    self.children.push(child);
  }

  /// Adds a new component to this node.
  pub fn add_component(&mut self, component: Box<dyn Component>) {
    self.components.push(component);
  }

  /// Notify this node's [`Component`] and all of it's child [`SceneNode`]s.
  pub fn notify(&mut self, event: SceneEvent) {
    for component in &mut self.components {
      component.on_event(event);
    }

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
      node: &self,
      index: Some(0),
    }
  }
}

impl Default for AffineTransform {
  fn default() -> Self {
    Self {
      position: Vector3::ZERO,
      rotation: Quaternion::IDENTITY,
      scale: Vector3::ONE,
    }
  }
}

impl AffineTransform {
  /// Converts this transform to a model [`Matrix4x4`].
  pub fn to_matrix4x4(&self) -> Matrix4x4 {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn node_should_iterate_child_nodes() {
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
      println!("Child: {:?}", child.position());
    }
  }

  #[test]
  pub fn node_should_notify_child_nodes() {
    struct TestComponent {}

    impl Component for TestComponent {
      fn on_update(&mut self, delta_time: f32) {
        println!("on_update: {}", delta_time);
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
  fn scene_graph_should_notify_all_child_nodes_recursively() {
    struct TestComponent {}

    impl Component for TestComponent {
      fn on_update(&mut self, delta_time: f32) {
        println!("on_update: {}", delta_time);
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
    };

    scene.notify(SceneEvent::Update(0.16));
  }
}
