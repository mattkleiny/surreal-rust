/// Represents a scene that can be loaded by the engine.
pub trait Scene {
  fn add_listener(&mut self, listener: Box<dyn SceneListener>);
  fn remove_listener(&mut self, listener: Box<dyn SceneListener>);
}

/// Represents a listener for scene events.
///
/// This is a high-level abstraction that allows callbacks in response to common
/// scene lifecycle events, independent of how those events are triggered.
#[allow(unused_variables)]
pub trait SceneListener {
  fn on_scene_start(&mut self) {}
  fn on_scene_stop(&mut self) {}
  fn on_scene_pause(&mut self) {}
  fn on_scene_resume(&mut self) {}
  fn on_scene_update(&mut self, delta_time: f32) {}
  fn on_scene_render(&mut self) {}
}

/// Represents a component that can be added to a scene.
pub trait SceneComponent {}

/// Descriptor for how to build a scene.
pub trait SceneDescriptor {
  type Output: Scene;

  /// Builds the scene.
  fn build(&self) -> Self::Output;
}
