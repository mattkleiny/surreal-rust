//! A lightweight audio system.

/// An abstraction over the audio device for the system.
pub trait AudioDevice {
  /// Plays the given clip on the device.
  fn play<A>(&mut self, audio_clip: A);
}
