use super::*;

/// Represents an audio source.
///
/// An audio source is a point in 3D space that emits audio,
/// and can be controlled to play back `AudioClip`s.
pub struct AudioSource {
  id: SourceId,
}

impl AudioSource {
  /// Creates a new audio source.
  pub fn new() -> Self {
    Self {
      id: audio().source_create().unwrap(),
    }
  }

  /// Returns the ID of this source.
  pub fn id(&self) -> SourceId {
    self.id
  }

  /// Returns whether this source is currently playing.
  pub fn is_playing(&self) -> bool {
    audio().source_is_playing(self.id).unwrap_or_default()
  }

  /// Returns the volume of this source.
  pub fn volume(&self) -> f32 {
    audio().source_get_volume(self.id).unwrap_or_default()
  }

  /// Sets the volume of this source.
  pub fn set_volume(&mut self, volume: f32) {
    audio().source_set_volume(self.id, volume).unwrap();
  }

  /// Plays the given audio clip on this source.
  pub fn play(&mut self, clip: &AudioClip) {
    audio().source_set_clip(self.id, clip.id()).unwrap();
    audio().source_play(self.id).unwrap()
  }
}

impl Drop for AudioSource {
  fn drop(&mut self) {
    audio().source_delete(self.id).unwrap();
  }
}
