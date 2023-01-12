use surreal::io::VirtualPath;

use crate::{Asset, AssetImporter};

#[derive(Default)]
pub struct ImageImporter {}

impl AssetImporter for ImageImporter {
  fn can_handle(&self, path: &VirtualPath) -> bool {
    todo!()
  }

  fn import(&self, path: &VirtualPath) -> surreal::Result<Box<dyn Asset>> {
    todo!()
  }
}
