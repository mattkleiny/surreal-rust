use super::*;

/// A buffer of audio data.
pub struct AudioBuffer {
  buffer_id: BufferId,
}

impl AudioBuffer {
  /// Creates a new audio buffer.
  pub fn new() -> Self {
    Self {
      buffer_id: audio().buffer_create().unwrap(),
    }
  }
}

impl Drop for AudioBuffer {
  fn drop(&mut self) {
    audio().buffer_delete(self.buffer_id).unwrap();
  }
}
