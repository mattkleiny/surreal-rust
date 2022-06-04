//! A lightweight entity component system.

use std::any::{type_name, TypeId};
use std::fmt::{Debug, Formatter};

use crate::collections::{AnyMap, Arena, ArenaIndex};

/// Represents an entity in the ECS.
pub type Entity = ArenaIndex;

/// Internally managed state for a single entity.
#[derive(Default)]
struct EntityState {
  /// Indicates which components are present on this entity.
  masks: ComponentMask,
}

/// A mask for a component.
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct ComponentMask(u64);

impl Debug for ComponentMask {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "ComponentMask({})", self.0)
  }
}

/// Represents a unique component type.
#[derive(Debug, Eq, PartialEq)]
pub struct ComponentType {
  pub type_id: TypeId,
  pub name: &'static str,
  pub mask: ComponentMask,
}

impl ComponentType {
  /// Constructs a new component type for the given [`T`].
  pub fn from<T>() -> Self
  where T: 'static {
    Self {
      type_id: TypeId::of::<T>(),
      name: type_name::<T>(),
      mask: ComponentMask(0),
    }
  }

  /// Returns all [`ComponentType`] indicated by the given mask.
  pub fn for_mask(_mask: ComponentMask) -> Vec<Self> {
    todo!()
  }
}

/// Represents a component in the ECS.
pub trait Component: Sized + 'static {
  type Storage: ComponentStorage<Self>;

  /// Gets the type for this component.
  fn component_type() -> ComponentType {
    ComponentType::from::<Self>()
  }
}

/// Blanket [`Component`] implementation for all standard types.
impl<C> Component for C
where C: Copy + Sized + 'static
{
  type Storage = DefaultStorage<C>;
}

/// Provides a means of storing entity [`Component`]s.
pub trait ComponentStorage<C>: Default {
  fn add_component(&mut self, entity: Entity, component: C);
  fn get_component(&self, entity: Entity) -> Option<&C>;
  fn get_component_mut(&mut self, entity: Entity) -> Option<&mut C>;
  fn remove_component(&mut self, entity: Entity);
}

/// A simple internal storage type for testing purposes.
type DefaultStorage<C> = [Option<C>; 32];

impl<C> ComponentStorage<C> for DefaultStorage<C>
where C: Copy
{
  fn add_component(&mut self, entity: Entity, component: C) {
    self[entity.index] = Some(component)
  }

  fn get_component(&self, entity: Entity) -> Option<&C> {
    self.get(entity.index).and_then(|c| c.as_ref())
  }

  fn get_component_mut(&mut self, entity: Entity) -> Option<&mut C> {
    self.get_mut(entity.index).and_then(|c| c.as_mut())
  }

  fn remove_component(&mut self, entity: Entity) {
    self[entity.index] = None;
  }
}

/// A simple entity component system world.
#[derive(Default)]
pub struct World {
  /// Each individual entity in the world.
  entities: Arena<EntityState>,
  /// Storage for each unique component type.
  components: AnyMap,
}

impl World {
  /// Creates a new blank world.
  pub fn new() -> Self {
    Self {
      entities: Arena::new(),
      components: AnyMap::new(),
    }
  }

  /// Spawns a new entity into the world.
  pub fn spawn(&mut self) -> Entity {
    self.entities.add(EntityState::default())
  }

  /// De-spawns an existing entity from the world.
  pub fn despawn(&mut self, entity: Entity) {
    if let Some(_state) = self.entities.remove(entity) {
      // TODO: iterate all storages and remove components
    }
  }

  /// Adds a component to the given entity.
  pub fn add_component<C: Component>(&mut self, entity: Entity, component: C) {
    if let Some(_state) = self.entities.get_mut(entity) {
      let storage = self.components.get_or_create::<C::Storage>();

      storage.add_component(entity, component);
      // state.masks |= C::component_type().mask;
    }
  }

  /// Retrieves a component for the given entity.
  pub fn get_component<C: Component>(&self, entity: Entity) -> Option<&C> {
    let _ = self.entities.get(entity)?;
    let storage = self.components.get::<C::Storage>()?;

    storage.get_component(entity)
  }

  /// Retrieves a component mutably for the given entity.
  pub fn get_component_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
    let _ = self.entities.get(entity)?;
    let storage = self.components.get_mut::<C::Storage>()?;

    storage.get_component_mut(entity)
  }

  /// Removes a component for the given entity.
  pub fn remove_component<C: Component>(&mut self, entity: Entity) {
    if let Some(_state) = self.entities.get_mut(entity) {
      if let Some(storage) = self.components.get_mut::<C::Storage>() {
        storage.remove_component(entity);
        // state.masks &= C::component_type().mask;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{vec2, Vector2};

  use super::*;

  #[test]
  fn world_should_insert_entity() {
    let mut world = World::new();

    let entity1 = world.spawn();
    let entity2 = world.spawn();

    assert_ne!(entity1, entity2);
  }

  #[test]
  fn world_should_add_and_get_component() {
    let mut world = World::new();
    let entity = world.spawn();

    world.add_component(entity, vec2(0f32, 10f32));
    world.add_component(entity, 2. * std::f32::consts::PI);

    let position = world
      .get_component::<Vector2<f32>>(entity)
      .expect("Failed to get component");

    let rotation = world
      .get_component::<f32>(entity)
      .expect("Failed to get component");

    assert_eq!(*position, vec2(0., 10.));
    assert_eq!(*rotation, 2. * std::f32::consts::PI);
  }

  #[test]
  fn world_should_get_component_mutably() {
    let mut world = World::new();
    let entity = world.spawn();

    world.add_component(entity, vec2(0f32, 10f32));

    let position = world
      .get_component_mut::<Vector2<f32>>(entity)
      .expect("Failed to get component");

    position.x += 10.;
    position.y += 10.;

    assert_eq!(*position, vec2(10., 20.));
  }

  #[test]
  fn world_should_remove_component() {
    let mut world = World::new();
    let entity = world.spawn();

    world.add_component(entity, vec2(0f32, 10f32));
    world.remove_component::<Vector2<f32>>(entity);

    let position = world.get_component::<Vector2<f32>>(entity);

    assert_eq!(position, None);
  }
}
