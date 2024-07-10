use super::*;

/// Represents an audio clip.
///
/// An audio clip is a piece of audio data that can be played back.
/// Playback is controlled by an `AudioSource`.
pub struct AudioClip {
  id: ClipId,
}

impl AudioClip {
  /// Creates a new audio clip.
  pub fn new() -> Self {
    Self {
      id: audio().clip_create().unwrap(),
    }
  }

  /// Returns the ID of this clip.
  pub fn id(&self) -> ClipId {
    self.id
  }

  /// Writes the given audio data to this clip.
  pub fn write_data(&mut self, data: &[u8]) {
    audio().clip_write_data(self.id, data.as_ptr(), data.len()).unwrap();
  }
}

impl Drop for AudioClip {
  fn drop(&mut self) {
    audio().clip_delete(self.id).unwrap();
  }
}
