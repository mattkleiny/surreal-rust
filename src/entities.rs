//! A lightweight and fast single-threaded entity system.
//!
//! This ECS implementation is very light-weight, and does not consider parallel execution nor pure Rust-invariant
//! safety. Instead, we concern ourselves with simplicity of access and script-ability of the entity system.
//!
//! If you're looking for better multi-core performance, or better code in general, I recommend you consider 'specs',
//! instead.

use std::collections::HashMap;

/// Uniquely identifies an entity in the entity system.
///
/// We use a style of indexing commonly known as generational indices.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EntityId {
  sequence: u16,
  generation: u16,
}

/// Describes a component that may be attached to an entity.
///
/// Each component type defines the way in which it is stored, as well as a unique mask value for use in aspect
/// calculations.
pub trait Component: Sized {
  type Storage: ComponentStorage<Self>;
}

/// Describes the component types that a entity system wishes to operate upon.
///
/// An aspect is a bit-mask of a set of component types, and allows efficient storage and access to those components
/// when requesting them from component storage.
pub struct Aspect {
  mask: usize,
}

/// Provides the ability to look-up entities by aspect.
pub trait EntityLookup {
  fn get_entities<'a>(&self, aspect: Aspect) -> &'a [EntityId];
}

/// Represents a system that operates on entities and processes their components.
pub trait System {
  /// Updates this system by a single frame.
  fn tick(&mut self, delta_time: f64, lookup: &impl EntityLookup);
}

/// Defines possible storage types for entity components.
pub trait ComponentStorage<C: Component> {
  /// Gets immutable access to a component in storage.
  fn get(&self, entity_id: &EntityId) -> &C;

  /// Gets mutable access to a component in storage.
  fn get_mut(&mut self, entity_id: &EntityId) -> &mut C;
}

/// Densely-packed component storage.
///
/// Wastes space for entities that don't possess the associated components, but is very efficient to iterate over for
/// data that is frequently accessed on a frame-by-frame basis.
pub struct DenseStorage<C: Component> {
  components: Vec<C>,
}

impl<C: Component> ComponentStorage<C> for DenseStorage<C> {
  fn get(&self, entity_id: &EntityId) -> &C {
    unimplemented!()
  }

  fn get_mut(&mut self, entity_id: &EntityId) -> &mut C {
    unimplemented!()
  }
}

/// Sparse-packed component storage.
///
/// Does not waste space for entities that don't possess the associated components, but is much less efficient to loop
/// over due to the components being retained in a hash-table with variable offsets.
pub struct SparseStorage<C: Component> {
  components: HashMap<EntityId, C>,
}

impl<C: Component> ComponentStorage<C> for SparseStorage<C> {
  fn get(&self, entity_id: &EntityId) -> &C {
    unimplemented!()
  }

  fn get_mut(&mut self, entity_id: &EntityId) -> &mut C {
    unimplemented!()
  }
}
