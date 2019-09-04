//! Asset management system.

use std::sync::Arc;

/// Represents a loaded asset in the system.
#[derive(Clone, Debug)]
pub enum Asset<T> {
  Ready(Arc<T>),
  Loading,
  NotFound,
}

/// The source details for an asset.
#[derive(Debug)]
pub struct AssetPath {}

/// An asset type that supports hot-reload whilst the application is running.
pub trait HotReload<T> {
  fn reload(&mut self, path: &AssetPath);
}