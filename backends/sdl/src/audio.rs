// Audio backend for SDL2

pub use audio::*;
use openal_sys::{
  alBufferData, alDeleteSources, alGenBuffers, alGenSources, alGetSourcei, alSource3f, alSourcef, alSourcei,
  ALCcontext, ALCdevice, ALint, ALuint,
};

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
  fn buffer_create(&self) -> Result<BufferId, BufferError> {
    unsafe {
      let mut buffer: ALuint = 0;

      alGenBuffers(1, &mut buffer as *mut _);

      if buffer == 0 {
        return Err(BufferError::FailedToCreate);
      }

      Ok(BufferId::from(buffer as u32))
    }
  }

  fn buffer_write_data(&self, buffer: BufferId, sampler_rate: AudioSampleRate, data: &[u8]) -> Result<(), BufferError> {
    unsafe {
      let buffer = buffer.into();

      alBufferData(
        buffer,
        openal_sys::AL_FORMAT_MONO16,
        data.as_ptr() as *const _,
        data.len() as i32,
        44100,
      );

      Ok(())
    }
  }

  fn buffer_delete(&self, buffer: BufferId) -> Result<(), BufferError> {
    unsafe {
      let buffer = buffer.into();

      alDeleteSources(1, &buffer as *const _);

      Ok(())
    }
  }

  fn clip_create(&self) -> Result<ClipId, ClipError> {
    unsafe {
      let mut clip: ALuint = 0;

      alGenSources(1, &mut clip as *mut _);

      if clip == 0 {
        return Err(ClipError::FailedToCreate);
      }

      Ok(ClipId::from(clip as u32))
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
      let mut source: ALuint = 0;

      alGenSources(1, &mut source as *mut _);
      alSourcef(source, openal_sys::AL_GAIN, 1.0);
      alSourcef(source, openal_sys::AL_PITCH, 1.0);
      alSource3f(source, openal_sys::AL_POSITION, 0.0, 0.0, 0.0);
      alSource3f(source, openal_sys::AL_VELOCITY, 0.0, 0.0, 0.0);
      alSourcei(source, openal_sys::AL_LOOPING, openal_sys::AL_FALSE as ALint);

      if source == 0 {
        return Err(SourceError::FailedToCreate);
      }

      Ok(SourceId::from(source as u32))
    }
  }

  fn source_is_playing(&self, source: SourceId) -> Option<bool> {
    unsafe {
      let source = source.into();
      let mut state: ALint = 0;

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

      alSourcef(source, openal_sys::AL_GAIN, volume);

      Ok(())
    }
  }

  fn source_get_clip(&self, source: SourceId) -> Option<ClipId> {
    unsafe {
      let source = source.into();
      let mut buffer: ALint = 0;

      alGetSourcei(source, openal_sys::AL_BUFFER, &mut buffer as *mut _);

      Some(ClipId::from(buffer as u32))
    }
  }

  fn source_set_clip(&self, source: SourceId, clip: ClipId) -> Result<(), SourceError> {
    unsafe {
      let source = source.into();
      let clip = clip.into();

      alSourcei(source, openal_sys::AL_BUFFER, clip);

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
