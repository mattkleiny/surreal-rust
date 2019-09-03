//! A lightweight and fast single-threaded entity system.
//!
//! This ECS implementation is very light-weight, and does not consider parallel execution nor pure Rust-invariant
//! safety. Instead, we concern ourselves with simplicity of access and script-ability of the entity system.
//!
//! If you're looking for better multi-core performance, or better code in general, I recommend you consider 'specs',
//! instead. I've modelled the API in a similar style to specs, both to allow easy upgrade if you desire, but also
//! because it's clean and simple for the most part.

use std::any::Any;
use std::collections::{BTreeMap, HashMap};

use crate::collections::{Arena, ArenaIndex};

// TODO: rethink the lifetime parameters used here.
// TODO: wrap the component bag to allow simpler access from systems.

/// We store entities in a generational arena, but we only use the indices
/// to represent the presence or absence of an entity.
///
/// The data for an entity is represented through the conjunction of it's
/// components.
type EntityArena = Arena<()>;

/// Uniquely identifies an entity in the entity system.
/// We use a style of indexing commonly known as generational indices.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct EntityId(ArenaIndex);

/// Describes a component that may be attached to an entity.
///
/// Each component type defines the way in which it is stored, as well as a unique mask value for use in aspect
/// calculations.
pub trait Component: Sized {
  type Storage: ComponentStorage<Self> = BTreeStorage<Self>;
}

type ComponentIndex = usize;
type ComponentMask = u64;

/// Retrieves the mask for the given component type.
#[inline(always)]
fn get_component_mask<C: 'static + Component>() -> ComponentMask {
  unsafe { std::intrinsics::type_id::<C>() }
}

/// Retrieves the mask for the given component type.
#[inline(always)]
fn get_component_name<C: 'static + Component>() -> &'static str {
  std::intrinsics::type_name::<C>()
}

/// Defines possible storage types for entity components.
pub trait ComponentStorage<C: Component>: Sized {
  /// Creates a new instance of this storage, ready to accept instances.
  fn new() -> Self;

  /// Creates a new component; either inserting a new component or resetting an existing one.
  fn create(&mut self, index: ComponentIndex, value: C);

  /// Gets immutable access to a component in storage.
  fn get(&self, index: ComponentIndex) -> &C;

  /// Gets mutable access to a component in storage.
  fn get_mut(&mut self, index: ComponentIndex) -> &mut C;

  /// Removes an existing component from storage.
  fn remove(&mut self, index: ComponentIndex);
}

/// B-tree based sparse component storage.
///
/// B-tree based storage is good enough for most use cases, with average memory usage and average loop
/// cost. It's the default choice for components.
pub struct BTreeStorage<C: Component> {
  components: BTreeMap<ComponentIndex, C>,
}

impl<C: Component> ComponentStorage<C> for BTreeStorage<C> {
  fn new() -> Self {
    Self { components: BTreeMap::new() }
  }

  fn create(&mut self, index: ComponentIndex, value: C) {
    self.components.insert(index, value);
  }

  fn get(&self, index: ComponentIndex) -> &C {
    self.components.get(&index).expect(&format!("Unable to find entity {}", index))
  }

  fn get_mut(&mut self, index: ComponentIndex) -> &mut C {
    self.components.get_mut(&index).expect(&format!("Unable to find entity {}", index))
  }

  fn remove(&mut self, index: ComponentIndex) {
    self.components.remove(&index);
  }
}

/// Vec-based dense component storage.
///
/// Wastes space for entities that don't possess the associated components, but is very efficient to iterate over for
/// data that is frequently accessed on a frame-by-frame basis.
pub struct VecStorage<C: Component> {
  components: Vec<C>,
}

impl<C: Component> ComponentStorage<C> for VecStorage<C> {
  fn new() -> Self {
    Self { components: Vec::new() }
  }

  fn create(&mut self, index: ComponentIndex, value: C) {
    let index = index as usize;
    let length = self.components.len();

    if length <= index {
      let delta = index + 1 - length;

      self.components.reserve(delta);
      unsafe { self.components.set_len(index + 1); }
    }

    self.components.insert(index, value);
  }

  fn get(&self, index: ComponentIndex) -> &C {
    self.components.get(index as usize).expect(&format!("Unable to find entity {}", index))
  }

  fn get_mut(&mut self, index: ComponentIndex) -> &mut C {
    self.components.get_mut(index as usize).expect(&format!("Unable to find entity {}", index))
  }

