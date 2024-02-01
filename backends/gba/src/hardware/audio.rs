//! Audio support for the GameBoy Advance.

use crate::GameBoyRuntime;

/// A handle to a sound that is currently playing.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SoundHandle(u16);

/// Represents a sound that can be played on the GameBoy Advance.
pub trait Sound {
  /// Samples the sound at the specified time.
  fn sample(&self, time: f32) -> f32;
}

/// A sound that is represented by a function.
impl<F: Fn(f32) -> f32> Sound for F {
  fn sample(&self, time: f32) -> f32 {
    self(time)
  }
}

/// Represents the audio device of the GameBoy Advance.
pub trait AudioDevice {
  fn play_sound(&mut self, sound: impl Sound) -> SoundHandle;
  fn stop_sound(&mut self, handle: SoundHandle);
  fn stop_all_sounds(&mut self);
}

impl AudioDevice for GameBoyRuntime {
  fn play_sound(&mut self, sound: impl Sound) -> SoundHandle {
    todo!()
  }

  fn stop_sound(&mut self, handle: SoundHandle) {
    todo!()
  }

  fn stop_all_sounds(&mut self) {
    todo!()
  }
}
