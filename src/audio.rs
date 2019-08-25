//! A lightweight audio system.

pub use mixing::*;
pub use music::*;
pub use sound::*;

mod mixing;
mod music;
mod sound;

/// An abstraction over the audio device for the system.
pub trait AudioDevice {
  /// Plays the given clip on the device.
  fn play(&mut self, clip: &SoundClip);
}

/// Represents uniquely some audio clip in the system.
struct AudioClipId(usize);

/// The possible state of an audio clip resource.
#[derive(Copy, Clone, Debug)]
pub enum AudioClipStatus {
  Ready,
  Loading,
}
