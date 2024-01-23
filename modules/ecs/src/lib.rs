//! A lightweight and flexible entity-component system for Surreal.
//!
//! This is a simple implementation of the ECS idea with direct support for
//! idioms and components that are used in Surreal.

#![feature(associated_type_defaults)]
#![feature(core_intrinsics)]
#![allow(internal_features)]

use common::{Arena, ArenaIndex, FastHashMap, StringName};

common::impl_rid!(EntityId);

/// Represents an entity.
#[derive(Default)]
pub struct Entity {
  pub name: Option<StringName>,
}

/// A reference to an entity in the entity manager.
pub struct EntityRef<'a> {
  entity_id: EntityId,
  entity_manager: &'a EntityManager,
}

impl<'a> EntityRef<'a> {
  /// Gets the component of the given type for this entity.
  #[inline]
  pub fn get_component<C: Component + 'static>(&self) -> Option<&C> {
    self.entity_manager.get_component::<C>(self.entity_id)
  }
}

/// A mutable reference to an entity in the entity manager.
pub struct EntityRefMut<'a> {
  entity_id: EntityId,
  entity_manager: &'a mut EntityManager,
}

impl<'a> EntityRefMut<'a> {
  /// Adds a component of the given type to this entity.
  #[inline]
  pub fn add_component<C: Component + 'static>(&mut self, component: C) {
    self.entity_manager.add_component(self.entity_id, component);
  }

  /// Removes the component of the given type from this entity.
  #[inline]
  pub fn remove_component<C: Component + 'static>(&mut self) {
    self.entity_manager.remove_component::<C>(self.entity_id);
  }

  /// Gets the component of the given type for this entity.
  #[inline]
  pub fn get_component<C: Component + 'static>(&self) -> Option<&C> {
    self.entity_manager.get_component::<C>(self.entity_id)
  }

  /// Mutably gets the component of the given type for this entity.
  #[inline]
  pub fn get_component_mut<C: Component + 'static>(&mut self) -> Option<&mut C> {
    self.entity_manager.get_component_mut::<C>(self.entity_id)
  }
}

/// A manager for entities.
#[derive(Default)]
pub struct EntityManager {
  entities: Arena<Entity>,
  components: FastHashMap<ComponentType, Box<dyn std::any::Any>>,
}

impl EntityManager {
  /// Creates a new entity.
  pub fn create_entity(&mut self) -> EntityRefMut {
    let entity_id = EntityId::from(self.entities.insert(Entity { name: None }));

    EntityRefMut {
      entity_id,
      entity_manager: self,
    }
  }

  /// Gets a reference to the entity with the given ID.
  pub fn get_entity(&self, entity_id: EntityId) -> Option<EntityRef> {
    if !self.entities.contains(entity_id.into()) {
      return None;
    }

    Some(EntityRef {
      entity_id,
      entity_manager: self,
    })
  }

  /// Gets a mutable reference to the entity with the given ID.
  pub fn get_entity_mut(&mut self, entity_id: EntityId) -> Option<EntityRefMut> {
    if !self.entities.contains(entity_id.into()) {
      return None;
    }

    Some(EntityRefMut {
      entity_id,
      entity_manager: self,
    })
  }

  /// Adds a component of the given type to the entity with the given ID.
  pub fn add_component<C: Component + 'static>(&mut self, entity_id: EntityId, component: C) {
    self
      .get_or_create_storage_mut::<C>()
      .add_component(entity_id, component);
  }

  /// Removes the component of the given type from the entity with the given ID.
  pub fn remove_component<C: Component + 'static>(&mut self, entity_id: EntityId) {
    if let Some(storage) = self.get_storage_mut::<C>() {
      storage.remove_component(entity_id);
    }
  }

  /// Gets the component of the given type for the given entity.
  pub fn get_component<C: Component + 'static>(&self, entity_id: EntityId) -> Option<&C> {
    self
      .get_storage::<C>()
      .and_then(|storage| storage.get_component(entity_id))
  }

  /// Mutably gets the component of the given type for the given entity.
  pub fn get_component_mut<C: Component + 'static>(&mut self, entity_id: EntityId) -> Option<&mut C> {
    self
      .get_storage_mut::<C>()
      .and_then(|storage| storage.get_component_mut(entity_id))
  }

  /// Gets the storage for the given component type.
  pub fn get_storage<C: Component + 'static>(&self) -> Option<&C::Storage> {
    self
      .components
      .get(&ComponentType::of::<C>())
      .and_then(|storage| storage.downcast_ref::<C::Storage>())
  }

  /// Mutably gets the storage for the given component type.
  pub fn get_storage_mut<C: Component + 'static>(&mut self) -> Option<&mut C::Storage> {
    self
      .components
      .get_mut(&ComponentType::of::<C>())
      .and_then(|storage| storage.downcast_mut::<C::Storage>())
  }

  /// Mutably gets the storage for the given component type.
  pub fn get_or_create_storage_mut<C: Component + 'static>(&mut self) -> &mut C::Storage {
    self
      .components
      .entry(ComponentType::of::<C>())
      .or_insert_with(|| Box::<<C as Component>::Storage>::default());

    self
      .components
      .get_mut(&ComponentType::of::<C>())
      .and_then(|storage| storage.downcast_mut::<C::Storage>())
      .expect("Failed to create storage for component type")
  }
}

