//! A lightweight audio system.

/// An abstraction over the audio device for the system.
pub trait AudioDevice {
  /// Plays the given clip on the device.
  fn play<A>(&mut self, audio_clip: &AudioClip);
}

/// Identifies an audio clip uniquely.
struct AudioId(usize);

/// Represents an audio clip that can be played.
pub struct AudioClip {
  clip_id: AudioId,
  volume: f32,
}