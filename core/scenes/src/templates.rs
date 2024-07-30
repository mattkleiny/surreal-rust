//! Templating system for scenes.

use common::{StringName, Variant};

use super::*;

/// Describes a template for a scene node.
#[derive(Debug, Clone)]
pub struct SceneNodeTemplate {
  pub id: Option<Guid>,
  pub name: Option<String>,
  pub components: Vec<SceneComponentTemplate>,
  pub children: Vec<SceneNodeTemplate>,
}

/// A template for a scene component.
#[derive(Debug, Clone)]
pub struct SceneComponentTemplate {
  pub kind: StringName,
  pub properties: Vec<(String, Variant)>,
}

impl SceneNodeTemplate {
  /// Converts the template to a scene node.
  pub fn to_node(&self) -> SceneNode {
    let mut node = SceneNode::default();

    node.id = self.id;
    node.name = self.name.clone();

    for component in &self.components {
      let component = component.to_component();
      node.components.push(component);
    }

    for child in &self.children {
      let child = child.to_node();
      node.children.push(child);
    }

    node
  }
}

impl SceneComponentTemplate {
  /// Converts the template to a component.
  pub fn to_component(&self) -> Box<dyn SceneComponent> {
    // TODO: build the component based on the kind, and set properties as required
    let component: Box<dyn SceneComponent> = match self.kind.to_string().as_str() {
      "Transform" => Box::new(TransformComponent {}),
      "Sprite" => Box::new(SpriteComponent {}),
      _ => panic!("Unknown component type: {}", self.kind),
    };

    component
  }
}

#[cfg(test)]
mod tests {
  use common::{Color32, ToStringName, Variant, Vec2};

  use super::*;

  #[test]
  fn test_read_write_simple_scene() {
    let template = SceneNodeTemplate {
      id: None,
      name: Some("Root".to_string()),
      components: vec![
        SceneComponentTemplate {
          kind: "Transform".to_string_name(),
          properties: vec![
            ("position".to_string(), Variant::Vec2(Vec2::ZERO)),
            ("rotation".to_string(), Variant::F32(0.)),
          ],
        },
        SceneComponentTemplate {
          kind: "Sprite".to_string_name(),
          properties: vec![
            ("texture".to_string(), Variant::String("player.png".to_string())),
            ("tint".to_string(), Variant::Color32(Color32::WHITE)),
          ],
        },
      ],
      children: vec![],
    };

    let node = template.to_node();

    assert_eq!(node.name, Some("Root".to_string()));
  }
}
