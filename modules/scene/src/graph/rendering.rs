//! Rendering support for scene graphs

use graphics::{MaterialSortingKey, RenderObject, RenderScene, Renderer, VisibleObject, VisibleObjectSet};

use super::*;

/// Allows arbitrary scene graphs to be rendered to a render pipeline.
impl<'a, T: Transform> RenderScene for SceneGraph<'a, T> {
  fn cameras(&self) -> Vec<&dyn Camera> {
    todo!()
  }

  fn cull_visible_objects(&self, camera: &dyn Camera) -> VisibleObjectSet<u64> {
    let frustum = camera.frustum();
    let mut objects = Vec::new();

    // walk the tree and find visible objects
    self.root.walk_recursive(|node| {
      if !node.is_visible_to(&frustum) {
        return false;
      }

      objects.push(VisibleObject {
        identifier: node.id().into(),
        material_sort_key: MaterialSortingKey::default(),
      });

      true
    });

    VisibleObjectSet { frustum, objects }
  }

  fn get_object(&self, identifier: u64) -> Option<&dyn RenderObject> {
    self
      .root
      .find_by_id(SceneNodeId::from(identifier))
      .map(|node| node as &dyn RenderObject)
  }
}

impl<'a, T: Transform> Debug for SceneGraph<'a, T> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (node, level) in self.root.iter_recursive() {
      let indent = if level > 0 {
        " ".repeat(level * 2) + "⤷"
      } else {
        " ".repeat(level * 2)
      };

      writeln!(formatter, "{indent}{node:?}")?;
    }

    Ok(())
  }
}

/// Allows arbitrary scene nodes to be rendered to a render pipeline.
impl<'a, T: Transform> RenderObject for SceneNode<'a, T> {
  fn render(&self, renderer: &mut Renderer) {
    for component in &self.components {
      component.on_draw(renderer);
    }
  }
}
