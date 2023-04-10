//! Audio engine for Surreal.

use surreal::utilities::{Size, TimeSpan};

mod headless;
mod rodio;

surreal::impl_server!(AudioEngine, AudioBackend);

surreal::impl_rid!(AudioClipId);
surreal::impl_rid!(AudioSourceId);

impl AudioEngine {
  /// Creates a new [`AudioServer`] with a no-op, headless backend.
  pub fn create_headless() -> Self {
    Self::new(headless::HeadlessAudioBackend::default())
  }

  /// Creates a new [`AudioServer`] with a Rodio backend.
  pub fn create_rodio() -> Self {
    Self::new(rodio::RodioAudioBackend::default())
  }
}

/// Describes sampling rates for an audio clip.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AudioSampleRate {
  pub frequency: u16,
  pub channels: u8,
  pub bits_per_sample: u8,
}

impl AudioSampleRate {
  /// A standard-purpose sampling rate.
  pub const STANDARD: Self = Self {
    frequency: 44_000,
    channels: 2,
    bits_per_sample: 16,
  };

  /// Calculates the bits per second for this sample rate.
  pub fn bits_per_second(&self) -> u16 {
    self.frequency * self.channels as u16 * self.bits_per_sample as u16
  }

  /// Calculates the bytes per second for this sample rate.
  pub fn bytes_per_second(&self) -> f32 {
    self.bits_per_second() as f32 / 8.0
  }

  /// Calculates the `Size` required for the given duration at this sample rate.
  pub fn calculate_size(&self, duration: TimeSpan) -> Size {
    Size::from_bytes((duration.total_seconds() * self.bytes_per_second()).ceil() as usize)
  }
}

/// An abstraction over the host capable of running audio.
pub trait AudioHost {}

/// Represents a backend implementation for the underlying audio API.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide
/// away implementation details. The server is intended to be a low-level
/// implementation abstraction.
pub trait AudioBackend {
  // clips
  fn clip_create(&self) -> AudioClipId;
  fn clip_write_data(&self, clip: AudioClipId, data: *const u8, length: usize);
  fn clip_delete(&self, clip: AudioClipId);

  // sources
  fn source_create(&self) -> AudioSourceId;
  fn source_is_playing(&self, source: AudioSourceId) -> bool;
  fn source_get_volume(&self, source: AudioSourceId) -> f32;
  fn source_set_volume(&self, source: AudioSourceId, volume: f32);
  fn source_get_clip(&self, source: AudioSourceId) -> Option<AudioClipId>;
  fn source_set_clip(&self, source: AudioSourceId, clip: AudioClipId);
  fn source_play(&self, source: AudioSourceId);
  fn source_delete(&self, source: AudioSourceId);
}
