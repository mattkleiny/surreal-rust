//! Rendering support for scene graphs

use graphics::{RenderFrame, RenderObject, RenderScene, VisibleObjectSet};

use super::*;

/// Allows arbitrary scene graphs to be rendered to a render pipeline.
impl<'a, T: Transform> RenderScene for SceneGraph<'a, T> {
  fn cameras(&self) -> Vec<&dyn Camera> {
    todo!()
  }

  fn cull_visible_objects(&self, camera: &dyn Camera) -> VisibleObjectSet<'a> {
    let frustum = camera.frustum();
    let objects = Vec::new();

    // walk the tree and find visible objects
    self.root.walk_recursive(|node| {
      if !node.is_visible_to(&frustum) {
        return false;
      }

      // TODO: push the object to the list

      true
    });

    VisibleObjectSet { frustum, objects }
  }
}

/// Allows arbitrary scene nodes to be rendered to a render pipeline.
impl<'a, T: Transform> RenderObject for SceneNode<'a, T> {
  fn render(&self, frame: &mut RenderFrame<'_>) {
    for component in &self.components {
      component.on_draw(frame.renderer);
    }
  }
}
