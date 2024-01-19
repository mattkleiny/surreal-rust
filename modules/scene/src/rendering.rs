use common::{Camera, Frustum, OrthographicCamera};
use graphics::{RenderCamera, RenderScene};

use super::*;

/// A component that allows a [`SceneNode`] to be used as a camera in a
/// rendering pipeline.
///
/// The type of camera is specified by the generic type parameter, [`C`],
/// and any camera type is supported provided it can cull visible objects.
pub struct CameraComponent<C: Camera> {
  pub camera: C,
  frustum: Frustum,
}

impl<C: Camera> CameraComponent<C> {
  /// Creates a new [`CameraComponent`] with the given camera.
  pub fn new(camera: C) -> Self {
    Self {
      frustum: camera.frustum(),
      camera,
    }
  }
}

/// Allows a [`CameraComponent`] to be used as a [`SceneComponent`].
impl<C: Camera> SceneComponent for CameraComponent<C> {
  fn on_transform_changed(&mut self) {
    self.frustum = self.camera.frustum();
  }
}

/// Allows a [`CameraComponent`] to be used as a [`RenderCamera`].
impl<C: Camera> RenderCamera for CameraComponent<C> {
  fn cull_visible_objects(&self) -> Vec<&dyn graphics::RenderObject> {
    todo!()
  }
}

/// Allows arbitrary scene graphs to be rendered.
impl<'a, T: Transform> RenderScene for SceneGraph<'a, T> {
  fn cameras(&self) -> Vec<&dyn graphics::RenderCamera> {
    let mut results = Vec::new();

    for (node, _) in self.root.iter_recursive() {
      if let Some(camera) = node.get_component::<CameraComponent<OrthographicCamera>>() {
        results.push(camera as &dyn RenderCamera);
      }

      if let Some(camera) = node.get_component::<CameraComponent<OrthographicCamera>>() {
        results.push(camera as &dyn RenderCamera);
      }
    }

    results
  }
}
