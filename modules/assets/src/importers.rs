use std::any::{Any, TypeId};

use common::{InputStream, VirtualPath};

/// An error that can occur when exporting an asset.
#[derive(Debug)]
pub enum AssetImportError {}

/// Imports assets from a specific format.
pub trait AssetImporter: Send + Sync + 'static {
  type Asset;

  /// Returns whether this importer can import the given asset type and path.
  fn can_import(&self, _path: VirtualPath) -> bool {
    true
  }

  /// Imports an asset from the given stream.
  fn import(&self, data: &mut dyn InputStream) -> Result<Self::Asset, AssetImportError>;
}

/// A trait for importing untyped assets.
pub trait UntypedAssetImporter: Send + Sync + 'static {
  fn can_import(&self, asset_type: TypeId, path: VirtualPath) -> bool;
  fn import(&self, data: &mut dyn InputStream) -> Result<Box<dyn Any>, AssetImportError>;
}

/// Allow any typed asset importer to be used as an untyped asset importer.
impl<A: 'static, T: AssetImporter<Asset = A>> UntypedAssetImporter for T {
  fn can_import(&self, asset_type: TypeId, path: VirtualPath) -> bool {
    asset_type == TypeId::of::<A>() && self.can_import(path)
  }

  fn import(&self, data: &mut dyn InputStream) -> Result<Box<dyn Any>, AssetImportError> {
    Ok(Box::new(self.import(data)?))
  }
}
