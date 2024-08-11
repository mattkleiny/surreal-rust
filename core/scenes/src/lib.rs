//! A scene system for managing game objects and components.

use std::any::Any;

pub use canvas::*;
pub use spatial::*;

mod canvas;
mod spatial;

use common::{impl_arena_index, Arena};

impl_arena_index!(EntityId);

pub struct Scene {
  entities: Arena<EntityId, Entity>,
}

impl Scene {
  pub fn new() -> Self {
    Self { entities: Arena::new() }
  }

  pub fn spawn(&mut self) -> EntityId {
    self.entities.insert(Entity { components: Vec::new() })
  }

  pub fn despawn(&mut self, id: EntityId) {
    self.entities.remove(id);
  }

  pub fn add_component<C: Component + 'static>(&mut self, id: EntityId, component: C) {
    if let Some(entity) = self.entities.get_mut(id) {
      entity.components.push(Box::new(component));
    }
  }

  pub fn emit<E>(&mut self, event: &mut E) {
    // ...

    for entity in &mut self.entities {
      for component in &mut entity.components {
        // TODO: use reflection to see if implemented EventListener<E>
      }
    }
  }

  pub fn emit_to<E>(&mut self, _id: EntityId, _event: E) {
    // ...
  }
}

pub struct Entity {
  components: Vec<Box<dyn Component>>,
}

#[allow(unused_variables)]
pub trait Component {
  fn on_attach(&self, node: &Entity) {}
  fn on_detach(&self, node: &Entity) {}
}

pub trait EventListener<E> {
  fn on_event(&self, event: &mut E);
}

pub trait IntoScene {
  fn into_scene(self) -> Scene;
}

pub trait IntoEntity {
  fn into_entity(self) -> Entity;
}

struct Tick;
struct Draw;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_scene_construction() {
    let mut scene = Scene::new();

    let entity1 = scene.spawn();
    let entity2 = scene.spawn();

    scene.add_component(entity1, SpriteComponent {});

    scene.emit(&mut Tick);
  }
}
