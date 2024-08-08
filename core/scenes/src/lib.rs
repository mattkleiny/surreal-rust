//! A scene system for managing game objects and components.

pub use canvas::*;
pub use spatial::*;

mod canvas;
mod spatial;

pub struct AwakeEvent;
pub struct StartEvent;
pub struct UpdateEvent;
pub struct DrawEvent;
pub struct DestroyEvent;

pub trait Event {}

impl Event for AwakeEvent {}
impl Event for StartEvent {}
impl Event for UpdateEvent {}
impl Event for DrawEvent {}
impl Event for DestroyEvent {}

pub trait Component {
  fn on_event(&mut self, event: &dyn Event);
}

pub trait System {
  fn on_event(&mut self, event: &dyn Event);
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
  pub fn emit(&mut self, event: &dyn Event) {
    // propagate the event to all systems
    for system in &mut self.systems {
      system.on_event(event);
    }
  }
}
