//! Project management for Surreal

use std::fmt::{Display, Formatter};

use thiserror::Error;

use surreal::diagnostics::info;
use surreal::utilities::TypeDatabase;

use super::*;

/// Possible errors when loading a project.
#[derive(Error, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProjectError {
  #[error("An invalid project format was detected")]
  InvalidFormat,
  #[error("The project version is incompatible with the current editor version")]
  IncompatibleVersion,
}

/// Represents a project in the Surreal editor.
///
/// A project is a collection of assets and settings that can be loaded and
/// edited in the editor. Projects are stored in the _local_ file system and
/// can be loaded from any location.
pub struct Project {
  root_path: String,
  asset_database: AssetDatabase,
  type_database: TypeDatabase,
}

impl Project {
  /// Opens a project at the given path.
  pub fn open(root_path: impl AsRef<str>) -> Result<Self, ProjectError> {
    // TODO: verify that the project is valid and the version is valid
    info!("Opening project at path {}", root_path.as_ref());

    let project = Self {
      root_path: root_path.as_ref().to_string(),
      asset_database: AssetDatabase::default(),
      type_database: TypeDatabase::default(),
    };

    Ok(project)
  }
}
