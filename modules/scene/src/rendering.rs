use graphics::RenderScene;

use super::*;

/// Allows arbitrary scene graphs to be rendered.
impl<'a, T: SceneTransform> RenderScene for super::SceneGraph<'a, T> {
  fn cameras(&self) -> Vec<&dyn graphics::RenderCamera> {
    todo!()
  }
}
