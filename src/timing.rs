//! A set of utilities for timing and synchronization.

use crate::collections::RingBuffer;

/// A representation of the time difference between frames.
pub type DeltaTime = f32;

#[derive(Clone, Debug)]
pub struct Clock {
  last_time: u64,
  current_time: u64,
  max_time: f32,
}

impl Clock {
  pub fn new(max_time: f32) -> Self {
    Self {
      last_time: 0,
      current_time: 0,
      max_time,
    }
  }

  /// Ticks the clock by a single frame, returning a time delta.
  pub fn tick(&mut self, current_time: u64, frequency: u64) -> DeltaTime {
    self.last_time = self.current_time;
    self.current_time = current_time;

    // compute delta time since the last update
    let delta_time = ((self.current_time - self.last_time) * 1000 / frequency) as f32 / 1000.;

    if delta_time > self.max_time {
      self.max_time
    } else {
      delta_time
    }
  }
}

/// Counts frames per second using a smoothed average.
#[derive(Clone, Debug)]
pub struct FpsCounter {
  samples: RingBuffer<f32>,
}

impl FpsCounter {
  pub fn new(samples: usize) -> Self {
    Self {
      samples: RingBuffer::new(samples),
    }
  }

  /// Advances the counter by the given delta amount.
  pub fn tick(&mut self, delta_time: f32) {
    self.samples.append(delta_time);
  }

  /// Returns the average frame time.
  pub fn average_frame_time(&self) -> f32 {
    let mut total_frame_time = 0.;

    for sample in &self.samples {
      total_frame_time += sample;
    }

    total_frame_time / self.samples.occupied() as f32
  }

  /// Returns the current measurement of FPS.
  pub fn fps(&self) -> f32 {
    1. / self.average_frame_time()
  }
}

/// A simple time which ticks on a given basis and returns true if an interval has elapsed.
#[derive(Clone, Debug)]
pub struct IntervalTimer {
  time_elapsed: f32,
  interval_in_secs: f32,
}

impl IntervalTimer {
  pub fn new(interval_in_secs: f32) -> Self {
    Self {
      time_elapsed: 0.,
      interval_in_secs,
    }
  }

  pub fn tick(&mut self, delta_time: DeltaTime) -> bool {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn clock_should_compute_delta_with_previous_frame() {
    let mut clock = Clock::new(32.);

    let delta1 = clock.tick(10000, 60);
    let delta2 = clock.tick(10010, 60);

    assert_ne!(delta1, delta2);
  }

  #[test]
  fn fps_counter_should_accumulate_over_time() {
    let mut counter = FpsCounter::new(100);

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
