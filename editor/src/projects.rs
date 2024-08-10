//! Project configuration for the editor.

use common::{info, ToVirtualPath, Version, VirtualPath};

/// The current [`Version`] of the [`Project`] schema.
pub const DEFAULT_PROJECT_VERSION: Version = Version::new(0, 0, 1);

/// An error that can occur when working with a [`Project`].
#[derive(Debug)]
pub enum ProjectError {
  InvalidVersion,
  GeneralIoError,
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
#[derive(Clone, Debug)]
pub struct ProjectDetails {
  pub name: String,
  pub path: String,
  pub version: Version,
}

impl Project {
  /// Opens a project at the given path, or creates a new one.
  pub fn open_or_create(name: &str, root_path: &str) -> Result<Self, ProjectError> {
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
  pub fn discover(_root_path: impl AsRef<str>) -> Result<Vec<ProjectDetails>, ProjectError> {
    todo!()
  }

  /// The root [`VirtualPath`] for the project.
  pub fn root_path(&self) -> VirtualPath {
    self.details.path.clone().to_virtual_path()
  }

  /// Reads the [`Version`] of the project from the settings file.
  pub fn version(&self) -> Result<Version, ProjectError> {
    let path = self.root_path().join("/Settings/ProjectVersion.txt");
    let raw = path.read_all_text().map_err(|_| ProjectError::GeneralIoError)?;

    Version::parse(&raw).map_err(|_| ProjectError::InvalidVersion)
  }
}
