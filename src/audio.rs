//! A lightweight audio system.

use std::sync::Arc;

/// Identifies an audio clip uniquely.
pub struct AudioClipID(usize);

/// Represents an audio clip that can be played.
pub struct AudioClip {
  pub id: Arc<AudioClipID>,
  pub volume: f32,
}

/// Defines the status of an audio clip resource.
#[derive(Copy, Clone, Debug)]
pub enum AudioClipStatus {
  Unknown,
  Loading,
  Ready,
  Playing,
  Unloading
}

/// An abstraction over the audio device for the system.
pub trait AudioDevice {
  /// Gets the status of the given audio clip.
  fn get_status(&self, id: AudioClipID) -> AudioClipStatus;

  /// Plays the given clip on the device.
  fn play(&mut self, audio_clip: &AudioClip);
}
