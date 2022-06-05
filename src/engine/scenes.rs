//! Scene management system for Surreal.
//!
//! Scenes are serialized representations of entities and components
//! that can be loaded and saved to disk.

#[derive(Serialize, Deserialize)]
pub struct Scene {}

impl Scene {
  pub fn new() -> Self {
    Self {}
  }
}
