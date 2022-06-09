//! Scene management system for Surreal.

/// A scene that can be serialized/deserialized from disk.
///
/// This is a high level representation of the objects present in the scene,
/// along with the parameters used to configure them.
///
/// A scene can be converted into a convential `Scene` via the `build()` method.
#[derive(Default, Serialize, Deserialize)]
pub struct PackedScene {}
