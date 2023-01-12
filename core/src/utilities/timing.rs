//! General utilities related to time.

use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};
use std::time::{Duration, Instant};

use crate::collections::RingBuffer;

/// A high resolution timestamp that can be used to calculate intervals.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct TimeStamp {
  instant: Instant,
}

impl TimeStamp {
  /// Creates a new timestamp from the current time.
  #[inline]
  pub fn now() -> Self {
    TimeStamp { instant: Instant::now() }
  }
}

impl Sub for TimeStamp {
  type Output = TimeSpan;

  #[must_use]
  fn sub(self, rhs: Self) -> Self::Output {
    TimeSpan::from(self.instant.duration_since(rhs.instant))
  }
}

/// A simple clock for measuring the time between ticks.
#[derive(Debug)]
pub struct DeltaClock {
  start_time: TimeStamp,
  last_time: TimeStamp,
  last_delta_time: f32,
  max_delta_time: f32,
}

impl DeltaClock {
  /// Creates a new clock.
  pub fn new() -> Self {
    Self {
      start_time: TimeStamp::now(),
      last_time: TimeStamp::now(),
      last_delta_time: 0.,
      max_delta_time: 0.16 * 2.,
    }
  }

  /// Ticks the clock by a single frame, returning a time delta in seconds.
  pub fn tick(&mut self) -> f32 {
    let current_time = TimeStamp::now();
    let delta_time = current_time - self.last_time;

    self.last_time = current_time;
    self.last_delta_time = delta_time.total_seconds().min(self.max_delta_time);

    self.last_delta_time
  }

  /// The last delta time observed on the [`DeltaClock`], in seconds.
  #[inline]
  pub fn last_delta_time(&self) -> f32 {
    self.last_delta_time
  }

  /// The total time observed since the [`DeltaClock`] was created.
  pub fn total_time(&self) -> f32 {
    let now = TimeStamp::now();

    (now - self.start_time).total_seconds()
  }
}

/// Counts frames per second using a smoothed average.
#[derive(Debug)]
pub struct FrameCounter {
  samples: RingBuffer<f32>,
}

impl FrameCounter {
  pub fn new(samples: usize) -> Self {
    Self {
      samples: RingBuffer::new(samples),
    }
  }

  pub fn tick(&mut self, delta_time: f32) {
    self.samples.push(delta_time);
  }

  pub fn average_frame_time(&self) -> f32 {
    let mut total_frame_time = 0.;

    for sample in &self.samples {
      total_frame_time += sample;
    }

    total_frame_time / self.samples.len() as f32
  }

  pub fn fps(&self) -> f32 {
    1. / self.average_frame_time()
  }
}

/// A simple time which ticks on a given basis and returns true if an interval has elapsed.
#[derive(Clone, Debug)]
pub struct IntervalTimer {
  time_elapsed: f32,
  interval: TimeSpan,
}

impl IntervalTimer {
  pub fn new(interval: TimeSpan) -> Self {
    Self {
      time_elapsed: 0.,
      interval,
    }
  }

  pub fn tick(&mut self, delta_time: f32) -> bool {
    self.time_elapsed += delta_time;
    self.time_elapsed >= self.interval.total_seconds()
  }

  pub fn reset(&mut self) {
    self.time_elapsed = 0.;
  }
}

/// A simple time which ticks on a given basis and returns true if the given number of frames have elapsed.
#[derive(Clone, Debug)]
pub struct FrameTimer {
  frames_elapsed: u64,
  interval_in_frames: u64,
}

impl FrameTimer {
  pub fn new(interval_in_frames: u64) -> Self {
    Self {
      frames_elapsed: 0,
      interval_in_frames,
    }
  }

  pub fn tick(&mut self) -> bool {
    self.frames_elapsed += 1;
    self.frames_elapsed >= self.interval_in_frames
  }

  pub fn reset(&mut self) {
    self.frames_elapsed = 0;
  }
}

/// A representation of a span of time.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct TimeSpan {
  seconds: f32,
}

impl TimeSpan {
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

impl Sub for TimeSpan {
  type Output = TimeSpan;

  #[inline]
  fn sub(self, rhs: Self) -> Self::Output {
    TimeSpan::from_seconds(self.seconds - rhs.seconds)
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
  fn fps_counter_should_accumulate_over_time() {
    let mut counter = FrameCounter::new(100);

    counter.tick(0.016);
    counter.tick(0.016);
    counter.tick(0.016);
    counter.tick(0.016);
    counter.tick(0.016);
  }

  #[test]
  fn interval_timer_should_tick_on_a_fixed_basis() {
    let mut timer = IntervalTimer::new(TimeSpan::from_seconds(1.));

    assert_eq!(false, timer.tick(0.5));
    assert_eq!(true, timer.tick(0.5));

    timer.reset();

    assert_eq!(false, timer.tick(0.5));
  }

  #[test]
  fn frame_timer_should_tick_correctly() {
    let mut timer = FrameTimer::new(3);

    assert_eq!(false, timer.tick());
    assert_eq!(false, timer.tick());
    assert_eq!(true, timer.tick());
  }
}
