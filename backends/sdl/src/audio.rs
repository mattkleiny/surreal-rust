// Audio backend for SDL2

pub use audio::*;
use common::Vec3;
use openal_sys::{
  alBufferData, alDeleteSources, alGenBuffers, alGenSources, alGetSourcef, alGetSourcefv, alGetSourcei, alSource3f,
  alSourcePlay, alSourcef, alSourcei, ALCcontext, ALCdevice, ALboolean, ALfloat, ALint, ALuint,
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
      alBufferData(
        buffer.into(),
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
      let mut state: ALint = 0;

      alGetSourcei(source.into(), openal_sys::AL_SOURCE_STATE, &mut state as *mut _);

      match state {
        openal_sys::AL_PLAYING => Some(true),
        _ => Some(false),
      }
    }
  }

  fn source_get_gain(&self, source: SourceId) -> Option<f32> {
    unsafe {
      let mut gain = 0.0f32;

      alGetSourcef(source.into(), openal_sys::AL_GAIN, &mut gain as *mut _);

      Some(gain)
    }
  }

  fn source_set_gain(&self, source: SourceId, gain: f32) -> Result<(), SourceError> {
    unsafe {
      alSourcef(source.into(), openal_sys::AL_GAIN, gain);

      Ok(())
    }
  }

  fn source_get_pitch(&self, source: SourceId) -> Option<f32> {
    unsafe {
      let mut pitch = 0.0f32;

      alGetSourcef(source.into(), openal_sys::AL_PITCH, &mut pitch as *mut _);

      Some(pitch)
    }
  }

  fn source_set_pitch(&self, source: SourceId, pitch: f32) -> Result<(), SourceError> {
    unsafe {
      alSourcef(source.into(), openal_sys::AL_PITCH, pitch);

      Ok(())
    }
  }

  fn source_get_position(&self, source: SourceId) -> Option<Vec3> {
    unsafe {
      let mut position = Vec3::ZERO;

      alGetSourcefv(source.into(), openal_sys::AL_POSITION, &mut position.x as *mut ALfloat);

      Some(position)
    }
  }

  fn source_set_position(&self, source: SourceId, position: Vec3) -> Result<(), SourceError> {
    unsafe {
      alSource3f(
        source.into(),
        openal_sys::AL_POSITION,
        position.x,
        position.y,
        position.z,
      );

      Ok(())
    }
  }

  fn source_set_velocity(&self, source: SourceId, velocity: Vec3) -> Result<(), SourceError> {
    unsafe {
      alSource3f(
        source.into(),
        openal_sys::AL_VELOCITY,
        velocity.x,
        velocity.y,
        velocity.z,
      );

      Ok(())
    }
  }

  fn source_get_velocity(&self, source: SourceId) -> Option<Vec3> {
    unsafe {
      let mut velocity = Vec3::ZERO;

      alGetSourcefv(source.into(), openal_sys::AL_VELOCITY, &mut velocity.x as *mut ALfloat);

      Some(velocity)
    }
  }

  fn source_is_looping(&self, source: SourceId) -> Option<bool> {
    unsafe {
      let mut looping: ALint = 0;

      alGetSourcei(source.into(), openal_sys::AL_LOOPING, &mut looping as *mut _);

      match looping as ALboolean {
        openal_sys::AL_TRUE => Some(true),
        _ => Some(false),
      }
    }
  }

  fn source_set_looping(&self, source: SourceId, looping: bool) -> Result<(), SourceError> {
    unsafe {
      let looping = if looping {
        openal_sys::AL_TRUE
      } else {
        openal_sys::AL_FALSE
      };

      alSourcei(source.into(), openal_sys::AL_LOOPING, looping as ALint);

      Ok(())
    }
  }

  fn source_get_clip(&self, source: SourceId) -> Option<ClipId> {
    unsafe {
      let mut buffer: ALint = 0;

      alGetSourcei(source.into(), openal_sys::AL_BUFFER, &mut buffer as *mut _);

      Some(ClipId::from(buffer as u32))
    }
  }

  fn source_set_clip(&self, source: SourceId, clip: ClipId) -> Result<(), SourceError> {
    unsafe {
      let clip = clip.into();

      alSourcei(source.into(), openal_sys::AL_BUFFER, clip);

      Ok(())
    }
  }

  fn source_play(&self, source: SourceId) -> Result<(), SourceError> {
    unsafe {
      alSourcePlay(source.into());

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
