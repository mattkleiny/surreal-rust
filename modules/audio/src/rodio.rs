use super::*;

/// A Rodio-based [`AudioBackend`] implementation.
#[derive(Default)]
pub struct RodioAudioBackend {}

#[allow(unused_variables)]
impl AudioBackend for RodioAudioBackend {
  fn new_audio_device(&self) -> Box<dyn AudioDevice> {
    todo!()
  }

  fn new_audio_recorder(&self) -> Box<dyn AudioRecorder> {
    todo!()
  }
}
