use super::*;

#[derive(Copy, Clone, Debug)]
pub enum BmpError {
  Unknown
}

impl Image {
  pub fn load_bmp(path: impl AsRef<str>) -> Result<Self, BmpError> {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_load_bmp_files_successfully() {
    Image::load_bmp("./test.bmp").expect("Failed to load image!");
  }
}