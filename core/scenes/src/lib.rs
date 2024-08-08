//! A scene system for managing game objects and components.

pub use canvas::*;
pub use spatial::*;

mod canvas;
mod spatial;

use common::Camera;
use graphics::{RenderFrame, RenderPass, RenderQueue, RenderScene};

pub struct AwakeEvent;
pub struct StartEvent;
pub struct UpdateEvent;
pub struct DestroyEvent;

pub struct DrawEvent<'a> {
  queue: &'a mut RenderQueue,
}

pub trait Event {}

impl Event for AwakeEvent {}
impl Event for StartEvent {}
impl Event for UpdateEvent {}
impl Event for DestroyEvent {}

impl<'a> Event for DrawEvent<'a> {}

pub trait Component {
  fn on_event(&self, event: &dyn Event);
}

pub trait System {
  fn on_event(&self, event: &dyn Event);
}

pub trait IntoSystem {
  fn into_system(self) -> Box<dyn System>;
}

/// A scene in the world.
#[derive(Default)]
pub struct Scene {
  entities: Vec<Entity>,
  systems: Vec<Box<dyn System>>,
}

/// An entity in a [`Scene`].
struct Entity {
  components: Vec<Box<dyn Component>>,
}

impl Scene {
  /// Emits an event into the scene graph.
  pub fn emit(&self, event: &mut dyn Event) {
    // propagate the event to all systems
    for system in &self.systems {
      system.on_event(event);
    }
  }
}

pub struct SceneRenderPass {}

impl RenderScene for Scene {
  fn cameras(&self) -> Vec<&Self::Camera> {
    vec![]
  }
}

impl RenderPass<Scene> for SceneRenderPass {
  fn render_camera(&mut self, scene: &Scene, _camera: &dyn Camera, frame: &mut RenderFrame<'_>) {
    scene.emit(&mut DrawEvent { queue: frame.queue });
  }
}
