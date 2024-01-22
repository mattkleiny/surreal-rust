use common::Camera;
use graphics::{RenderScene, VisibleObjectSet};

use super::*;

/// Allows arbitrary scene graphs to be rendered.
impl<'a, T: Transform> RenderScene for SceneGraph<'a, T> {
  fn cameras(&self) -> Vec<&dyn Camera> {
    todo!()
  }
  fn cull_visible_objects(&self, _camera: &dyn Camera) -> VisibleObjectSet {
    todo!()
  }
}
