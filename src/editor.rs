//! Editor support for the engine.
//!
//! Editor support is non-intrusive and instead works through a single interface into the core
//! game state. Implementing the `Interface` trait on your game state will allow the internal
//! mechanics enough information about the game state to provide editor functionality.

/// Primary interface for the editor to access game state.
pub trait Editor {
  /// Requests information about the game's `Actor`s.
  fn get_scene_actors(&self, results: &mut Vec<Actor>);

  // Read/write access to actor properties
  fn read_property<T>(&self, actor: Actor, property: Property) -> Option<T>;
  fn write_property<T>(&mut self, actor: Actor, property: Property, value: T);
}

/// Contains information about a single actor in the game's world.
///
/// This information permits the editor to read/write property information through a shim on `Interface`.
#[derive(Clone, Debug)]
pub struct Actor {
  pub properties: Vec<Property>
}

/// Describes a single property of a single actor.
#[derive(Clone, Debug)]
pub struct Property {
  /// The unique name for this property.
  pub name: String,
  /// A category for this property, for display purposes.
  pub category: String,
  /// The archetype of this property.
  pub archetype: PropertyArchetype,
}

/// The possible types of property that an actor might possess.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PropertyArchetype {
  /// A primitive type, natively supported by the engine.
  Primitive,
  /// A complex type that was created through user-land code.
  Complex,
}
