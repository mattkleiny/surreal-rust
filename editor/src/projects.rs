//! Project management for Surreal

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
  root_path: String,
  _asset_database: AssetDatabase,
}

impl Project {
  /// Opens a project at the given path.
  pub fn open(root_path: impl AsRef<str>) -> surreal::Result<Self> {
    // TODO: verify that the project is valid and the version is valid
    info!("Opening project at path {}", root_path.as_ref());

    let project = Self {
      root_path: root_path.as_ref().to_string(),
      _asset_database: AssetDatabase::default(),
    };

    Ok(project)
  }

  /// The root [`VirtualPath`] for the project.
  pub fn root_path(&self) -> VirtualPath {
    VirtualPath::from(&self.root_path)
  }

  /// Reads the [`Version`] of the project from the settings file.
  pub fn version(&self) -> surreal::Result<Version> {
    let path = self.root_path().resolve("/Settings/ProjectVersion.txt");
    let raw = path.read_all_text()?;

    Ok(Version::from_str(&raw)?)
  }
}
