use std::fmt::{Debug, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

/// A canonical representation of size, with simple conversions between units.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Size(usize);

impl Size {
  #[inline(always)]
  pub const fn from_bytes(amount: usize) -> Self {
    Self(amount)
  }

  #[inline(always)]
  pub fn from_kilobytes(amount: f32) -> Self {
    Self::from_bytes((amount * 1024.) as usize)
  }

  #[inline(always)]
  pub fn from_megabytes(amount: f32) -> Self {
    Self::from_kilobytes(amount * 1024.)
  }

  #[inline(always)]
  pub fn from_gigabytes(amount: f32) -> Self {
    Self::from_megabytes(amount * 1024.)
  }

  #[inline(always)]
  pub fn as_bytes(&self) -> usize {
    self.0
  }

  #[inline(always)]
  pub fn as_kilobytes(&self) -> f32 {
    self.as_bytes() as f32 / 1024.
  }

  #[inline(always)]
  pub fn as_megabytes(&self) -> f32 {
    self.as_kilobytes() / 1024.
  }

  #[inline(always)]
  pub fn as_gigabytes(&self) -> f32 {
    self.as_megabytes() / 1024.
  }
}

impl Add for Size {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    Self::from_bytes(self.as_bytes() + rhs.as_bytes())
  }
}

impl Add<usize> for Size {
  type Output = Self;

  #[inline]
  fn add(self, rhs: usize) -> Self::Output {
    Self::from_bytes(self.as_bytes() + rhs)
  }
}

impl AddAssign for Size {
  #[inline]
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Sub for Size {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    Self::from_bytes(self.as_bytes() - rhs.as_bytes())
  }
}

impl Sub<usize> for Size {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: usize) -> Self::Output {
    Self::from_bytes(self.as_bytes() - rhs)
  }
}

impl SubAssign for Size {
  #[inline]
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl Mul for Size {
  type Output = Self;

  #[inline]
  fn mul(self, rhs: Self) -> Self::Output {
    Self::from_bytes(self.as_bytes() * rhs.as_bytes())
  }
}

impl Mul<usize> for Size {
  type Output = Self;

  #[inline]
  fn mul(self, rhs: usize) -> Self::Output {
    Self::from_bytes(self.as_bytes() * rhs)
  }
}

impl Div for Size {
  type Output = Self;

  #[inline]
  fn div(self, rhs: Self) -> Self::Output {
    Self::from_bytes(self.as_bytes() / rhs.as_bytes())
  }
}

impl Div<usize> for Size {
  type Output = Self;

  #[inline]
  fn div(self, rhs: usize) -> Self::Output {
    Self::from_bytes(self.as_bytes() / rhs)
  }
}

impl Sum for Size {
  #[inline]
  fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
    iter.fold(Self::from_bytes(0), |a, b| a + b)
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

macro_rules! impl_size_from {
  ($type:ty) => {
    impl From<$type> for Size {
      #[inline(always)]
      fn from(bytes: $type) -> Self {
        Self::from_bytes(bytes as usize)
      }
    }
  };
}

impl_size_from!(u8);
impl_size_from!(u16);
impl_size_from!(u32);
impl_size_from!(u64);
impl_size_from!(u128);
impl_size_from!(usize);

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
