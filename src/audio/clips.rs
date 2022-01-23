use super::*;

/// Represents an audio clip backed by an audio device.
///
/// Clips can be played on a corresponding `AudioSource`.
pub struct AudioClip {
  handle: AudioHandle,
}

impl AudioClip {
  /// Creates a new empty clip.
  pub fn new(server: &impl AudioServer) -> Self {
    Self {
      handle: server.create_clip()
    }
  }

  /// Uploads raw data to the audio clip.
  pub fn upload(&mut self, raw_waveform: &[u8]) {
    unimplemented!()
  }
}
