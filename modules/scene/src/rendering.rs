use graphics::RenderScene;

use super::*;

/// Allows arbitrary scene graphs to be rendered.
impl<'a, T: Transform> RenderScene for SceneGraph<'a, T> {
  fn cameras(&self) -> Vec<&dyn graphics::RenderCamera> {
    todo!()
  }
}
