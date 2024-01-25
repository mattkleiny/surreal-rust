use common::OutputStream;

/// An error that can occur when exporting an asset.
#[derive(Debug)]
enum AssetExportError {}

/// Exports assets to a specific format.
pub trait AssetExporter: Send + Sync + 'static {
  /// The type of asset processed by this exporter.
  type Asset;

  /// Exports an asset to the given stream.
  fn export(&self, asset: &Self::Asset, stream: &mut dyn OutputStream) -> Result<(), AssetExportError>;
}
