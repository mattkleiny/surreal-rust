//! A lightweight entity component system that favours developer productivity over feature set.

use std::any::{Any, TypeId};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::rc::Rc;

/// An opaque handle to an entity in the ECS.
pub struct Entity {
  id: usize,
  world: WorldContext,
}

impl Entity {
  /// Adds a component 'C' to the entity.
  pub fn add_component<C: 'static + Component>(&mut self, component: C) -> &mut C {
    self.world.get_or_create_storage::<C>().add_component(self.id, component)
  }

  /// Retrieves a reference for the given component type `C` for the entity.
  pub fn get_component<C: 'static + Component>(&mut self) -> Option<&C> {
    self.world.get_storage::<C>().and_then(|storage| storage.get_component(self.id))
  }

  /// Retrieves a mutable reference for the given component type `C` for the entity.
  pub fn get_component_mut<C: 'static + Component>(&mut self) -> Option<&mut C> {
    self.world.get_or_create_storage::<C>().get_component_mut(self.id)
  }

  /// Retrieves a reference for the given component type `C` for the entity, creating it if it doesn't exist.
  pub fn get_or_create_component<C: 'static + Component>(&mut self) -> &mut C {
    self.world.get_or_create_storage::<C>().get_or_create_component(self.id)
  }
}

/// Represents a type of component that can be used in the entity component system.
pub trait Component: Sized + Default {
  /// The type of storage associated with this component.
  type Storage: ComponentStorage<Self>;
}

/// Allows storage of `Component`s in different schemes depending on use cases.
pub trait ComponentStorage<C: Component> {
  fn new() -> Self where Self: Sized;

  fn add_component(&mut self, entity_id: usize, component: C) -> &mut C;
  fn get_component(&self, entity_id: usize) -> Option<&C>;
  fn get_component_mut(&mut self, entity_id: usize) -> Option<&mut C>;
  fn get_or_create_component(&mut self, entity_id: usize) -> &mut C;
}

/// Stores components in a flat vec for fast iteration but slower insertion and removal.
pub struct VecStorage<C> {
  components: Vec<C>,
}

impl<C: Component> ComponentStorage<C> for VecStorage<C> {
  fn new() -> Self where Self: Sized {
    Self { components: Vec::new() }
  }

  fn add_component(&mut self, entity_id: usize, component: C) -> &mut C {
    self.components.insert(entity_id, component);
    self.get_component_mut(entity_id).unwrap()
  }

  fn get_component(&self, entity_id: usize) -> Option<&C> {
    self.components.get(entity_id)
  }

  fn get_component_mut(&mut self, entity_id: usize) -> Option<&mut C> {
    self.components.get_mut(entity_id)
  }

  fn get_or_create_component(&mut self, entity_id: usize) -> &mut C {
    if self.components.get(entity_id).is_none() {
      self.add_component(entity_id, C::default())
    } else {
      self.get_component_mut(entity_id).unwrap()
    }
  }
}

/// Stores components in a hash map for fast insertion and removal but slower iteration.
pub struct HashMapStorage<C> {
  components: HashMap<usize, C>,
}

impl<C: Component> ComponentStorage<C> for HashMapStorage<C> {
  fn new() -> Self where Self: Sized {
    Self { components: HashMap::new() }
  }

  fn add_component(&mut self, entity_id: usize, component: C) -> &mut C {
    self.components.insert(entity_id, component);
    self.get_component_mut(entity_id).unwrap()
  }

  fn get_component(&self, entity_id: usize) -> Option<&C> {
    self.components.get(&entity_id)
  }

  fn get_component_mut(&mut self, entity_id: usize) -> Option<&mut C> {
    self.components.get_mut(&entity_id)
  }

  fn get_or_create_component(&mut self, entity_id: usize) -> &mut C {
    if self.components.get(&entity_id).is_none() {
      self.add_component(entity_id, C::default())
    } else {
      self.get_component_mut(entity_id).unwrap()
    }
  }
}

/// The entity world.
pub struct World {
  entities: Vec<usize>,
  components: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
  /// Creates a new empty world.
  pub fn new() -> WorldContext {
    WorldContext::with_world(Self {
      entities: Vec::new(),
      components: HashMap::new(),
    })
  }
}

/// A context for accessing a `World`.
#[derive(Clone)]
pub struct WorldContext {
  world: Rc<UnsafeCell<World>>,
}

impl WorldContext {
  /// Constructs a new world context with the given world.
  pub fn with_world(world: World) -> Self {
    Self { world: Rc::new(UnsafeCell::new(world)) }
  }

  /// Spawns a new entity into the world.
  pub fn spawn(&mut self) -> Entity {
    let world = unsafe { &*self.world.get() };
    let entity_id = world.entities.len();

    Entity {
      id: entity_id,
      world: self.clone(),
    }
  }

  /// Retrieves the component storage for the given component type, `C`.
  pub fn get_storage<C: 'static + Component>(&mut self) -> Option<&mut C::Storage> {
    let world = unsafe { &mut *self.world.get() };

    world.components
        .get_mut(&TypeId::of::<C>())
        .map(|value| value
            .downcast_mut::<C::Storage>()
            .unwrap())
  }

  /// Retrieves the component storage for the given component type, `C`.
  pub fn get_or_create_storage<C: 'static + Component>(&mut self) -> &mut C::Storage {
    let world = unsafe { &mut *self.world.get() };

    world.components
        .entry(TypeId::of::<C>())
        .or_insert_with(|| Box::new(C::Storage::new()))
        .downcast_mut::<C::Storage>()
        .unwrap()
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{vec2, Vector2};

  use super::*;

  #[derive(Default, Debug)]
  struct Position(Vector2<f32>);

  impl Component for Position {
    type Storage = VecStorage<Self>;
  }

  #[derive(Default, Debug)]
  struct Rotation(f32);

  impl Component for Rotation {
    type Storage = VecStorage<Self>;
  }

  #[test]
  fn it_should_add_entities_and_update_components() {
    let mut world = World::new();
    let mut entity = world.spawn();

    println!("Position is {:#?}", entity.add_component(Position(vec2(1., 2.))));
    println!("Rotation is {:#?}", entity.add_component(Rotation(2. * std::f32::consts::PI)));
  }
}