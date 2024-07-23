use common::{Size, TimeSpan};

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
    Size::from_bytes((duration.as_seconds() * self.bytes_per_second()).ceil() as usize)
  }
}
