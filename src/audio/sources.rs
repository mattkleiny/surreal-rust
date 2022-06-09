//! Audio source management for the engine.
//!
//! Audio sources are the 'containers' that play audio clips in the underlying runtime.

use std::{cell::RefCell, rc::Rc};

use super::*;

/// A source for audio output in the underlying audio server.
#[derive(Clone)]
pub struct AudioSource {
  state: Rc<RefCell<AudioSourceState>>,
}

/// Internal state for the `AudioSource`.
struct AudioSourceState {
  handle: AudioHandle,
  audio: AudioServer,
}

impl AudioSource {
  /// Creates a new audio source.
  pub fn new(audio: &AudioServer) -> Self {
    AudioSource {
      state: Rc::new(RefCell::new(AudioSourceState {
        handle: audio.create_source(),
        audio: audio.clone(),
      })),
    }
  }

  /// Determines if this audio source is playing.
  pub fn is_playing(&self) -> bool {
    let state = self.state.borrow();

    state.audio.is_source_playing(self.handle())
  }

  /// Gets the volume of this particular audio source.
  pub fn volume(&self) -> f32 {
    let state = self.state.borrow();

    state.audio.get_source_volume(self.handle())
  }

  /// Sets the volume of this particular audio source.
  pub fn set_volume(&mut self, volume: f32) {
    let state = self.state.borrow();

    state.audio.set_source_volume(self.handle(), volume)
  }

  /// Plays the given clip on this audio source.
  ///
  /// If the source was already playing, it will restart.
  pub fn play(&mut self, _clip: &AudioClip) {
    todo!()
  }

  /// Stops the audio source from playing.
  pub fn stop(&mut self) {
    todo!();
  }
}

impl Drop for AudioSourceState {
  fn drop(&mut self) {
    self.audio.delete_source(self.handle);
  }
}

impl AudioResource for AudioSource {
  fn handle(&self) -> AudioHandle {
    self.state.borrow().handle
  }
}
