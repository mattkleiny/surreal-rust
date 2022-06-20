//! A scene management system, based on a very lightweight single threaded ECS system.

use crate::collections::{AnyMap, Arena, ArenaIndex};

/// Represents an entity in a [`Scene`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Entity(ArenaIndex, *mut Scene);

impl Entity {
  /// Inserts the given component into the entity.
  pub fn insert<C: Component>(self, component: C) -> Self {
    let scene = unsafe { &mut *self.1 };

    scene.insert_component(&self, component);

    self
  }

  /// Retrieves the given component from the entity.
  pub fn get<C: Component>(&self) -> Option<&C> {
    let scene = unsafe { &*self.1 };

    scene.get_component(self)
  }

  /// Retrieves the given component from the entity.
  pub fn get_unchecked<C: Component>(&self) -> &C {
    let scene = unsafe { &*self.1 };

    scene.get_component(self).unwrap()
  }

  /// Mutably retrieves the given component from the entity.
  pub fn get_mut<C: Component>(&self) -> Option<&mut C> {
    let scene = unsafe { &mut *self.1 };

    scene.get_component_mut(self)
  }

  /// Retrieves the given component from the entity.
  pub fn get_mut_unchecked<C: Component>(&self) -> &mut C {
    let scene = unsafe { &mut *self.1 };

    scene.get_component_mut(self).unwrap()
  }

  /// Retrieves the given component from the entity, creating it if it doesn't exist.
  pub fn get_or_create<C: Component + Default>(&self) -> &mut C {
    let scene = unsafe { &mut *self.1 };

    scene.get_or_create_component(self)
  }
}

/// An archetype for instantiating an entity.
pub trait Archetype {
  /// Instantiates the given entity with the associated archetype.
  fn create(&self, entity: Entity) -> Entity;
}

/// Represents a copmonent that can be attached to an entity.
pub trait Component: Sized + 'static {}

/// Blanket implementation for all possible component types.
impl<C: Sized + 'static> Component for C {}

/// A scene of entities and components.
///
/// Entities can be spawned and components can be attached/detached from them at runtime.
#[derive(Default)]
pub struct Scene {
  entities: Arena<EntityState>,
}

impl Scene {
  /// Spawns a new entity into the scene.
  pub fn spawn(&mut self) -> Entity {
    let state = EntityState::default();
    let id = self.entities.add(state);

    Entity(id, self as *mut _)
  }

  /// Spawns a new entity into the scene with the given archetype.
  pub fn spawn_archetype(&mut self, archetype: impl Archetype) -> Entity {
    archetype.create(self.spawn())
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
  is_alive: bool,
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
    graphics::Color32,
    maths::{vec2, FromRandom, Vector2},
  };

  use super::*;

  struct Transform {
    position: Vector2<f32>,
    rotation: f32,
  }

  struct Model {
    animation: String,
    tint: Color32,
  }

  #[test]
  fn scene_should_maintain_simple_set_of_entities() {
    let mut scene = Scene::default();

    let _player = scene
      .spawn()
      .insert(Transform {
        position: vec2(0., 0.),
        rotation: 0.,
      })
      .insert(Model {
        animation: "idle_down".to_string(),
        tint: Color32::random(),
      });
  }
}
