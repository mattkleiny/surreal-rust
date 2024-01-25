//! Possible documents that can be opened in the editor.

/// Represents a document that can be opened in the editor.
pub trait Document {}

/// A document describing a scene in the project.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SceneDocument {}
