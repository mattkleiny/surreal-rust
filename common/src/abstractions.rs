//! Shared abstractions for different modules of the engine.
//!
//! These abstractions allow 'interop' between disparate crates that otherwise
//! wouldn't know about each other. For example, the `graphics` module doesn't
//! know about the `scene` module, but graphics components can be present in a
//! scene.

/// Represents a component in a scene.
///
/// This is a high-level abstraction that allows callbacks in response to common
/// scene lifecycle events, independent of how those events are triggered.
#[allow(unused_variables)]
pub trait SceneLifecycle {
  fn on_scene_start(&mut self) {}
  fn on_scene_stop(&mut self) {}
  fn on_scene_pause(&mut self) {}
  fn on_scene_resume(&mut self) {}
  fn on_scene_update(&mut self, delta_time: f32) {}
  fn on_scene_render(&mut self) {}
}
