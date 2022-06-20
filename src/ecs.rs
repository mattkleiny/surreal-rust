//! A lightweight single threaded ECS system for use with Surreal.

use crate::collections::{AnyMap, Arena, ArenaIndex};

/// Represents a copmonent that can be attached to an entity.
pub trait Component: Sized + 'static {}

/// Blanket implementation for all possible component types.
impl<C: Sized + 'static> Component for C {}

/// Represents an entity in an `EntityWorld`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Entity(ArenaIndex, *mut EntityWorld);

impl Entity {
  pub fn insert<C: Component>(self, component: C) -> Self {
    let world = unsafe { &mut *self.1 };

    world.insert_component(&self, component);

    self
  }

  /// Retrieves the given component from the entity.
  pub fn get<C: Component>(&self) -> Option<&C> {
    let world = unsafe { &*self.1 };

    world.get_component(self)
  }

  /// Retrieves the given component from the entity.
  pub fn get_unchecked<C: Component>(&self) -> &C {
    let world = unsafe { &*self.1 };

    world.get_component(self).unwrap()
  }

  /// Mutably retrieves the given component from the entity.
  pub fn get_mut<C: Component>(&self) -> Option<&mut C> {
    let world = unsafe { &mut *self.1 };

    world.get_component_mut(self)
  }

  /// Retrieves the given component from the entity.
  pub fn get_mut_unchecked<C: Component>(&self) -> &mut C {
    let world = unsafe { &mut *self.1 };

    world.get_component_mut(self).unwrap()
  }

  /// Retrieves the given component from the entity, creating it if it doesn't exist.
  pub fn get_or_create<C: Component + Default>(&self) -> &mut C {
    let world = unsafe { &mut *self.1 };

    world.get_or_create_component(self)
  }
}

/// A world of entities and components.
///
/// Entities can be spawned and components can be attached/detached from them at runtime.
#[derive(Default)]
pub struct EntityWorld {
  entities: Arena<EntityState>,
}

impl EntityWorld {
  /// Spawns a new entity into the world.
  pub fn spawn(&mut self) -> Entity {
    let state = EntityState::default();
    let id = self.entities.add(state);

    Entity(id, self as *mut _)
  }

  /// Inserts a component into the given entity.
  pub fn insert_component<C: Component>(&mut self, entity: &Entity, component: C) {
    if let Some(state) = self.entities.get_mut(entity.0) {
      state.insert_component(component);
    }
  }

  /// Retrieves the given component for the given entity.
  pub fn get_component<C: Component>(&self, entity: &Entity) -> Option<&C> {
    if let Some(state) = self.entities.get(entity.0) {
      return state.get_component::<C>();
    }

    None
  }

  /// Mutably retrieves the given component for the given entity.
  pub fn get_component_mut<C: Component>(&mut self, entity: &Entity) -> Option<&mut C> {
    if let Some(state) = self.entities.get_mut(entity.0) {
      return state.get_component_mut::<C>();
    }

    None
  }

  /// Retrieves the given component for the given entity, creating it if it doesn't exist.
  pub fn get_or_create_component<C: Component + Default>(&mut self, entity: &Entity) -> &mut C {
    let state = self.entities.get_mut(entity.0).unwrap();

    state.get_or_create_component::<C>()
  }
}

/// Internal state for a particular entity.
#[derive(Default)]
struct EntityState {
  components: AnyMap,
}

impl EntityState {
  /// Retrieves component for the given component type.
  pub fn insert_component<C: Component>(&mut self, component: C) {
    self.components.insert(component);
  }

  /// Retrieves component for the given component type.
  pub fn get_component<C: Component>(&self) -> Option<&C> {
    self.components.get::<C>()
  }

  /// Mutably retrieves component for the given component type.
  pub fn get_component_mut<C: Component>(&mut self) -> Option<&mut C> {
    self.components.get_mut::<C>()
  }

  /// Retrieves component for the given component type, creating it if necessary.
  pub fn get_or_create_component<C: Component + Default>(&mut self) -> &mut C {
    self.components.get_or_create::<C>()
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    graphics::{Color32, Sprite},
    maths::{vec2, FromRandom},
  };

  use super::*;

  #[test]
  fn world_should_maintain_simple_set_of_entities() {
    let mut world = EntityWorld::default();

    let player = world
      .spawn()
      .insert(Sprite::default())
      .insert(Color32::random())
      .insert(vec2(0., 0.));
  }
}
