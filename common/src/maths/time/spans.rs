use std::{
  fmt::{Display, Formatter},
  iter::Sum,
  ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
  time::Duration,
};

use crate::{FromBinary, ToBinary};

/// A representation of a span of time.
#[derive(Default, Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimeSpan {
  seconds: f32,
}

impl TimeSpan {
  pub const ZERO: Self = Self { seconds: 0. };

  #[inline]
  pub fn from_millis(milliseconds: f32) -> TimeSpan {
    Self::from_seconds(milliseconds / 1000.)
  }

  #[inline]
  pub fn from_seconds(seconds: f32) -> TimeSpan {
    Self { seconds }
  }

  #[inline]
  pub fn from_minutes(minutes: f32) -> TimeSpan {
    Self::from_seconds(minutes * 60.)
  }

  #[inline]
  pub fn from_hours(hours: f32) -> TimeSpan {
    Self::from_minutes(hours * 60.)
  }

  #[inline]
  pub fn from_days(days: f32) -> TimeSpan {
    Self::from_hours(days * 24.)
  }

  #[inline]
  pub fn total_duration(&self) -> Duration {
    Duration::from_micros((self.seconds * 1000.) as u64)
  }

  #[inline]
  pub fn total_millis(&self) -> f32 {
    self.total_seconds() * 1000.
  }

  #[inline]
  pub fn total_seconds(&self) -> f32 {
    self.seconds
  }

  #[inline]
  pub fn total_minutes(&self) -> f32 {
    self.total_seconds() / 60.
  }

  #[inline]
  pub fn total_hours(&self) -> f32 {
    self.total_minutes() / 60.
  }

  #[inline]
  pub fn total_days(&self) -> f32 {
    self.total_hours() / 24.
  }
}

impl Add for TimeSpan {
  type Output = TimeSpan;

  #[inline]
  fn add(self, rhs: Self) -> Self::Output {
    TimeSpan::from_seconds(self.seconds + rhs.seconds)
  }
}

impl AddAssign for TimeSpan {
  #[inline]
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl Sub for TimeSpan {
  type Output = TimeSpan;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    TimeSpan::from_seconds(self.seconds - rhs.seconds)
  }
}

impl SubAssign for TimeSpan {
  #[inline]
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl Mul<f32> for TimeSpan {
  type Output = TimeSpan;

  #[inline]
  fn mul(self, rhs: f32) -> Self::Output {
    TimeSpan::from_seconds(self.seconds * rhs)
  }
}

impl Div<f32> for TimeSpan {
  type Output = TimeSpan;

  #[inline]
  fn div(self, rhs: f32) -> Self::Output {
    TimeSpan::from_seconds(self.seconds / rhs)
  }
}

impl Sum for TimeSpan {
  #[inline]
  fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
    iter.fold(TimeSpan::from_seconds(0.), |a, b| a + b)
  }
}

impl From<Duration> for TimeSpan {
  #[inline]
  fn from(value: Duration) -> Self {
    Self::from_millis(value.as_nanos() as f32 / (1000. * 1000.))
  }
}

impl From<TimeSpan> for Duration {
  #[inline]
  fn from(value: TimeSpan) -> Self {
    Duration::from_secs_f32(value.seconds)
  }
}

impl Display for TimeSpan {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      _ if self.total_days() > 1. => write!(formatter, "{} days", self.total_days()),
      _ if self.total_hours() > 1. => write!(formatter, "{} hours", self.total_hours()),
      _ if self.total_minutes() > 1. => write!(formatter, "{} minutes", self.total_minutes()),
      _ if self.total_seconds() > 1. => write!(formatter, "{} seconds", self.total_seconds()),
      _ => write!(formatter, "{} milliseconds", self.total_millis()),
    }
  }
}

impl ToBinary for TimeSpan {
  #[inline]
  fn to_binary(&self) -> Vec<u8> {
    self.seconds.to_binary()
  }
}

impl FromBinary for TimeSpan {
  #[inline]
  fn from_binary(bytes: &[u8]) -> Self {
    Self::from_seconds(f32::from_binary(bytes))
  }
}

/// Allows a type to be converted into a [`TimeSpan`].
pub trait IntoTimeSpan {
  /// Creates a [`TimeSpan`] representing milliseconds.
  fn milliseconds(&self) -> TimeSpan;

  /// Creates a [`TimeSpan`] representing seconds.
  fn seconds(&self) -> TimeSpan;

  /// Creates a [`TimeSpan`] representing minutes.
  fn minutes(&self) -> TimeSpan;

  /// Creates a [`TimeSpan`] representing hours.
  fn hours(&self) -> TimeSpan;
}

macro_rules! impl_into_time_span {
  ($type:ty) => {
    impl IntoTimeSpan for $type {
      #[inline]
      fn milliseconds(&self) -> TimeSpan {
        TimeSpan::from_millis(*self as f32)
      }

      #[inline]
      fn seconds(&self) -> TimeSpan {
        TimeSpan::from_seconds(*self as f32)
      }

      #[inline]
      fn minutes(&self) -> TimeSpan {
        TimeSpan::from_minutes(*self as f32)
      }

      #[inline]
      fn hours(&self) -> TimeSpan {
        TimeSpan::from_hours(*self as f32)
      }
    }
  };
}

impl_into_time_span!(u8);
impl_into_time_span!(u16);
impl_into_time_span!(u32);
impl_into_time_span!(u64);
impl_into_time_span!(usize);
impl_into_time_span!(i8);
impl_into_time_span!(i16);
impl_into_time_span!(i32);
impl_into_time_span!(i64);
impl_into_time_span!(isize);
impl_into_time_span!(f32);
impl_into_time_span!(f64);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_time_span_from_millis() {
    let time_span = TimeSpan::from_millis(100.0);

    assert_eq!(time_span.total_millis(), 100.0);
  }

  #[test]
  fn test_time_span_from_seconds() {
    let time_span = TimeSpan::from_seconds(2.5);

    assert_eq!(time_span.total_millis(), 2500.0);
  }

  #[test]
  fn test_time_span_from_minutes() {
    let time_span = TimeSpan::from_minutes(1.5);

    assert_eq!(time_span.total_millis(), 90000.0);
  }

  #[test]
  fn test_time_span_from_hours() {
    let time_span = TimeSpan::from_hours(0.5);

    assert_eq!(time_span.total_millis(), 1800000.0);
  }

  #[test]
  fn test_u8_into_time_span() {
    let value: u8 = 10;
    let time_span = value.milliseconds();

    assert_eq!(time_span.total_millis(), 10.0);
  }
}
