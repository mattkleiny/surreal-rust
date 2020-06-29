//! A virtual file system.

/// Represents a path in a virtual file system.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Path {
  pub path: String,
  pub scheme: String,
}

impl Path {
  // movement
  pub fn copy_to(&self, to: Path) { unimplemented!() }
  pub fn move_to(&self, to: Path) { unimplemented!() }
  pub fn delete(&self, to: Path) { unimplemented!() }

  // reading
  pub fn read_raw(&self) -> Vec<u8> { unimplemented!() }
  pub fn read_text(&self) -> String { unimplemented!() }

  // writing
  pub fn write_raw(&self, buffer: &[u8]) { unimplemented!() }
  pub fn write_text(&self, buffer: impl AsRef<str>) { unimplemented!() }

  /// Accesses the file system represented by the path.
  pub fn filesystem(&self) -> &dyn FileSystem { unimplemented!() }
}

impl AsRef<Path> for &'static str {
  fn as_ref(&self) -> &Path {
    unimplemented!()
  }
}

/// Abstractly represents a file system.
pub trait FileSystem {}