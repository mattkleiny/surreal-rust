use super::*;

#[derive(Copy, Clone, Debug)]
pub enum PngError {
  Unknown
}

impl Image {
  pub fn load_png(path: impl AsRef<str>) -> Result<Self, PngError> {
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_load_png_files_successfully() {
    Image::load_png("./test.png").expect("Failed to load image!");
  }
}