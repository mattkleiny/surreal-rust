use common::{InputStream, Result};

/// Imports assets from a specific format.
pub trait AssetImporter: Send + Sync + 'static {
  /// The type of asset processed by this importer.
  type Asset;

  /// Imports an asset from the given stream.
  fn import(&self, data: &mut dyn InputStream) -> Result<Self::Asset>;
}
