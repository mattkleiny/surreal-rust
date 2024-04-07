use super::*;

pub struct AudioSource {
  id: SourceId,
  engine: AudioEngine,
}

impl AudioSource {
  pub fn new(engine: &AudioEngine) -> Self {
    Self {
      id: engine.source_create().unwrap(),
      engine: engine.clone(),
    }
  }

  pub fn id(&self) -> SourceId {
    self.id
  }

  pub fn is_playing(&self) -> bool {
    self.engine.source_is_playing(self.id).unwrap_or_default()
  }

  pub fn volume(&self) -> f32 {
    self.engine.source_get_volume(self.id).unwrap_or_default()
  }

  pub fn set_volume(&mut self, volume: f32) {
    self.engine.source_set_volume(self.id, volume).unwrap();
  }

  pub fn play(&mut self, clip: &AudioClip) {
    self.engine.source_set_clip(self.id, clip.id()).unwrap();
    self.engine.source_play(self.id).unwrap()
  }
}

impl Drop for AudioSource {
  fn drop(&mut self) {
    self.engine.source_delete(self.id).unwrap();
  }
}
