//! Editor support for the engine.
//!
//! This module is non-intrusive and instead works through a single interface into the core
//! game state. Implementing the `Interface` trait on your game state will allow the internal
//! mechanics enough information about the game state to provide editor functionality.

// TODO: think about how to render a basic editor interface (maybe using a widget framework like Conrod)?
// TODO: think about editor state, how to get the editor access to the GameState structure.
// TODO: think about how the editor can fetch and access entity information and emit changes to components
// TODO: think about how to implement hot-reloading and other niceties
// TODO: think about

use std::rc::Rc;

use crate::graphics::Color;
use crate::maths::{Vector2, Vector3};

/// Primary interface for the editor to access game state.
pub trait EditorAccess {
  fn query_scene_objects(&self) -> Vec<Object>;
}

/// Identifies this game object uniquely amongst it's peers.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ObjectID(usize);

/// Contains information about a single object in the game's world.
/// This permits the editor to read/write properties directly in the game.
#[derive(Clone, Debug)]
pub struct Object {
  pub id: ObjectID,
  pub prop: Vec<Property>,
}

#[derive(Clone, Debug)]
pub struct Property {
  pub name: String,
  pub category: String,
  pub hint: PropertyHint,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PropertyHint {}

#[repr(u8)]
#[derive(BitFlags, Copy, Clone, Debug, Eq, PartialEq)]
pub enum PropertyUsage {
  Normal = 1 << 0,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Variant {
  Nil,
  Bool(bool),
  Int(i64),
  Float(f64),
  String(Rc<String>),
  Vector2(Vector2<f32>),
  Vector3(Vector3<f32>),
  Color(Color),
}
