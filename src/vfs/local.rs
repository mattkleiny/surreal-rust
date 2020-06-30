use crate::vfs::{FileSystem, Path, VFSResult};

#[derive(Debug)]
pub struct LocalFileSystem;

impl LocalFileSystem {
  pub fn new() -> Self {
    Self {}
  }
}

impl FileSystem for LocalFileSystem {
  fn schemes(&self) -> &[&'static str] { &["", "local", "file"] }

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