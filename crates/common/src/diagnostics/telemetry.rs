/// Indicates a kind of telemetry.
pub trait Telemetry {
  /// The name of the telemetry.
  fn name(&self) -> &'static str;
}

/// A listener for telemetry updates.
pub trait TelemetryListener {
  /// Called when the telemetry is updated.
  fn on_telemetry_update(&mut self, telemetry: &dyn Telemetry);
}

/// Allows a closure to be used as a telemetry listener.
impl<F: FnMut(&dyn Telemetry)> TelemetryListener for F {
  #[inline(always)]
  fn on_telemetry_update(&mut self, telemetry: &dyn Telemetry) {
    self(telemetry)
  }
}

/// A recorder for telemetry.
#[derive(Default)]
pub struct TelemetryRecorder {
  listeners: Vec<Box<dyn TelemetryListener>>,
}

impl TelemetryRecorder {
  /// Adds a listener to the recorder.
  pub fn add_listener(&mut self, listener: impl TelemetryListener + 'static) {
    self.listeners.push(Box::new(listener));
  }

  /// Removes a listener from the recorder.
  pub fn remove_listener(&mut self, listener: &impl TelemetryListener) {
    self.listeners.retain(|it| !std::ptr::eq(it.as_ref(), listener));
  }

  /// Records the given telemetry.
  pub fn record(&mut self, telemetry: &dyn Telemetry) {
    for listener in &mut self.listeners {
      listener.on_telemetry_update(telemetry);
    }
  }
}

/// Implements [`Telemetry`] for the given type.
#[macro_export]
macro_rules! impl_telemetry {
  ($type:ty, $name:expr) => {
    impl Telemetry for $type {
      fn name(&self) -> &'static str {
        $name
      }
    }
  };
}

pub mod frames {
  //! Frame rate telemetry.
  use super::*;

  pub struct FramesPerSecond(pub u32);
  pub struct FrameTime(pub std::time::Duration);
  pub struct FrameTimeAverage(pub std::time::Duration);
  pub struct FrameTimeMinimum(pub std::time::Duration);
  pub struct FrameTimeMaximum(pub std::time::Duration);

  impl_telemetry!(FramesPerSecond, "frames_per_second");
  impl_telemetry!(FrameTime, "frame_time");
  impl_telemetry!(FrameTimeAverage, "frame_time_average");
  impl_telemetry!(FrameTimeMinimum, "frame_time_minimum");
  impl_telemetry!(FrameTimeMaximum, "frame_time_maximum");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_recorder_operations() {
    let mut recorder = TelemetryRecorder::default();

    recorder.add_listener(|telemetry: &dyn Telemetry| {
      assert_eq!(telemetry.name(), "frames_per_second");
      println!("telemetry: {}", telemetry.name());
    });

    recorder.record(&frames::FramesPerSecond(60));
    recorder.record(&frames::FramesPerSecond(60));
    recorder.record(&frames::FramesPerSecond(60));
    recorder.record(&frames::FramesPerSecond(59));
    recorder.record(&frames::FramesPerSecond(60));
  }
}
