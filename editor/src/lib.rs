//! Editor support for Surreal
//!
//! Editing include scene management, resource loaders, hot loading, plugins,
//! inspectors, reflection and a central message bus as well as a UI that
//! can be composed for differing workflows depending on the game being built.

#![feature(anonymous_lifetime_in_impl_trait)]

pub use assets::*;
pub use projects::*;
pub use reflect::*;
pub use serialization::*;
pub use ui::*;

mod assets;
mod projects;
mod reflect;
mod serialization;
mod ui;

/// A unique identifier for a [`Resource`] in the application.
///
/// IDs are unique across a single instance of an application, but not
/// necessarily across all applications.
pub type ResourceId = surreal::maths::Guid;

/// Represents a kind of resource in the application.
///
/// Resources are naturally serializable and can be persisted and loaded
/// from the virtual file system.
pub trait Resource: Sized + surreal::utilities::Object + serde::Serialize + for<'de> serde::Deserialize<'de> {
  /// Loads this resource from the given [`surreal::io::VirtualPath`].
  fn load(path: impl Into<surreal::io::VirtualPath>) -> surreal::Result<Self>;

  /// Saves this resource to the given [`surreal::io::VirtualPath`].
  fn save(&self, path: impl Into<surreal::io::VirtualPath>) -> surreal::Result<()>;
}
