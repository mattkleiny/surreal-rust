use std::fmt::Display;

use super::{to_degrees, to_radians};

/// A trait for converting between [`Degrees`] and [`Radians`].
pub trait Angle {
  /// Converts the angle to [`Degrees`].
  fn to_degrees(self) -> Degrees;

  /// Converts the angle to [`Radians`].
  fn to_radians(self) -> Radians;
}

impl Angle for f32 {
  #[inline]
  fn to_degrees(self) -> Degrees {
    Degrees(self as f64)
  }

  #[inline]
  fn to_radians(self) -> Radians {
    Radians(self as f64)
  }
}

impl Angle for f64 {
  #[inline]
  fn to_degrees(self) -> Degrees {
    Degrees(self)
  }

  #[inline]
  fn to_radians(self) -> Radians {
    Radians(self)
  }
}

/// A representation of an angle in radians.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Radians(f64);

impl Radians {
  pub const ZERO: Radians = Radians(0.0);
  pub const _2_PI: Radians = Radians(std::f64::consts::PI * 2.0);
  pub const _PI: Radians = Radians(std::f64::consts::PI);
  pub const _PI_2: Radians = Radians(std::f64::consts::PI / 2.0);
  pub const _PI_4: Radians = Radians(std::f64::consts::PI / 4.0);
}

impl Angle for Radians {
  #[inline]
  fn to_degrees(self) -> Degrees {
    Degrees(to_degrees(self.0))
  }

  #[inline]
  fn to_radians(self) -> Radians {
    self
  }
}

impl From<Degrees> for Radians {
  #[inline(always)]
  fn from(value: Degrees) -> Self {
    Self(to_radians(value.0))
  }
}

impl Display for Radians {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "{}rad", self.0)
  }
}

/// A representation of an angle in degrees.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Degrees(f64);

impl Degrees {
  pub const ZERO: Degrees = Degrees(0.0);
  pub const _45: Degrees = Degrees(45.0);
  pub const _90: Degrees = Degrees(90.0);
  pub const _180: Degrees = Degrees(180.0);
  pub const _360: Degrees = Degrees(360.0);
}

impl Angle for Degrees {
  #[inline]
  fn to_degrees(self) -> Degrees {
    self
  }

  #[inline]
  fn to_radians(self) -> Radians {
    Radians(to_radians(self.0))
  }
}

impl From<Radians> for Degrees {
  #[inline(always)]
  fn from(value: Radians) -> Self {
    Self(to_degrees(value.0))
  }
}

impl Display for Degrees {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "{}°", self.0)
  }
}