/// Represents a component that can be attached to an entity.
pub trait Component: Default + Sized {
  /// The storage type for this component.
  type Storage: ComponentStorage<Self> = FastHashMap<EntityId, Self>;
}

/// Encapsulates the type of a component.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ComponentType(u128);

impl ComponentType {
  /// Creates a new component type.
  pub fn of<C: Component + 'static>() -> Self {
    Self(std::intrinsics::type_id::<C>())
  }
}

/// Storage for a component.
pub trait ComponentStorage<C: Component>: Default {
  /// Adds a component to this storage.
  fn add_component(&mut self, entity_id: EntityId, component: C);

  /// Removes the component for the given entity.
  fn remove_component(&mut self, entity_id: EntityId);

  /// Gets the component for the given entity.
  fn get_component(&self, entity_id: EntityId) -> Option<&C>;

  /// Mutably gets the component for the given entity.
  fn get_component_mut(&mut self, entity_id: EntityId) -> Option<&mut C>;
}

/// Dense storage for a component using a vector.
impl<C: Component> ComponentStorage<C> for Vec<C> {
  fn add_component(&mut self, entity_id: EntityId, component: C) {
    let index = u32::from(entity_id) as usize;

    if index >= self.len() {
      self.resize_with(index + 1, || C::default());
    }

    self[index] = component;
  }

  fn remove_component(&mut self, entity_id: EntityId) {
    let index = u32::from(entity_id) as usize;

    if index < self.len() {
      self.remove(index);
    }
  }

  fn get_component(&self, entity_id: EntityId) -> Option<&C> {
    self.get(u32::from(entity_id) as usize)
  }

  fn get_component_mut(&mut self, entity_id: EntityId) -> Option<&mut C> {
    self.get_mut(u32::from(entity_id) as usize)
  }
}

/// Dense storage for a component using an arena.
impl<C: Component> ComponentStorage<C> for Arena<C> {
  fn add_component(&mut self, _entity_id: EntityId, _component: C) {
    todo!()
  }

  fn remove_component(&mut self, _entity_id: EntityId) {
    todo!()
  }

  fn get_component(&self, entity_id: EntityId) -> Option<&C> {
    self.get(ArenaIndex::from(entity_id))
  }

  fn get_component_mut(&mut self, entity_id: EntityId) -> Option<&mut C> {
    self.get_mut(ArenaIndex::from(entity_id))
  }
}

/// Sparse storage for a component using a hash map.
impl<C: Component> ComponentStorage<C> for FastHashMap<EntityId, C> {
  fn add_component(&mut self, entity_id: EntityId, component: C) {
    self.insert(entity_id, component);
  }

  fn remove_component(&mut self, entity_id: EntityId) {
    self.remove(&entity_id);
  }

  fn get_component(&self, entity_id: EntityId) -> Option<&C> {
    self.get(&entity_id)
  }

  fn get_component_mut(&mut self, entity_id: EntityId) -> Option<&mut C> {
    self.get_mut(&entity_id)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Default)]
  struct TestComponent {}

  impl Component for TestComponent {
    type Storage = FastHashMap<EntityId, Self>;
  }

  #[test]
  fn test_basic_component_access() {
    let mut entity_manager = EntityManager::default();
    let mut entity = entity_manager.create_entity();

    entity.add_component(TestComponent {});
  }
}
