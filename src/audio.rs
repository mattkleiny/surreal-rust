//! A lightweight audio system.

/// An abstraction over the audio device for the system.
pub trait AudioDevice {
  /// Plays the given clip on the device.
  fn play(&mut self, clip: &AudioClip);
}

/// An audio clip that can be played on an audio device.
pub struct AudioClip {
  id: AudioClipId,
}

/// Represents uniquely some audio clip in the system.
struct AudioClipId(usize);

/// The possible state of an audio clip resource.
#[derive(Copy, Clone, Debug)]
pub enum AudioClipStatus {
  Ready,
  Loading,
}
