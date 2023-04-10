/// Trait for importing assets from a specific format.
///
/// This trait is implemented by asset importers, which are used to import
/// assets from a specific format. This is used to convert assets from one
/// format to another, or to import assets into a format which can be used by
/// the game.
pub trait AssetImporter {
  type Asset;

  /// Imports an asset from a specific format.
  fn import(&self, data: &[u8]) -> Self::Asset;
}
