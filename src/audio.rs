//! A lightweight cross-platform audio engine.

use crate::utilities::{Size, TimeSpan};

/// Represents a fallible result in the audio subsystem.
pub type AudioResult<T> = anyhow::Result<T>;

/// The audio server implementation.
pub type AudioServer<A> = std::rc::Rc<A>;

/// Represents a server implementation for the underlying audio subsystem.
///
/// Permits interaction with the underlying audio API through unsafe lower-level abstraction.
pub trait AudioImpl {
  type Handle: Copy + std::fmt::Debug;

  // clips
  fn create_clip(&self) -> Self::Handle;
  fn upload_clip_data(&self, handle: Self::Handle, data: &[u8]);
  fn delete_clip(&self, handle: Self::Handle);
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