macro_rules! impl_operations {
  ($type:ty) => {
    impl From<f64> for $type {
      #[inline(always)]
      fn from(value: f64) -> Self {
        Self(value)
      }
    }

    impl From<f32> for $type {
      #[inline(always)]
      fn from(value: f32) -> Self {
        Self(value as f64)
      }
    }

    impl From<$type> for f64 {
      #[inline(always)]
      fn from(value: $type) -> Self {
        value.0
      }
    }

    impl From<$type> for f32 {
      #[inline(always)]
      fn from(value: $type) -> Self {
        value.0 as f32
      }
    }

    impl std::ops::Add<$type> for $type {
      type Output = $type;

      #[inline]
      fn add(self, rhs: $type) -> Self::Output {
        Self(self.0 + rhs.0)
      }
    }

    impl std::ops::AddAssign<$type> for $type {
      #[inline]
      fn add_assign(&mut self, rhs: $type) {
        self.0 += rhs.0;
      }
    }

    impl std::ops::Sub<$type> for $type {
      type Output = $type;

      #[inline]
      fn sub(self, rhs: $type) -> Self::Output {
        Self(self.0 - rhs.0)
      }
    }

    impl std::ops::SubAssign<$type> for $type {
      #[inline]
      fn sub_assign(&mut self, rhs: $type) {
        self.0 -= rhs.0;
      }
    }

    impl std::ops::Mul<$type> for $type {
      type Output = $type;

      #[inline]
      fn mul(self, rhs: $type) -> Self::Output {
        Self(self.0 * rhs.0)
      }
    }

    impl std::ops::MulAssign<$type> for $type {
      #[inline]
      fn mul_assign(&mut self, rhs: $type) {
        self.0 *= rhs.0;
      }
    }

    impl std::ops::Div<$type> for $type {
      type Output = $type;

      #[inline]
      fn div(self, rhs: $type) -> Self::Output {
        Self(self.0 / rhs.0)
      }
    }

    impl std::ops::DivAssign<$type> for $type {
      #[inline]
      fn div_assign(&mut self, rhs: $type) {
        self.0 /= rhs.0;
      }
    }

    impl std::ops::Rem<$type> for $type {
      type Output = $type;

      #[inline]
      fn rem(self, rhs: $type) -> Self::Output {
        Self(self.0 % rhs.0)
      }
    }

    impl std::ops::RemAssign<$type> for $type {
      #[inline]
      fn rem_assign(&mut self, rhs: $type) {
        self.0 %= rhs.0;
      }
    }

    impl std::ops::Neg for $type {
      type Output = $type;

      #[inline]
      fn neg(self) -> Self::Output {
        Self(-self.0)
      }
    }

    impl std::ops::Add<f64> for $type {
      type Output = $type;

      #[inline]
      fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs)
      }
    }

    impl std::ops::AddAssign<f64> for $type {
      #[inline]
      fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
      }
    }

    impl std::ops::Sub<f64> for $type {
      type Output = $type;

      #[inline]
      fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs)
      }
    }

    impl std::ops::SubAssign<f64> for $type {
      #[inline]
      fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
      }
    }

    impl std::ops::Mul<f64> for $type {
      type Output = $type;

      #[inline]
      fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
      }
    }

    impl std::ops::MulAssign<f64> for $type {
      #[inline]
      fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
      }
    }

    impl std::ops::Div<f64> for $type {
      type Output = $type;

      #[inline]
      fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
      }
    }

    impl std::ops::DivAssign<f64> for $type {
      #[inline]
      fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
      }
    }

    impl std::ops::Rem<f64> for $type {
      type Output = $type;

      #[inline]
      fn rem(self, rhs: f64) -> Self::Output {
        Self(self.0 % rhs)
      }
    }

    impl std::ops::RemAssign<f64> for $type {
      #[inline]
      fn rem_assign(&mut self, rhs: f64) {
        self.0 %= rhs;
      }
    }
  };
}

impl_operations!(Degrees);
impl_operations!(Radians);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn angles_are_converted_to_degrees_from_radians() {
    let angle = 360.0.to_degrees();
    let radians = angle.to_radians();

    assert_eq!(radians, Radians::_2_PI);
  }

  #[test]
  fn angles_are_converted_to_radians_from_degrees() {
    let angle = Radians::_2_PI;
    let degrees = angle.to_degrees();

    assert_eq!(degrees, Degrees(360.0));
  }

  #[test]
  fn degrees_should_support_basic_arithmetic() {
    let angle1 = 90.0.to_degrees();
    let angle2 = 180.0.to_degrees();

    assert_eq!(angle1 + angle2, Degrees(270.0));
    assert_eq!(angle1 - angle2, Degrees(-90.0));
    assert_eq!(angle1 * angle2, Degrees(16200.0));
    assert_eq!(angle1 / angle2, Degrees(0.5));
    assert_eq!(angle1 % angle2, Degrees(90.0));
  }

  #[test]
  fn radians_should_support_basic_arithmetic() {
    let angle1 = 0.5.to_radians();
    let angle2 = 2.0.to_radians();

    assert_eq!(angle1 + angle2, Radians(2.5));
    assert_eq!(angle1 - angle2, Radians(-1.5));
    assert_eq!(angle1 * angle2, Radians(1.0));
    assert_eq!(angle1 / angle2, Radians(0.25));
    assert_eq!(angle1 % angle2, Radians(0.5));
  }
}
