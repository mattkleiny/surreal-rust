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

#[cfg(feature = "serde")]
impl serde::Serialize for TimeStamp {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_f64(self.0.elapsed().as_secs_f64())
  }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TimeStamp {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    let seconds = f64::deserialize(deserializer)?;

    Ok(TimeStamp(Instant::now() - std::time::Duration::from_secs_f64(seconds)))
  }
}
