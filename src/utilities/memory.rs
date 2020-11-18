/// A canonical representation of size, with simple conversions between units.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Size(usize);

impl Size {
  pub const fn bytes(amount: usize) -> Self { Self(amount) }
  pub const fn kilobytes(amount: usize) -> Self { Self::bytes(amount * 1024) }
  pub const fn megabytes(amount: usize) -> Self { Self::kilobytes(amount * 1024) }
  pub const fn gigabytes(amount: usize) -> Self { Self::megabytes(amount * 1024) }

  pub fn as_bytes(&self) -> usize { self.0 }
  pub fn as_kilobytes(&self) -> usize { self.as_bytes() / 1024 }
  pub fn as_megabytes(&self) -> usize { self.as_kilobytes() / 1024 }
  pub fn as_gigabytes(&self) -> usize { self.as_megabytes() / 1024 }
}

impl From<usize> for Size {
  #[inline]
  fn from(amount: usize) -> Self { Self::bytes(amount) }
}

impl From<Size> for usize {
  #[inline]
  fn from(size: Size) -> Self { size.0 }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn size_should_convert_between_scales() {
    let size = Size::gigabytes(1);

    assert_eq!(size.as_gigabytes(), 1);
    assert_eq!(size.as_megabytes(), 1024);
    assert_eq!(size.as_kilobytes(), 1024 * 1024);
    assert_eq!(size.as_bytes(), 1024 * 1024 * 1024);
  }
}
