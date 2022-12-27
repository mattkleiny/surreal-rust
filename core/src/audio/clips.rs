//! Audio clip management abstractions
//!
//! Audio clips allow for audio data to be uploaded to the audio backend, and played back
//! at a later time via an `AudioSource`.

use std::{cell::RefCell, rc::Rc};

use super::*;

/// A clip contains audio data that can be streamed at runtime into an audio source for playback.
#[derive(Clone)]
pub struct AudioClip {
  state: Rc<RefCell<AudioClipState>>,
}

/// Internal state for an `AudioClip`.
struct AudioClipState {
  handle: AudioHandle,
  audio: AudioServer,
}

impl AudioClip {
  /// Creates a new audio clip.
  pub fn new(audio: &AudioServer) -> Self {
    Self {
      state: Rc::new(RefCell::new(AudioClipState {
        handle: audio.create_clip(),
        audio: audio.clone(),
      })),
    }
  }

  /// Does the clip contain any data?
  pub fn is_empty(&self) -> bool {
    todo!()
  }

  /// The number of bytes in the clip data.
  pub fn len(&self) -> usize {
    todo!()
  }

  /// Writes the given data to the clip.
  pub fn write_data(&mut self, _data: &[u8]) {
    todo!()
  }
}

impl Drop for AudioClipState {
  fn drop(&mut self) {
    self.audio.delete_clip(self.handle);
  }
}

impl AudioResource for AudioClip {
  fn handle(&self) -> AudioHandle {
    self.state.borrow().handle
  }
}
