/// Represents an audio clip backed by an audio device.
///
/// Clips can be played on a corresponding `AudioSource`.
pub struct AudioClip {}

impl AudioClip {
  /// Creates a new empty clip.
  pub fn new() -> Self {
    Self {}
  }

  /// Creates a clip from raw waveform data.
  pub fn create(raw_waveform: &[u8]) -> Self {
    unimplemented!()
  }

  /// Uploads raw data to the audio clip.
  pub fn upload(&mut self, raw_waveform: &[u8]) {
    unimplemented!()
  }
}
