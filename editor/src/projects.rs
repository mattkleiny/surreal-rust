//! Project management for Surreal

use serde::{Deserialize, Serialize};
use thiserror::Error;

use surreal::diagnostics::info;
use surreal::io::VirtualPath;
use surreal::utilities::Version;

use super::*;

/// The current [`Version`] of the [`Project`] schema.
pub const DEFAULT_PROJECT_VERSION: Version = Version::new(0, 0, 1);

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
  /// The top-level details for this project.
  pub details: ProjectDetails,
  _asset_database: AssetDatabase,
}

/// Top-level details for a [`Project`].
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectDetails {
  pub name: String,
  pub version: Version,
  pub path: String,
}

impl Project {
  /// Opens a project at the given path.
  pub fn open(_root_path: impl AsRef<str>) -> surreal::Result<Self> {
    todo!()
  }

  /// Creates a project at the given path, removing any old project data if it already exists.
  pub fn create(_root_path: impl AsRef<str>) -> surreal::Result<Self> {
    todo!()
  }

  /// Opens a project at the given path, or creates a new one if it doesn't exist.
  pub fn open_or_create(name: impl AsRef<str>, root_path: impl AsRef<str>) -> surreal::Result<Self> {
    info!("Opening project {} at path {}", name.as_ref(), root_path.as_ref());

    let project = Self {
      details: ProjectDetails {
        name: name.as_ref().to_string(),
        version: DEFAULT_PROJECT_VERSION,
        path: root_path.as_ref().to_string(),
      },
      _asset_database: AssetDatabase::default(),
    };

    // TODO: verify that the project is valid and the version is valid

    Ok(project)
  }

  /// Discovers all available projects beneath a given path.
  pub fn discover(_root_path: impl AsRef<str>) -> surreal::Result<Vec<ProjectDetails>> {
    todo!()
  }

  /// The root [`VirtualPath`] for the project.
  pub fn root_path(&self) -> VirtualPath {
    VirtualPath::from(&self.details.path)
  }

  /// Reads the [`Version`] of the project from the settings file.
  pub fn version(&self) -> surreal::Result<Version> {
    let path = self.root_path().resolve("/Settings/ProjectVersion.txt");
    let raw = path.read_all_text()?;

    Ok(Version::from_str(&raw)?)
  }
}
