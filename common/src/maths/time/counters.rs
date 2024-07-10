use super::TimeSpan;
use crate::RingBuffer;

/// A simple time which ticks on a given basis and returns true if an interval
/// has elapsed.
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
    self.time_elapsed >= self.interval.as_seconds()
  }

  pub fn reset(&mut self) {
    self.time_elapsed = 0.;
  }
}

/// A simple time which ticks on a given basis and returns true if the given
/// number of frames have elapsed.
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

/// Counts frames per second using a smoothed average.
#[derive(Debug)]
pub struct FrameCounter {
  samples: RingBuffer<f32>,
}

impl Default for FrameCounter {
  fn default() -> Self {
    Self::new(60)
  }
}

impl FrameCounter {
  /// Creates a new frame counter with the given number of samples.
  pub fn new(samples: usize) -> Self {
    Self {
      samples: RingBuffer::new(samples),
    }
  }

  /// Ticks the frame counter with the given delta time.
  pub fn tick(&mut self, delta_time: f32) {
    self.samples.push(delta_time);
  }

  /// The average frame time observed by the frame counter.
  pub fn average_frame_time(&self) -> f32 {
    let mut total_frame_time = 0.;

    for sample in &self.samples {
      total_frame_time += sample;
    }

    total_frame_time / self.samples.len() as f32
  }

  /// The average frames per second observed by the frame counter.
  pub fn average_fps(&self) -> f32 {
    1. / self.average_frame_time()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_interval_timer_should_tick_on_a_fixed_basis() {
    let mut timer = IntervalTimer::new(TimeSpan::from_seconds(1.));

    assert!(!timer.tick(0.5));
    assert!(timer.tick(0.5));

    timer.reset();

    assert!(!timer.tick(0.5));
  }

  #[test]
  fn test_frame_timer_should_tick_correctly() {
    let mut timer = FrameTimer::new(3);

    assert!(!timer.tick());
    assert!(!timer.tick());
    assert!(timer.tick());
  }

  #[test]
  fn test_fps_counter_should_accumulate_over_time() {
    let mut counter = FrameCounter::new(100);

    counter.tick(0.016);
    counter.tick(0.016);
    counter.tick(0.016);
    counter.tick(0.016);
    counter.tick(0.016);
  }
}
