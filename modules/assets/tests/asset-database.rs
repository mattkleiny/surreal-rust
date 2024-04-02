use common::{InputStream, VirtualPath};
use surreal_assets::{AssetDatabase, AssetImporter, AssetImportError};

#[test]
pub fn it_should_build_a_valid_asset_database() {
  let mut database = AssetDatabase::open("assets").unwrap();

  database.add_importer(AsepriteFileImporter);

  let test = database.load::<AsepriteFile>("assets/test.ase").unwrap();
}

pub struct AsepriteFile;
pub struct AsepriteFileImporter;

impl AssetImporter for AsepriteFileImporter {
  type Asset = AsepriteFile;

  fn can_import(&self, path: VirtualPath) -> bool {
    return path.extension().ends_with("aseprite") || path.extension().ends_with("ase");
  }

  fn import(&self, data: &mut dyn InputStream) -> Result<Self::Asset, AssetImportError> {
    todo!()
  }
}
