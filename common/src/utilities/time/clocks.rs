use super::TimeStamp;

/// A simple clock for measuring the time between ticks.
#[derive(Debug)]
pub struct DeltaClock {
  start_time: TimeStamp,
  last_time: TimeStamp,
  last_delta_time: f32,
  max_delta_time: f32,
}

impl Default for DeltaClock {
  fn default() -> Self {
    Self::new()
  }
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
