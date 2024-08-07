use std::{ops::Sub, time::Instant};

use super::TimeSpan;

/// A high resolution timestamp that can be used to calculate intervals.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct TimeStamp(Instant);

impl TimeStamp {
  /// Creates a new timestamp from the current time.
  #[inline]
  pub fn now() -> Self {
    TimeStamp(Instant::now())
  }
}

impl Sub for TimeStamp {
  type Output = TimeSpan;

  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    TimeSpan::from(self.0.duration_since(rhs.0))
  }
}
