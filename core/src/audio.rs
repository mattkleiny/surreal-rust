//! A lightweight cross-platform audio engine.
//!
//! This engine is a light abstraction on top of OpenAL; it offers basic lifecycle management
//! of common OpenAL primitives (buffers, sounds, sources); these primitives are backed by
//! a particular `AudioBackend` implementation, which allows us to gracefully swap the internal
//! audio implementation through a single dynamic pointer.

pub use clips::*;
pub use headless::*;
pub use openal::*;
pub use sources::*;

use crate::utilities::{Size, TimeSpan};

mod clips;
mod headless;
mod openal;
mod sources;

/// An opaque handle to a resource in the audio system.
pub type AudioHandle = u32;

/// A pointer to the core [`AudioBackend`] implementation.
///
/// This pointer is safe to pass around the application.
pub type AudioServer = std::rc::Rc<Box<dyn AudioBackend>>;

/// Represents a resource that possesses an `AudioHandle`.
pub trait AudioResource {
  fn handle(&self) -> AudioHandle;
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

  /// Calculates the total `Size` required for the given duration at this sample rate.
  pub fn calculate_size(&self, duration: TimeSpan) -> Size {
    Size::from_bytes((duration.total_seconds() * self.bytes_per_second()).ceil() as usize)
  }
}

/// Represents a backend implementation for the underlying audio API.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide away implementation
/// details. The server is intended to be a low-level implementation abstraction.
///
/// Theoretically different backends could be supported; though it's unlikely to be anything other
/// than OpenAL. We do provide a headless backend to facilitate testing and related, however.
pub trait AudioBackend {
  // clips
  fn create_clip(&self) -> AudioHandle;
  fn upload_clip_data(&self, clip: AudioHandle, data: *const u8, length: usize);
  fn delete_clip(&self, clip: AudioHandle);

  // sources
  fn create_source(&self) -> AudioHandle;
  fn is_source_playing(&self, source: AudioHandle) -> bool;
  fn get_source_volume(&self, source: AudioHandle) -> f32;
  fn set_source_volume(&self, source: AudioHandle, volume: f32);
  fn delete_source(&self, source: AudioHandle);
}
