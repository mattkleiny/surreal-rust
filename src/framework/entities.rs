//! A lightweight entity management system.

use specs::prelude::*;

pub use components::*;
pub use systems::*;

use crate::diagnostics::*;

mod components;
mod systems;

/// Resource wrapper for system delta time.
#[derive(Default, Debug)]
pub struct DeltaTime(f32);

/// An entity in an entity manager.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entity {
  inner_entity: specs::Entity,
}

/// Manages the set of all entities in a scene.
pub struct EntityManager {
  // TODO: rethink dispatcher lifetime
  world: World,
  dispatcher: Dispatcher<'static, 'static>,
}

impl EntityManager {
  pub fn new() -> Self {
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().build();

    world.register::<Transform2d>();
    dispatcher.setup(&mut world);

    Self { world, dispatcher }
  }

  /// Creates a new entity.
  pub fn create_entity(&mut self) -> Entity {
    Entity {
      inner_entity: self.world.create_entity().build(),
    }
  }

  /// Deletes an existing entity.
  pub fn delete_entity(&mut self, entity: Entity) {
    if let Err(error) = self.world.delete_entity(entity.inner_entity) {
      error!("An error occurred whilst deleting an entity: {}", error);
    }
  }

  /// Advances the entity manager by a single frame.
  pub fn tick(&mut self, delta_time: f32) {
    self.world.insert(DeltaTime(delta_time));
    self.dispatcher.dispatch(&mut self.world);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Default, Debug)]
  struct TestComponent {
    x: f32,
    y: f32,
    z: f32,
  }

  #[test]
  fn it_should_create_and_delete_entities() {
    let mut manager = EntityManager::new();
    let entity = manager.create_entity();

    manager.tick(16.);
    manager.tick(16.);
    manager.tick(16.);

    manager.delete_entity(entity);
  }
}