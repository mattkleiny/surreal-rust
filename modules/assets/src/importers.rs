use common::InputStream;

/// An error that can occur when exporting an asset.
#[derive(Debug)]
pub enum AssetImportError {}

/// Imports assets from a specific format.
pub trait AssetImporter: Send + Sync + 'static {
  /// The type of asset processed by this importer.
  type Asset;

  /// Imports an asset from the given stream.
  fn import(&self, data: &mut dyn InputStream) -> Result<Self::Asset, AssetImportError>;
}
