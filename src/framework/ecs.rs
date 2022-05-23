use std::any::{Any, type_name, TypeId};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::collections::{Arena, ArenaIndex};

/// Represents an entity in the ECS.
pub type Entity = ArenaIndex;

/// A mask for a component.
#[derive(Copy, Clone, Eq, PartialEq)]
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
  pub fn from<T>() -> Self where T: 'static {
    Self {
      type_id: TypeId::of::<T>(),
      name: type_name::<T>(),
      mask: ComponentMask(0),
    }
  }

  /// Returns all [`ComponentType`] indicated by the given mask.
  pub fn for_mask(_mask: ComponentMask) -> Self {
    todo!()
  }
}

/// Represents a component in the ECS.
pub trait Component: Sized + 'static {
  type Storage: ComponentStorage<Self>;
}

/// Blanket [`Component`] implementation for all standard types.
impl<C> Component for C where C: Copy + Sized + 'static {
  type Storage = DefaultStorage<C>;
}

/// Provides a means of storing entity [`Component`]s.
pub trait ComponentStorage<C> {
  /// Constructs a new instance of this storage.
  fn new() -> Self;

  fn add_component(&mut self, entity: Entity, component: C);
  fn get_component(&self, entity: Entity) -> Option<&C>;
  fn get_component_mut(&mut self, entity: Entity) -> Option<&mut C>;
  fn remove_component(&mut self, entity: Entity);
}

/// A simple internal storage type for testing purposes.
type DefaultStorage<C> = [Option<C>; 32];

impl<C> ComponentStorage<C> for DefaultStorage<C> where C: Copy {
  fn new() -> Self {
    [None; 32]
  }

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
pub struct World {
  entities: Arena<()>,
  components: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
  /// Creates a new blank world.
  pub fn new() -> Self {
    Self {
      entities: Arena::new(),
      components: HashMap::new(),
    }
  }

  /// Spawns a new entity into the world.
  pub fn spawn(&mut self) -> Entity {
    self.entities.add(())
  }

  /// De-spawns an existing entity from the world.
  pub fn despawn(&mut self, entity: Entity) {
    if self.entities.remove(entity) {
      // TODO: destroy components as well?
    }
  }

  /// Adds a component to the given entity.
  pub fn add_component<C>(&mut self, entity: Entity, component: C) where C: Component {
    let type_id = TypeId::of::<C>();

    let storage = self.components
        .entry(type_id)
        .or_insert_with(|| Box::new(C::Storage::new()))
        .downcast_mut::<C::Storage>()
        .expect(format!("Failed to access component storage for {:?}", type_id).as_str());

    storage.add_component(entity, component);
  }

  /// Retrieves a component for the given entity.
  pub fn get_component<C>(&self, entity: Entity) -> Option<&C> where C: Component {
    let type_id = TypeId::of::<C>();

    let storage = self.components
        .get(&type_id)?
        .downcast_ref::<C::Storage>()?;

    storage.get_component(entity)
  }

  /// Retrieves a component mutably for the given entity.
  pub fn get_component_mut<C>(&mut self, entity: Entity) -> Option<&mut C> where C: Component {
    let type_id = TypeId::of::<C>();

    let storage = self.components
        .get_mut(&type_id)?
        .downcast_mut::<C::Storage>()?;

    return storage.get_component_mut(entity);
  }

  /// Removes a component for the given entity.
  pub fn remove_component<C>(&mut self, entity: Entity) where C: Component {
    let type_id = TypeId::of::<C>();

    if let Some(storage) = self.components
        .get_mut(&type_id)
        .and_then(|storage| storage.downcast_mut::<C::Storage>()) {
      storage.remove_component(entity);
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

  #[test]
  fn component_type_should_construct_arbitrarily() {
    let component_type = ComponentType::from::<Vector2<f32>>();

    println!("{:#?}", component_type);
  }
}