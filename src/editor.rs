//! Editor support for the engine.
//!
//! This module is non-intrusive and instead works through a single interface into the core
//! game state. Implementing the `Interface` trait on your game state will allow the internal
//! mechanics enough information about the game state to provide editor functionality.

// TODO: think about how to render a basic editor interface (maybe using a widget framework like Conrod)?
// TODO: think about editor state, how to get the editor access to the GameState structure.
// TODO: think about how the editor can fetch and access entity information and emit changes to components
// TODO: think about how to implement hot-reloading and other niceties
// TODO: pooled arrays of primitive types and pooled allocator?

use std::rc::Rc;

use crate::graphics::{Canvas, Color};
use crate::maths::{Vector2, Vector3};

/// Provides editor functionality to a game.
pub struct Editor {
  canvas: Canvas,
  plugins: Vec<Box<dyn EditorPlugin>>,
}

impl Editor {
  pub fn new() -> Self {
    Self {
      canvas: Canvas::new(),
      plugins: Vec::new(),
    }
  }
}

/// Primary interface for the editor to access game state.
pub trait EditorAccess {}

/// A plugin for the editor that can extend it's functionality holistically.
pub trait EditorPlugin {}

/// Contains information about a single object in the game's world.
/// This permits the editor to read/write properties directly in the game.
pub trait Object {
  fn get(&self, name: impl AsRef<str>) -> Result<Variant, ObjectError>;
  fn set(&mut self, name: impl AsRef<str>, value: Variant) -> Result<(), ObjectError>;
  fn props(&self) -> &[Property];
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ObjectError {
  PropertyNotFound
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

#[derive(Clone, Debug)]
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

