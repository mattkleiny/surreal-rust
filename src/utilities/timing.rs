//! General utilities related to time.

use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use std::time::{Duration, Instant};

use crate::collections::RingBuffer;

/// A high resolution timestamp that can be used to calculate intervals.
#[derive(Copy, Clone, Debug)]
pub struct TimeStamp {
  instant: Instant,
}

impl TimeStamp {
  /// Creates a new timestamp from the current time.
  pub fn now() -> Self {
    TimeStamp {
      instant: Instant::now(),
    }
  }
}

impl Sub for TimeStamp {
  type Output = TimeSpan;

  fn sub(self, rhs: Self) -> Self::Output {
    TimeSpan::from_duration(self.instant.duration_since(rhs.instant))
  }
}

/// A simple clock for measuring the time between ticks.
#[derive(Debug)]
pub struct Clock {
  start_time: TimeStamp,
  last_time: TimeStamp,
}

impl Default for Clock {
  fn default() -> Self {
    Self::new()
  }
}

impl Clock {
  /// Creates a new clock.
  pub fn new() -> Self {
    Self {
      start_time: TimeStamp::now(),
      last_time: TimeStamp::now(),
    }
  }

  /// Ticks the clock by a single frame, returning a time delta in seconds.
  pub fn tick(&mut self) -> f32 {
    let current_time = TimeStamp::now();
    let delta_time = current_time - self.last_time;

    self.last_time = current_time;

    delta_time.total_seconds()
  }

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
    self.samples.append(delta_time);
  }

  pub fn average_frame_time(&self) -> f32 {
    let mut total_frame_time = 0.;

    for sample in &self.samples {
      total_frame_time += sample;
    }

    total_frame_time / self.samples.occupied() as f32
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
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct TimeSpan {
  milliseconds: f32,
}

impl TimeSpan {
  pub fn from_duration(duration: Duration) -> TimeSpan {
    Self::from_millis(duration.as_nanos() as f32 / (1000. * 1000.))
  }

  pub fn from_millis(milliseconds: f32) -> TimeSpan {
    Self { milliseconds }
  }

  pub fn from_seconds(seconds: f32) -> TimeSpan {
    Self::from_millis(seconds * 1000.)
  }

  pub fn from_minutes(minutes: f32) -> TimeSpan {
    Self::from_seconds(minutes * 60.)
  }

  pub fn from_hours(hours: f32) -> TimeSpan {
    Self::from_minutes(hours * 60.)
  }

  pub fn from_days(days: f32) -> TimeSpan {
    Self::from_hours(days * 24.)
  }

  pub fn total_duration(&self) -> Duration {
    Duration::from_micros((self.milliseconds * 1000.) as u64)
  }

  pub fn total_millis(&self) -> f32 {
    self.milliseconds
  }

  pub fn total_seconds(&self) -> f32 {
    self.total_millis() / 1000.
  }

  pub fn total_minutes(&self) -> f32 {
    self.total_seconds() / 60.
  }

  pub fn total_hours(&self) -> f32 {
    self.total_minutes() / 60.
  }

  pub fn total_days(&self) -> f32 {
    self.total_hours() / 24.
  }
}

impl Add for TimeSpan {
  type Output = TimeSpan;

  fn add(self, rhs: Self) -> Self::Output {
    TimeSpan::from_millis(self.milliseconds + rhs.milliseconds)
  }
}

impl Sub for TimeSpan {
  type Output = TimeSpan;

  fn sub(self, rhs: Self) -> Self::Output {
    TimeSpan::from_millis(self.milliseconds - rhs.milliseconds)
  }
}

impl Mul<f32> for TimeSpan {
  type Output = TimeSpan;

  fn mul(self, rhs: f32) -> Self::Output {
    TimeSpan::from_millis(self.milliseconds * rhs)
  }
}

impl Div<f32> for TimeSpan {
  type Output = TimeSpan;

  fn div(self, rhs: f32) -> Self::Output {
    TimeSpan::from_millis(self.milliseconds / rhs)
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
