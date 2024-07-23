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

  /// Gets the ID of this source.
  pub fn id(&self) -> SourceId {
    self.id
  }

  /// Gets the position of this source.
  pub fn position(&self) -> Vec3 {
    audio().source_get_position(self.id).unwrap_or_default()
  }

  /// Sets the position of this source.
  pub fn set_position(&mut self, position: Vec3) {
    audio().source_set_position(self.id, position).unwrap();
  }

  /// Gets the velocity of this source.
  pub fn velocity(&self) -> Vec3 {
    audio().source_get_velocity(self.id).unwrap_or_default()
  }

  /// Sets the velocity of this source.
  pub fn set_velocity(&mut self, velocity: Vec3) {
    audio().source_set_velocity(self.id, velocity).unwrap();
  }

  /// Gets the volume of this source.
  pub fn gain(&self) -> f32 {
    audio().source_get_gain(self.id).unwrap_or_default()
  }

  /// Sets the gain of this source.
  pub fn set_gain(&mut self, gain: f32) {
    audio().source_set_gain(self.id, gain).unwrap();
  }

  /// Gets the pitch of this source.
  pub fn pitch(&self) -> f32 {
    audio().source_get_pitch(self.id).unwrap_or_default()
  }

  /// Sets the pitch of this source.
  pub fn set_pitch(&mut self, pitch: f32) {
    audio().source_set_pitch(self.id, pitch).unwrap();
  }

  /// Determines whether this source is looping.
  pub fn is_looping(&self) -> bool {
    audio().source_is_looping(self.id).unwrap_or_default()
  }

  /// Sets whether this source is looping.
  pub fn set_looping(&mut self, looping: bool) {
    audio().source_set_looping(self.id, looping).unwrap();
  }

  /// Gets whether this source is currently playing.
  pub fn is_playing(&self) -> bool {
    audio().source_is_playing(self.id).unwrap_or_default()
  }

  /// Plays this source.
  pub fn play(&mut self) {
    audio().source_play(self.id).unwrap();
  }

  /// Plays the given audio clip on this source with the current loop setting.
  pub fn play_clip(&mut self, clip: &AudioClip) {
    audio().source_set_clip(self.id, clip.id()).unwrap();
    audio().source_play(self.id).unwrap();
  }

  /// Plays the given audio clip on this source once-off.
  pub fn play_once(&mut self, clip: &AudioClip) {
    self.set_looping(false);

    audio().source_set_clip(self.id, clip.id()).unwrap();
    audio().source_play(self.id).unwrap()
  }

  /// Plays the given audio clip on this source in a loop.
  pub fn play_looping(&mut self, clip: &AudioClip) {
    self.set_looping(true);

    audio().source_set_clip(self.id, clip.id()).unwrap();
    audio().source_play(self.id).unwrap()
  }
}

impl Drop for AudioSource {
  fn drop(&mut self) {
    audio().source_delete(self.id).unwrap();
  }
}
