//! The OpenAL backend implementation for the audio subsystem.

use super::*;

/// An abstraction over the host capable of running OpenAL.
///
/// This type implemented by the host application and is used to provide the
/// audio backend with access to the host's OpenAL functions.
pub trait OpenALHost {
  /// Gets the address of an OpenAL function.
  fn get_proc_address(&self, name: &str) -> *const std::ffi::c_void;
}

/// A OpenAL-based [`AudioBackend`] implementation.
pub struct OpenALAudioBackend {}

impl OpenALAudioBackend {
  /// Creates a new OpenAL graphics backend.
  pub fn new(_host: &dyn OpenALHost) -> Self {
    todo!()
  }
}

#[allow(unused_variables)]
impl AudioBackend for OpenALAudioBackend {
  fn new_audio_device(&self) -> Box<dyn AudioDevice> {
    todo!()
  }

  fn new_audio_recorder(&self) -> Box<dyn AudioRecorder> {
    todo!()
  }
}