  fn remove(&mut self, index: ComponentIndex) {
    self.components.remove(index as usize);
  }
}

/// Hash-based sparse component storage.
///
/// Does not waste space for entities that don't possess the associated components, but is much less efficient to loop
/// over due to the components being retained in a hash-table with variable offsets.
pub struct HashMapStorage<C: Component> {
  components: HashMap<ComponentIndex, C>,
}

impl<C: Component> ComponentStorage<C> for HashMapStorage<C> {
  fn new() -> Self {
    Self { components: HashMap::new() }
  }

  fn create(&mut self, index: ComponentIndex, value: C) {
    self.components.insert(index, value);
  }

  fn get(&self, index: ComponentIndex) -> &C {
    self.components.get(&index).expect(&format!("Unable to find entity {}", index))
  }

  fn get_mut(&mut self, index: ComponentIndex) -> &mut C {
    self.components.get_mut(&index).expect(&format!("Unable to find entity {}", index))
  }

  fn remove(&mut self, index: ComponentIndex) {
    self.components.remove(&index);
  }
}

/// A bag of component storage.
pub struct ComponentBag {
  storages: HashMap<ComponentMask, Box<dyn Any>>,
}

impl ComponentBag {
  pub fn new() -> Self {
    Self { storages: HashMap::new() }
  }

  /// Creates the storage for the given component type.
  pub fn create<C: 'static + Component>(&mut self) {
    let mask = get_component_mask::<C>();
    let storage = Box::new(C::Storage::new());

    self.storages.insert(mask, storage);
  }

  /// Gets or creates the storage for the given component.
  pub fn get<C: 'static + Component>(&mut self) -> &mut C::Storage {
    let mask = get_component_mask::<C>();
    let name = get_component_name::<C>();

    let result = self.storages.get_mut(&mask)
        .expect(&format!("Attempted to access component storage for unregistered type {}", name));

    result.downcast_mut().unwrap() // this should never fault; otherwise we've screwed up good
  }
}

/// Describes the component types that a entity system wishes to operate upon.
///
/// An aspect is a bit-mask of a set of component types, and allows efficient storage and access to those components
/// when requesting them from component storage.
#[derive(Copy, Clone, Debug)]
pub struct Aspect {
  mask: ComponentMask,
}

impl Aspect {
  pub fn new() -> Self {
    Self { mask: 0 }
  }

  /// Includes the given type in the aspect.
  #[inline]
  pub fn include<C: 'static + Component>(self) -> Aspect {
    let component_mask = get_component_mask::<C>();
    Self { mask: self.mask | component_mask }
  }

  /// Excludes the given type from the aspect.
  #[inline]
  pub fn exclude<C: 'static + Component>(self) -> Aspect {
    let component_mask = get_component_mask::<C>();
    Self { mask: self.mask & !component_mask }
  }

  /// Determines if the aspect includes the given component.
  #[inline]
  pub fn has<C: 'static + Component>(&self) -> bool {
    let component_mask = get_component_mask::<C>();
    self.mask | component_mask == component_mask
  }
}

/// The entity world.
///
/// This is the entry point to the ECS system, and provides storage for all entities, systems and
/// components in the game world.
///
/// The type of system we retain, S, is generalizable on a world-by-world basis.
pub struct World<S: Sized> {
  entities: EntityArena,
  components: ComponentBag,
  systems: Vec<S>,
}

impl<S> World<S> {
  pub fn new() -> Self {
    Self {
      entities: EntityArena::new(),
      components: ComponentBag::new(),
      systems: Vec::new(),
    }
  }

  /// Registers the given component with the system.
  pub fn register_component<C: 'static + Component>(&mut self) {
    self.components.create::<C>();
  }

  /// Registers the given system.
  pub fn register_system(&mut self, system: S) {
    self.systems.push(system);
  }

  /// Creates a new entity.
  pub fn create_entity(&mut self) -> EntityId {
    EntityId(self.entities.insert(()))
  }

  /// Deletes an existing entity from the world
  pub fn delete_entity(&mut self, entity_id: EntityId) {
    self.entities.remove(entity_id.0);
  }

  /// Executes the given instruction on all of the attached systems.
  pub fn execute<B>(&mut self, mut body: B)
    where B: FnMut(&mut S) -> () {
    for system in self.systems.iter_mut() {
      body(system);
    }
  }
}

