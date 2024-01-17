use super::*;

/// A OpenAL-based [`AudioBackend`] implementation.
#[derive(Default)]
pub struct OpenALAudioBackend {}

#[allow(unused_variables)]
impl AudioBackend for OpenALAudioBackend {
  fn new_audio_device(&self) -> Box<dyn AudioDevice> {
    todo!()
  }

  fn new_audio_recorder(&self) -> Box<dyn AudioRecorder> {
    todo!()
  }
}
