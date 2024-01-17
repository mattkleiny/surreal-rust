use super::*;

/// A headless [`AudioBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
#[derive(Default)]
pub struct HeadlessAudioBackend {}

#[allow(unused_variables)]
impl AudioBackend for HeadlessAudioBackend {
  fn new_audio_device(&self) -> Box<dyn AudioDevice> {
    todo!()
  }

  fn new_audio_recorder(&self) -> Box<dyn AudioRecorder> {
    todo!()
  }
}
