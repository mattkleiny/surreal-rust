//! A virtual file system.

// TODO: play with this some more.

/// Represents a path in a virtual file system.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path {
  scheme: String,
  path: String,
}

impl Path {
  pub fn copy_to(&self, to: Path) { unimplemented!() }
  pub fn move_to(&self, to: Path) { unimplemented!() }
  pub fn delete(&self, to: Path) { unimplemented!() }
  pub fn write_raw(&self, buffer: &[u8]) { unimplemented!() }
  pub fn write_text(&self, buffer: impl AsRef<str>) { unimplemented!() }
  pub fn read_raw(&self) -> Vec<u8> { unimplemented!() }
  pub fn read_text(&self) -> String { unimplemented!() }

  pub fn open_stream(&self) -> Stream { unimplemented!() }
}

#[derive(Debug)]
pub struct Stream<'a> {
  path: &'a Path
}