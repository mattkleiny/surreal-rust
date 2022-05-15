//! A lightweight entity component system that favours developer productivity over feature set.

use std::any::{Any, TypeId};
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::rc::Rc;

/// An opaque handle to an entity in the ECS.
pub struct Entity {
  id: usize,
  world: World,
}

impl Entity {
  /// Adds a component 'C' to the entity.
  pub fn add<C: 'static + Component>(&self, component: C) -> &mut C {
    self.world.get_or_create_storage::<C>().add_component(self.id, component)
  }

  /// Retrieves a reference for the given component type `C` for the entity.
  pub fn get<C: 'static + Component>(&self) -> Option<&C> {
    self.world.get_storage::<C>().and_then(|storage| storage.get_component(self.id))
  }

  /// Retrieves a mutable reference for the given component type `C` for the entity.
  pub fn get_mut<C: 'static + Component>(&self) -> Option<&mut C> {
    self.world.get_or_create_storage::<C>().get_component_mut(self.id)
  }

  /// Retrieves a reference for the given component type `C` for the entity, creating it if it doesn't exist.
  pub fn get_or_create<C: 'static + Component>(&self) -> &mut C {
    self.world.get_or_create_storage::<C>().get_or_create_component(self.id)
  }
}

/// Represents a type of component that can be used in the entity component system.
pub trait Component: Sized + Default {
  /// The type of storage associated with this component.
  type Storage: ComponentStorage<Self>;
}

/// Blanket storage for all potential component types, `T`.
impl<T> Component for T where T: Sized + Default {
  type Storage = VecStorage<Self>;
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

impl<C> ComponentStorage<C> for VecStorage<C> where C: Component {
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

impl<C> ComponentStorage<C> for HashMapStorage<C> where C: Component {
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

/// Represents an entity `World`.
///
/// World state is managed through interior mutability;
///
/// A world can be cloned and internally will point to the same world state, thus it's safe to
/// pass a world instance to different systems and structs.
#[derive(Clone)]
pub struct World {
  state: Rc<UnsafeCell<WorldState>>,
}

/// Worlds can be sent between threads but we don't support automatic `Sync`.
unsafe impl Send for World {}

impl World {
  /// Constructs a new blank world.
  pub fn new() -> Self {
    Self {
      state: Rc::new(UnsafeCell::new(WorldState::new()))
    }
  }

  /// Spawns a new entity into the world.
  pub fn spawn(&mut self) -> Entity {
    let state = unsafe { &*self.state.get() };
    let entity_id = state.entities.len();

    Entity {
      id: entity_id,
      world: self.clone(),
    }
  }

  /// Retrieves the component storage for the given component type, `C`.
  pub fn get_storage<C: 'static + Component>(&self) -> Option<&mut C::Storage> {
    let state = unsafe { &mut *self.state.get() };

    state.components
        .get_mut(&TypeId::of::<C>())
        .map(|value| value
            .downcast_mut::<C::Storage>()
            .unwrap())
  }

  /// Retrieves the component storage for the given component type, `C`, creating it if it doesn't exist.
  pub fn get_or_create_storage<C: 'static + Component>(&self) -> &mut C::Storage {
    let state = unsafe { &mut *self.state.get() };

    state.components
        .entry(TypeId::of::<C>())
        .or_insert_with(|| Box::new(C::Storage::new()))
        .downcast_mut::<C::Storage>()
        .unwrap()
  }

  /// Updates the world a single frame.
  pub fn tick(&self) {
    let state = unsafe { &mut *self.state.get() };

    state.tick();
  }
}

/// The internal state of a `World`.
struct WorldState {
  entities: Vec<usize>,
  components: HashMap<TypeId, Box<dyn Any>>,
}

impl WorldState {
  /// Creates a new empty world state.
  pub fn new() -> Self {
    Self {
      entities: Vec::new(),
      components: HashMap::new(),
    }
  }

  /// Updates the world state a single frame.
  pub fn tick(&mut self) {
    // TODO: implement me
  }
}

#[cfg(test)]
mod tests {
  use crate::maths::{vec2, Vector2};

  use super::*;

  /// An example component representing a position in 2-space.
  #[derive(Default, Debug)]
  struct Position(Vector2<f32>);

  /// An example component representing a rotation in 2-space (in radians from the positive Y axis).
  #[derive(Default, Debug)]
  struct Rotation(f32);

  #[test]
  fn it_should_add_components_to_entities() {
    let mut world = World::new();

    let entity = world.spawn();

    let position = entity.add(Position(vec2(1., 2.)));
    let rotation = entity.add(Rotation(std::f32::consts::PI));

    println!("Position is {:#?}", position);
    println!("Rotation is {:#?}", rotation);
  }
}