use super::*;

#[derive(Copy, Clone, Debug)]
pub enum JpgError {
  Unknown
}

impl Image {
  pub fn load_jpg(path: impl AsRef<str>) -> Result<Self, JpgError> {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_load_jpg_files_successfully() {
    Image::load_jpg("./test.jpg").expect("Failed to load image!");
  }
}