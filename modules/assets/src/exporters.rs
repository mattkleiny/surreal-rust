use std::any::{Any, TypeId};

use common::{OutputStream, VirtualPath};

/// An error that can occur when exporting an asset.
#[derive(Debug)]
pub enum AssetExportError {}

/// Exports assets to a specific format.
pub trait AssetExporter: Send + Sync + 'static {
  type Asset;

  /// Returns whether this export can export the given asset type and path.
  fn can_export(&self, _path: VirtualPath) -> bool {
    true
  }

  /// Exports an asset to the given stream.
  fn export(&self, asset: &Self::Asset, stream: &mut dyn OutputStream) -> Result<(), AssetExportError>;
}

/// A trait for exporting untyped assets.
pub trait UntypedAssetExporter: Send + Sync + 'static {
  fn can_export(&self, asset_type: TypeId, path: VirtualPath) -> bool;
  fn export(&self, asset: &dyn Any, stream: &mut dyn OutputStream) -> Result<(), AssetExportError>;
}

/// Allow any typed asset importer to be used as an untyped asset importer.
impl<A: 'static, T: AssetExporter<Asset = A>> UntypedAssetExporter for T {
  fn can_export(&self, asset_type: TypeId, path: VirtualPath) -> bool {
    asset_type == TypeId::of::<A>() && self.can_export(path)
  }

  fn export(&self, asset: &dyn Any, stream: &mut dyn OutputStream) -> Result<(), AssetExportError> {
    self.export(asset.downcast_ref::<A>().unwrap(), stream)
  }
}
