use crate::vfs::{FileSystem, Path, VFSResult};

#[derive(Debug)]
pub struct ResourceFileSystem;

impl ResourceFileSystem {
  pub fn new() -> Self {
    Self {}
  }
}

impl FileSystem for ResourceFileSystem {
  fn schemes(&self) -> &[&'static str] { &["res", "resource"] }

  fn copy_to(&self, from: &Path, to: &Path) -> VFSResult<()> {
    unimplemented!()
  }

  fn move_to(&self, from: &Path, to: &Path) -> VFSResult<()> {
    unimplemented!()
  }

  fn delete(&self, from: &Path, to: &Path) -> VFSResult<()> {
    unimplemented!()
  }
}