/// A utility for fluently building entities.
pub struct EntityBuilder<'a, S> {
  world: &'a World<S>,
  entity_id: EntityId,
}

impl<'a, S> EntityBuilder<'a, S> {
  pub fn new(world: &'a mut World<S>) -> Self {
    let entity_id = world.create_entity();
    Self { world, entity_id }
  }

  /// Attaches a component to the entity.
  pub fn with<C: 'static + Component>(self, component: C) -> Self {
    unimplemented!()
  }

  /// Builds the resultant entity.
  pub fn build(self) -> EntityId {
    self.entity_id
  }
}

#[cfg(test)]
mod tests {
  use glam::Vec2;

  use super::*;

  #[derive(Default, Debug)]
  struct TestComponent1 {
    position: Vec2,
    velocity: Vec2,
    rotation: f32,
  }

  impl Component for TestComponent1 {
    type Storage = HashMapStorage<Self>;
  }

  #[derive(Default, Debug)]
  struct TestComponent2;

  impl Component for TestComponent2 {
    type Storage = VecStorage<Self>;
  }

  struct TestSystem;

  trait System {
    fn tick(&mut self, delta_time: f64);
  }

  impl System for TestSystem {
    fn tick(&mut self, delta_time: f64) {}
  }

  #[test]
  fn aspect_should_build_a_valid_component_mask() {
    let aspect = Aspect::new()
        .include::<TestComponent1>()
        .exclude::<TestComponent2>();

    assert!(aspect.mask > 0);
    assert!(aspect.has::<TestComponent1>());
    assert!(!aspect.has::<TestComponent2>());
  }

  #[test]
  fn component_bag_should_read_and_write_components() {
    let index = 42;
    let mut components = ComponentBag::new();

    components.create::<TestComponent1>();
    let storage = components.get::<TestComponent1>();

    storage.create(index, TestComponent1::default());
    storage.get(index);
    storage.get_mut(index);
    storage.remove(index);
  }

  #[test]
  fn btree_storage_should_read_and_write() {
    let index = 42;
    let mut storage = BTreeStorage::<TestComponent1>::new();

    storage.create(index, TestComponent1::default());
    storage.get(index);
    storage.get_mut(index);
    storage.remove(index);
  }

  #[test]
  fn vec_storage_should_read_and_write() {
    let index = 42;
    let mut storage = VecStorage::<TestComponent1>::new();

    storage.create(index, TestComponent1::default());
    storage.get(index);
    storage.get_mut(index);
    storage.remove(index);
  }

  #[test]
  fn hash_storage_should_read_and_write() {
    let index = 42;
    let mut storage = HashMapStorage::<TestComponent1>::new();

    storage.create(index, TestComponent1::default());
    storage.get(index);
    storage.get_mut(index);
    storage.remove(index);
  }

  #[test]
  fn world_should_register_components() {
    let mut world = World::<Box<dyn System>>::new();

    world.register_component::<TestComponent1>();
    world.register_component::<TestComponent2>();
  }

  #[test]
  fn world_should_register_systems() {
    let mut world = World::<Box<dyn System>>::new();

    world.register_system(Box::new(TestSystem));
  }

  #[test]
  fn world_should_create_and_delete_entities() {
    let mut world = World::<Box<dyn System>>::new();

    let entity1 = world.create_entity();
    let entity2 = world.create_entity();
    let entity3 = world.create_entity();

    world.delete_entity(entity1);
    world.delete_entity(entity2);
    world.delete_entity(entity3);
  }

  #[test]
  fn world_should_build_entities() {
    let mut world = World::<Box<dyn System>>::new();

    let entity = EntityBuilder::new(&mut world)
        .with(TestComponent1::default())
        .with(TestComponent2::default())
        .build();

    world.delete_entity(entity);
  }

  #[test]
  fn world_should_tick_attached_systems() {
    let mut world = World::<Box<dyn System>>::new();

    world.register_component::<TestComponent1>();
    world.register_component::<TestComponent2>();

    world.register_system(Box::new(TestSystem));

    let storage = world.components.get::<TestComponent1>();

    for i in 0..1000 {
      storage.create(i, TestComponent1 {
        position: Vec2::zero(),
        velocity: Vec2::zero(),
        rotation: 90.,
      });
    }

    for i in 0..100 {
      world.execute(|system| {
        system.tick(0.16);
      });
    }
  }
}