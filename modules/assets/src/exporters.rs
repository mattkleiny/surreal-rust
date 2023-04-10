/// Exports assets to a specific format.
///
/// This trait is implemented by asset exporters, which are used to export
/// assets to a specific format. This is used to convert assets from one format
/// to another, or to export assets to a format which can be used by the game.
pub trait AssetExporter {
  type Asset;

  /// Exports an asset to a specific format.
  fn export(&self, asset: &Self::Asset) -> Vec<u8>;
}
