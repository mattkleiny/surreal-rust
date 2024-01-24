//! Project configuration for the editor.

use common::{info, AsVirtualPath, Version, VirtualPath};
use serde::{Deserialize, Serialize};

/// The current [`Version`] of the [`Project`] schema.
pub const DEFAULT_PROJECT_VERSION: Version = Version::new(0, 0, 1);

/// Possible errors when loading a project.
#[derive(thiserror::Error, Debug, Copy, Clone, Eq, PartialEq)]
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
  pub asset_path: VirtualPath,
  pub target_path: VirtualPath,
}

/// Top-level details for a [`Project`].
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectDetails {
  pub name: String,
  pub path: String,
  pub version: Version,
}

impl Project {
  /// Opens a project at the given path, or creates a new one if it doesn't
  /// exist.
  pub fn open_or_create(name: &str, root_path: &str) -> common::Result<Self> {
    let root_path = root_path.to_virtual_path();

    let asset_path = root_path.join("assets");
    let target_path = root_path.join("target");

    info!("Opening project {} at path {}", name, root_path);

    let project = Self {
      details: ProjectDetails {
        name: name.to_string(),
        path: root_path.to_string(),
        version: DEFAULT_PROJECT_VERSION,
      },
      asset_path,
      target_path,
    };

    // verify that the project is valid and the version is valid
    let version = project.version()?;
    if version != DEFAULT_PROJECT_VERSION {
      todo!("handle incompatible versions")
    }

    Ok(project)
  }

  /// Discovers all available projects beneath a given path.
  pub fn discover(_root_path: impl AsRef<str>) -> common::Result<Vec<ProjectDetails>> {
    todo!()
  }

  /// The root [`VirtualPath`] for the project.
  pub fn root_path(&self) -> VirtualPath {
    self.details.path.to_virtual_path()
  }

  /// Reads the [`Version`] of the project from the settings file.
  pub fn version(&self) -> common::Result<Version> {
    let path = self.root_path().join("/Settings/ProjectVersion.txt");
    let raw = path.read_all_text()?;

    Version::parse(&raw)
  }
}
