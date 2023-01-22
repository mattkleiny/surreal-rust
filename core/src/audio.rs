//! A lightweight cross-platform audio engine.

use crate::utilities::{Size, TimeSpan};

mod headless;
mod rodio;

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

/// A wrapper for the core [`AudioBackend`] implementation.
#[derive(Clone)]
pub struct AudioServer {
  backend: std::sync::Arc<Box<dyn AudioBackend>>,
}

impl AudioServer {
  /// Creates a new [`AudioServer`] with a [`HeadlessAudioBackend`].
  pub fn headless() -> Self {
    Self::new(headless::HeadlessAudioBackend::default())
  }

  /// Creates a new [`AudioServer`] for the given [`AudioBackend`].
  pub fn new(backend: impl AudioBackend + 'static) -> Self {
    Self {
      backend: std::sync::Arc::new(Box::new(backend)),
    }
  }
}

unsafe impl Send for AudioServer {}
unsafe impl Sync for AudioServer {}

impl std::ops::Deref for AudioServer {
  type Target = Box<dyn AudioBackend>;

  fn deref(&self) -> &Self::Target {
    self.backend.as_ref()
  }
}

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
  fn source_delete(&self, source: AudioSourceId);
}

crate::impl_rid!(AudioClipId);
crate::impl_rid!(AudioSourceId);
