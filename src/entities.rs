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

// TODO: rethink the lifetime parameters used here

/// The type we use for entity and component indexes.
type Index = u32;

/// Uniquely identifies an entity in the entity system.
///
/// We use a style of indexing commonly known as generational indices.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EntityId {
  index: Index,
  generation: u16,
}

impl EntityId {
  pub fn new(index: Index, generation: u16) -> Self {
    Self {
      index,
      generation,
    }
  }
}

/// An arena of entities managed using generational indices.
struct EntityArena {}

impl EntityArena {
  pub fn new() -> Self {
    Self {}
  }
}

/// Describes a component that may be attached to an entity.
///
/// Each component type defines the way in which it is stored, as well as a unique mask value for use in aspect
/// calculations.
pub trait Component: Sized + Default {
  type Storage: ComponentStorage<Self> = BTreeStorage<Self>;
}

/// Retrieves the mask for the given component type.
#[inline(always)]
fn get_component_mask<C: 'static + Component>() -> ComponentMask {
  unsafe { std::intrinsics::type_id::<C>() }
}

/// Defines possible storage types for entity components.
pub trait ComponentStorage<C: Component>: Sized {
  /// Creates a new instance of this storage, ready to accept instances.
  fn new() -> Self;

  /// Creates a new component; either inserting a new component or resetting an existing one.
  fn create(&mut self, index: Index);

  /// Gets immutable access to a component in storage.
  fn get(&self, index: Index) -> &C;

  /// Gets mutable access to a component in storage.
  fn get_mut(&mut self, index: Index) -> &mut C;

  /// Removes an existing component from storage.
  fn remove(&mut self, index: Index);
}

/// B-tree based sparse component storage.
///
/// B-tree based storage is good enough for most use cases, with average memory usage and average loop
/// cost. It's the default choice for components.
pub struct BTreeStorage<C: Component> {
  components: BTreeMap<Index, C>,
}

impl<C: Component> ComponentStorage<C> for BTreeStorage<C> {
  fn new() -> Self {
    Self { components: BTreeMap::new() }
  }

  fn create(&mut self, index: Index) {
    self.components.insert(index, C::default());
  }

  fn get(&self, index: Index) -> &C {
    self.components.get(&index).unwrap()
  }

  fn get_mut(&mut self, index: Index) -> &mut C {
    self.components.get_mut(&index).unwrap()
  }

  fn remove(&mut self, index: Index) {
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

  fn create(&mut self, index: Index) {
    let index = index as usize;
    let length = self.components.len();

    if length <= index {
      let delta = index + 1 - length;

      self.components.reserve(delta);
      unsafe { self.components.set_len(index + 1); }
    }

    self.components.insert(index, C::default());
  }

  fn get(&self, index: Index) -> &C {
    self.components.get(index as usize).unwrap()
  }

  fn get_mut(&mut self, index: Index) -> &mut C {
    self.components.get_mut(index as usize).unwrap()
  }

  fn remove(&mut self, index: Index) {
    self.components.remove(index as usize);
  }
}

/// Hash-based sparse component storage.
///
/// Does not waste space for entities that don't possess the associated components, but is much less efficient to loop
/// over due to the components being retained in a hash-table with variable offsets.
pub struct HashStorage<C: Component> {
  components: HashMap<Index, C>,
}

impl<C: Component> ComponentStorage<C> for HashStorage<C> {
  fn new() -> Self {
    Self { components: HashMap::new() }
  }

  fn create(&mut self, index: Index) {
    self.components.insert(index, C::default());
  }

  fn get(&self, index: Index) -> &C {
    self.components.get(&index).unwrap()
  }

  fn get_mut(&mut self, index: Index) -> &mut C {
    self.components.get_mut(&index).unwrap()
  }

  fn remove(&mut self, index: Index) {
    self.components.remove(&index);
  }
}

/// A bag of component storage.
struct ComponentBag {
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
    let result = self.storages.get_mut(&mask);

    result.unwrap().downcast_mut().unwrap()
  }
}

/// Provides the ability to look-up entities by aspect.
pub trait AspectProvider {
  fn get_entities<'a>(&self, aspect: Aspect) -> &'a [EntityId];
}

/// The precision we require for representing components on entities.
type ComponentMask = u64;

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

/// Represents a system that operates on entities and processes their components.
pub trait System {
  /// Updates this system by a single frame.
  fn tick(&mut self, delta_time: f64);
}

/// The entity world.
///
/// This is the entry point to the ECS system, and provides storage for all entities, systems and components in the
/// game world.
pub struct World {
  entities: EntityArena,
  components: ComponentBag,
  systems: Vec<Box<dyn System>>,
}

impl World {
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
  pub fn register_system<S: 'static + System>(&mut self, system: S) {
    self.systems.push(Box::new(system));
  }

  /// Ticks all of the attached systems.
  pub fn tick(&mut self, delta_time: f64) {
    for system in self.systems.iter_mut() {
      system.tick(delta_time);
    }
  }
}

#[cfg(test)]
mod tests {
  use glam::Vec2;

  use super::*;

  #[derive(Default, Debug)]
  struct TestComponent1 {
    position: Vec2,
    rotation: f32,
  }

  impl Component for TestComponent1 {
    type Storage = HashStorage<Self>;
  }

  #[derive(Default, Debug)]
  struct TestComponent2;

  impl Component for TestComponent2 {
    type Storage = VecStorage<Self>;
  }

  struct TestSystem;

  impl System for TestSystem {
    fn tick(&mut self, delta_time: f64) {
      println!("Delta time is {}", delta_time);
    }
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

    storage.create(index);
    storage.get(index);
    storage.get_mut(index);
    storage.remove(index);
  }

  #[test]
  fn btree_storage_should_read_and_write() {
    let index = 42;
    let mut storage = BTreeStorage::<TestComponent1>::new();

    storage.create(index);
    storage.get(index);
    storage.get_mut(index);
    storage.remove(index);
  }

  #[test]
  fn vec_storage_should_read_and_write() {
    let index = 42;
    let mut storage = VecStorage::<TestComponent1>::new();

    storage.create(index);
    storage.get(index);
    storage.get_mut(index);
    storage.remove(index);
  }

  #[test]
  fn hash_storage_should_read_and_write() {
    let index = 42;
    let mut storage = HashStorage::<TestComponent1>::new();

    storage.create(index);
    storage.get(index);
    storage.get_mut(index);
    storage.remove(index);
  }

  #[test]
  fn world_should_register_components() {
    let mut world = World::new();

    world.register_component::<TestComponent1>();
    world.register_component::<TestComponent2>();
  }

  #[test]
  fn world_should_register_systems() {
    let mut world = World::new();

    world.register_system(TestSystem);
  }

  #[test]
  fn world_should_tick_attached_systems() {
    let mut world = World::new();

    world.register_system(TestSystem);

    for i in 0..100 {
      world.tick(0.16);
    }
  }
}