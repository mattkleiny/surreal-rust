// Audio backend for SDL2

use std::ffi::{c_int, c_uint};

pub use audio::*;
use openal_sys::{alBufferData, alDeleteSources, alGenSources, alGetSourcei, ALCcontext, ALCdevice};

/// An audio backend for SDL2.
pub struct SdlAudioBackend {
  device: *mut ALCdevice,
  context: *mut ALCcontext,
}

impl SdlAudioBackend {
  pub fn new() -> Self {
    let device = unsafe { openal_sys::alcOpenDevice(std::ptr::null_mut()) };
    let context = unsafe { openal_sys::alcCreateContext(device, std::ptr::null_mut()) };

    Self { device, context }
  }
}

impl Drop for SdlAudioBackend {
  fn drop(&mut self) {
    unsafe {
      openal_sys::alcDestroyContext(self.context);
      openal_sys::alcCloseDevice(self.device);
    }
  }
}

#[allow(unused_variables)]
impl AudioBackend for SdlAudioBackend {
  fn clip_create(&self) -> Result<ClipId, ClipError> {
    unsafe {
      let mut clip: c_uint = 0;
      alGenSources(1, &mut clip as *mut _);

      if clip == 0 {
        return Err(ClipError::FailedToCreate);
      }

      Ok(ClipId::from(clip as u32))
    }
  }

  fn clip_write_data(&self, clip: ClipId, data: *const u8, length: usize) -> Result<(), ClipError> {
    unsafe {
      let clip = clip.into();

      alBufferData(
        clip,
        openal_sys::AL_FORMAT_MONO16,
        data as *const _,
        length as i32,
        44100,
      );

      Ok(())
    }
  }

  fn clip_delete(&self, clip: ClipId) -> Result<(), ClipError> {
    unsafe {
      let clip = clip.into();

      alDeleteSources(1, &clip as *const _);

      Ok(())
    }
  }

  fn source_create(&self) -> Result<SourceId, SourceError> {
    unsafe {
      let mut source: c_uint = 0;
      alGenSources(1, &mut source as *mut _);

      if source == 0 {
        return Err(SourceError::FailedToCreate);
      }

      Ok(SourceId::from(source as u32))
    }
  }

  fn source_is_playing(&self, source: SourceId) -> Option<bool> {
    unsafe {
      let source = source.into();
      let mut state: c_int = 0;

      alGetSourcei(source, openal_sys::AL_SOURCE_STATE, &mut state as *mut _);

      match state {
        openal_sys::AL_PLAYING => Some(true),
        _ => Some(false),
      }
    }
  }

  fn source_get_volume(&self, source: SourceId) -> Option<f32> {
    unsafe {
      let source = source.into();
      let mut volume: i32 = 0;

      alGetSourcei(source, openal_sys::AL_GAIN, &mut volume as *mut _);

      Some(volume as f32)
    }
  }

  fn source_set_volume(&self, source: SourceId, volume: f32) -> Result<(), SourceError> {
    unsafe {
      let source = source.into();

      openal_sys::alSourcef(source, openal_sys::AL_GAIN, volume);

      Ok(())
    }
  }

  fn source_get_clip(&self, source: SourceId) -> Option<ClipId> {
    unsafe {
      let source = source.into();
      let mut buffer: c_int = 0;

      alGetSourcei(source, openal_sys::AL_BUFFER, &mut buffer as *mut _);

      Some(ClipId::from(buffer as u32))
    }
  }

  fn source_set_clip(&self, source: SourceId, clip: ClipId) -> Result<(), SourceError> {
    unsafe {
      let source = source.into();
      let clip = clip.into();

      openal_sys::alSourcei(source, openal_sys::AL_BUFFER, clip);

      Ok(())
    }
  }

  fn source_play(&self, source: SourceId) -> Result<(), SourceError> {
    unsafe {
      let source = source.into();

      openal_sys::alSourcePlay(source);

      Ok(())
    }
  }

  fn source_delete(&self, source: SourceId) -> Result<(), SourceError> {
    unsafe {
      let source = source.into();

      alDeleteSources(1, &source as *const _);

      Ok(())
    }
  }
}
