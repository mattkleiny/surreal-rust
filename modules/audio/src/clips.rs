use super::*;

pub struct AudioClip {
  id: ClipId,
  engine: AudioEngine,
}

impl AudioClip {
  pub fn new(engine: &AudioEngine) -> Self {
    Self {
      id: engine.clip_create().unwrap(),
      engine: engine.clone(),
    }
  }

  pub fn id(&self) -> ClipId {
    self.id
  }

  pub fn write_data(&mut self, data: &[u8]) {
    self.engine.clip_write_data(self.id, data.as_ptr(), data.len()).unwrap();
  }
}

impl Drop for AudioClip {
  fn drop(&mut self) {
    self.engine.clip_delete(self.id).unwrap();
  }
}
