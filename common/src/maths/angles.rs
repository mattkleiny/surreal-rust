use std::fmt::Display;

use bytemuck::{Pod, Zeroable};

use super::{FromRandom, Random};

/// A trait for converting between [`Degrees`] and [`Radians`].
pub trait Angle {
  /// Converts the angle to [`Degrees`].
  fn into_degrees(self) -> Degrees;

  /// Converts the angle to [`Radians`].
  fn into_radians(self) -> Radians;
}

impl Angle for f32 {
  #[inline]
  fn into_degrees(self) -> Degrees {
    Degrees(self as f64)
  }

  #[inline]
  fn into_radians(self) -> Radians {
    Radians(self as f64)
  }
}

impl Angle for f64 {
  #[inline]
  fn into_degrees(self) -> Degrees {
    Degrees(self)
  }

  #[inline]
  fn into_radians(self) -> Radians {
    Radians(self)
  }
}

impl FromRandom for Radians {
  fn from_random(random: &mut Random) -> Self {
    Self(random.next_f64() * std::f64::consts::PI * 2.0)
  }
}

/// A representation of an angle in radians.
#[repr(transparent)]
#[derive(Pod, Zeroable, Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
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
  fn into_degrees(self) -> Degrees {
    Degrees(self.0.to_degrees())
  }

  #[inline]
  fn into_radians(self) -> Radians {
    self
  }
}

impl From<Degrees> for Radians {
  #[inline(always)]
  fn from(value: Degrees) -> Self {
    Self(value.0.to_radians())
  }
}

impl Display for Radians {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "{}rad", self.0)
  }
}

/// A representation of an angle in degrees.
#[repr(transparent)]
#[derive(Pod, Zeroable, Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
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
  fn into_degrees(self) -> Degrees {
    self
  }

  #[inline]
  fn into_radians(self) -> Radians {
    Radians(self.0.to_radians())
  }
}

impl From<Radians> for Degrees {
  #[inline(always)]
  fn from(value: Radians) -> Self {
    Self(value.0.to_degrees())
  }
}

impl FromRandom for Degrees {
  fn from_random(random: &mut Random) -> Self {
    Self(random.next_f64() * 360.0)
  }
}

impl Display for Degrees {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "{}°", self.0)
  }
}

macro_rules! impl_std_ops {
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

impl_std_ops!(Degrees);
impl_std_ops!(Radians);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_converted_to_degrees_from_radians() {
    let angle = 360.0.into_degrees();
    let radians = angle.into_radians();

    assert_eq!(radians, Radians::_2_PI);
  }

  #[test]
  fn test_converted_to_radians_from_degrees() {
    let angle = Radians::_2_PI;
    let degrees = angle.into_degrees();

    assert_eq!(degrees, Degrees(360.0));
  }

  #[test]
  fn test_degrees_should_support_basic_arithmetic() {
    let angle1 = 90.0.into_degrees();
    let angle2 = 180.0.into_degrees();

    assert_eq!(angle1 + angle2, Degrees(270.0));
    assert_eq!(angle1 - angle2, Degrees(-90.0));
    assert_eq!(angle1 * angle2, Degrees(16200.0));
    assert_eq!(angle1 / angle2, Degrees(0.5));
    assert_eq!(angle1 % angle2, Degrees(90.0));
  }

  #[test]
  fn test_radians_should_support_basic_arithmetic() {
    let angle1 = 0.5.into_radians();
    let angle2 = 2.0.into_radians();

    assert_eq!(angle1 + angle2, Radians(2.5));
    assert_eq!(angle1 - angle2, Radians(-1.5));
    assert_eq!(angle1 * angle2, Radians(1.0));
    assert_eq!(angle1 / angle2, Radians(0.25));
    assert_eq!(angle1 % angle2, Radians(0.5));
  }
}
