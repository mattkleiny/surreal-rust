use common::{InputStream, ToVirtualPath};

use super::*;

/// Represents an audio clip.
///
/// An audio clip is a piece of audio data that can be played back.
/// Playback is controlled by an `AudioSource`.
pub struct AudioClip {
  clip_id: ClipId,
}

impl AudioClip {
  /// Creates a new audio clip.
  pub fn new() -> Self {
    Self {
      clip_id: audio().clip_create().unwrap(),
    }
  }

  /// Creates a new audio clip from the given WAV file.
  pub fn from_wav_path(path: &impl ToVirtualPath) -> Result<Self, ClipError> {
    let path = path.to_virtual_path();
    let stream = path.open_input_stream().map_err(|_| ClipError::FailedToCreate)?;
    let bytes = stream.to_buffer().map_err(|_| ClipError::FailedToCreate)?;

    Self::from_wav_bytes(&bytes)
  }

  /// Creates a new audio clip from the given raw WAV data.
  pub fn from_wav_bytes(_data: &[u8]) -> Result<Self, ClipError> {
    todo!()
  }

  /// Returns the ID of this clip.
  pub fn id(&self) -> ClipId {
    self.clip_id
  }
}

impl Drop for AudioClip {
  fn drop(&mut self) {
    audio().clip_delete(self.clip_id).unwrap();
  }
}
