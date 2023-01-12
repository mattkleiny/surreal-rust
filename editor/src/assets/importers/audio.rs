use surreal::io::VirtualPath;

use crate::{Asset, AssetImporter};

#[derive(Default)]
pub struct AudioImporter {}

impl AssetImporter for AudioImporter {
  fn can_handle(&self, path: &VirtualPath) -> bool {
    todo!()
  }

  fn import(&self, path: &VirtualPath) -> surreal::Result<Box<dyn Asset>> {
    todo!()
  }
}
