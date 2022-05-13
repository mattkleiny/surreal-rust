use std::time::Instant;

use crate::collections::RingBuffer;
use crate::maths::clamp;

/// Returns the current time, in seconds since the epoch.
pub fn now() -> u64 {
  Instant::now().elapsed().as_secs()
}

/// Contains information on the game's timing state.
#[derive(Copy, Clone, Debug)]
pub struct GameTime {
  pub delta_time: f64,
  pub total_time: f64,
  pub is_running_slowly: bool,
}

/// A simple clock for measuring the time between ticks.
#[derive(Debug)]
pub struct Clock {
  last_time: u64,
  pub max_time: f64,
  pub time_scale: f64,
}

impl Clock {
  pub fn new(max_time: f64) -> Self {
    Self {
      last_time: 0,
      max_time,
      time_scale: 1.,
    }
  }

  /// Ticks the clock by a single frame, returning a time delta.
  pub fn tick(&mut self,  frequency: u64) -> f64 {
    let current_time = now();

    // compute delta time since the last update
    let delta_time = ((current_time - self.last_time) * 1000 / frequency) as f64 / 1000.;
    let clamped_time = clamp(delta_time, 0., self.max_time);

    self.last_time = current_time;

    clamped_time * self.time_scale
  }
}

/// Counts frames per second using a smoothed average.
#[derive(Debug)]
pub struct FrameCounter {
  samples: RingBuffer<f64>,
}

impl FrameCounter {
  pub fn new(samples: usize) -> Self {
    Self {
      samples: RingBuffer::new(samples),
    }
  }

  pub fn tick(&mut self, delta_time: f64) {
    self.samples.append(delta_time);
  }

  pub fn average_frame_time(&self) -> f64 {
    let mut total_frame_time = 0.;

    for sample in &self.samples {
      total_frame_time += sample;
    }

    total_frame_time / self.samples.occupied() as f64
  }

  pub fn fps(&self) -> f64 {
    1. / self.average_frame_time()
  }
}

/// A simple time which ticks on a given basis and returns true if an interval has elapsed.
#[derive(Clone, Debug)]
pub struct IntervalTimer {
  time_elapsed: f64,
  interval_in_secs: f64,
}

impl IntervalTimer {
  pub fn new(interval_in_secs: f64) -> Self {
    Self {
      time_elapsed: 0.,
      interval_in_secs,
    }
  }

  pub fn tick(&mut self, delta_time: f64) -> bool {
    self.time_elapsed += delta_time;
    self.time_elapsed >= self.interval_in_secs
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TimeSpan {
  offset: u64,
}

impl TimeSpan {
  pub const fn from_milliseconds(milliseconds: f32) -> TimeSpan { todo!() }
  pub const fn from_seconds(seconds: f32) -> TimeSpan { todo!() }
  pub const fn from_minutes(minutes: f32) -> TimeSpan { todo!() }
  pub const fn from_hours(hours: f32) -> TimeSpan { todo!() }
  pub const fn from_days(days: f32) -> TimeSpan { todo!() }

  pub fn total_milliseconds(&self) -> f32 { todo!() }
  pub fn total_seconds(&self) -> f32 { todo!() }
  pub fn total_minutes(&self) -> f32 { todo!() }
  pub fn total_hours(&self) -> f32 { todo!() }
  pub fn total_days(&self) -> f32 { todo!() }
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
    let mut timer = IntervalTimer::new(1.);

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
