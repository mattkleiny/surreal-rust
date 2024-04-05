use surreal_common::{AssetDatabase, AssetImportError, AssetImporter, InputStream};

#[test]
pub fn it_should_load_an_aseprite_file_and_export_it() {
  let mut database = AssetDatabase::open("assets").unwrap();

  database.add_importer(Box::new(AsepriteFileImporter));

  let _aseprite_file = database.load::<AsepriteFile>("assets/test.ase").unwrap();
}

pub struct AsepriteFile;
pub struct AsepriteFileImporter;

impl AssetImporter for AsepriteFileImporter {
  type Asset = AsepriteFile;

  fn import(&self, data: &mut dyn InputStream) -> Result<Self::Asset, AssetImportError> {
    let _magic_number = data.read_u8()?;
    let _frames = data.read_u16()?;
    let _width = data.read_u16()?;
    let _height = data.read_u16()?;

    Ok(AsepriteFile)
  }
}
