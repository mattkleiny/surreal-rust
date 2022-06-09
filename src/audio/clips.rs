//! Audio clip management abstractions
//!
//! Audio clips allow for audio data to be uploaded to the audio backend, and played back
//! at a later time via an `AudioSource`.

use std::{cell::RefCell, rc::Rc};

use super::*;

#[derive(Clone)]
pub struct AudioClip {
  state: Rc<RefCell<AudioClipState>>,
}

struct AudioClipState {
  handle: AudioHandle,
  audio: AudioServer,
}

impl AudioClip {
  pub fn new(audio: &AudioServer) -> Self {
    Self {
      state: Rc::new(RefCell::new(AudioClipState {
        handle: audio.create_clip(),
        audio: audio.clone(),
      })),
    }
  }

  pub fn is_empty(&self) -> bool {
    todo!()
  }

  pub fn length(&self) -> usize {
    todo!()
  }

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
