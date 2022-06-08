//! Scene management system for Surreal.
//!
//! Scenes are serialized representations of entities and components
//! that can be loaded and saved to disk.

/// A scene that can be serialized/deserialized from disk.
///
/// This is a high level representation of the objects present in the scene,
/// along with the parameters used to configure them.
///
/// A scene can be converted into a convential `Scene` via the `build()` method.
#[derive(Default, Serialize, Deserialize)]
pub struct PackedScene {}

#[cfg(test)]
mod tests {
  use super::*;

  const SCENE_DOCUMENT: &str = r#"
    objects:
      - name: Player
        archetype: Player
  "#;

  #[test]
  fn packed_scene_should_be_loadable_from_yaml() {
    todo!()
  }
}
