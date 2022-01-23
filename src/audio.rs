//! A lightweight and fast cross-platform audio engine.

pub use clips::*;

use crate::utilities::{Size, TimeSpan};

mod clips;

/// Represents a fallible result in the audio subsystem.
pub type AudioResult<T> = anyhow::Result<T>;

/// An opaque handle to an underlying resource in the `AudioServer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AudioHandle(u64);

/// A server for the underlying audio subsystem.
///
/// Permits interaction with the underlying audio API through unsafe lower-level abstraction.
pub unsafe trait AudioServer {
  // clips
  fn create_clip(&self) -> AudioHandle;
  fn upload_clip_data<T>(&self, handle: AudioHandle, data: &[T]);
  fn delete_clip(&self, handle: AudioHandle);
}

/// Describes sampling rates for an audio clip.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AudioSampleRate {
  pub frequency: u16,
  pub channels: u8,
  pub bits_per_sample: u8,
}

impl AudioSampleRate {
  /// A standard-purpose sampling rate, good for most situations.
  pub const STANDARD: Self = Self { frequency: 44_000, channels: 2, bits_per_sample: 16 };

  fn bits_per_second(&self) -> u16 {
    self.frequency * self.channels as u16 * self.bits_per_sample as u16
  }

  fn bytes_per_second(&self) -> f32 {
    self.bits_per_second() as f32 / 8.0
  }

  /// Calculates the total `Size` required for the given duration at this sample rate.
  fn calculate_size(&self, duration: TimeSpan) -> Size {
    Size::from_bytes((duration.total_seconds() * self.bytes_per_second()).ceil() as usize)
  }
}
