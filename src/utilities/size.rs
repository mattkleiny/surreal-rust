use std::fmt::{Debug, Formatter};

/// A canonical representation of size, with simple conversions between units.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Size(usize);

impl Size {
  pub const fn from_bytes(amount: usize) -> Self { Self(amount) }
  pub fn from_kilobytes(amount: f32) -> Self { Self::from_bytes((amount * 1024.) as usize) }
  pub fn from_megabytes(amount: f32) -> Self { Self::from_kilobytes(amount * 1024.) }
  pub fn from_gigabytes(amount: f32) -> Self { Self::from_megabytes(amount * 1024.) }

  pub fn as_bytes(&self) -> usize { self.0 }
  pub fn as_kilobytes(&self) -> f32 { self.as_bytes() as f32 / 1024. }
  pub fn as_megabytes(&self) -> f32 { self.as_kilobytes() / 1024. }
  pub fn as_gigabytes(&self) -> f32 { self.as_megabytes() / 1024. }
}

impl From<usize> for Size {
  fn from(amount: usize) -> Self {
    Self::from_bytes(amount)
  }
}

impl Debug for Size {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    match () {
      _ if self.as_gigabytes() >= 1. => write!(formatter, "{} gigabytes", self.as_gigabytes()),
      _ if self.as_megabytes() >= 1. => write!(formatter, "{} megabytes", self.as_megabytes()),
      _ if self.as_kilobytes() >= 1. => write!(formatter, "{} kilobytes", self.as_kilobytes()),
      _ => write!(formatter, "{} bytes", self.as_bytes()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn size_should_convert_between_scales() {
    let size = Size::from_gigabytes(1.);

    assert_eq!(size.as_gigabytes(), 1.);
    assert_eq!(size.as_megabytes(), 1024.);
    assert_eq!(size.as_kilobytes(), 1024. * 1024.);
    assert_eq!(size.as_bytes(), 1024 * 1024 * 1024);
  }

  #[test]
  fn size_should_print_to_string() {
    assert_eq!(format!("{:?}", Size::from_gigabytes(1.5)), "1.5 gigabytes");
    assert_eq!(format!("{:?}", Size::from_megabytes(2.)), "2 megabytes");
    assert_eq!(format!("{:?}", Size::from_kilobytes(3.)), "3 kilobytes");
    assert_eq!(format!("{:?}", Size::from_bytes(1024 * 4 + 512)), "4.5 kilobytes");
    assert_eq!(format!("{:?}", Size::from_bytes(512)), "512 bytes");
  }
}
