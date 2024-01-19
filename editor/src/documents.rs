//! Possible documents that can be opened in the editor.

use serde::{Deserialize, Serialize};

/// Represents a document that can be opened in the editor.
pub trait Document {}

/// A document describing a scene in the project.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SceneDocument {}
