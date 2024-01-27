//! Audio engine for Surreal.

pub use sampling::*;

mod headless;
mod openal;
mod sampling;

common::impl_rid!(ClipId, "Identifies an Audio Clip.");
common::impl_rid!(SourceId, "Identifies an Audio Source.");

common::impl_server!(AudioEngine, AudioBackend);

impl AudioEngine {
  /// Creates a new [`AudioEngine`] with a no-op, headless backend.
  pub fn headless() -> Self {
    Self::new(headless::HeadlessAudioBackend::default())
  }

  /// Creates a new [`AudioEngine`] with a OpenAL backend.
  pub fn openal() -> Self {
    Self::new(openal::OpenALAudioBackend::new())
  }
}

/// A possible error when interacting with clips.
#[derive(Debug)]
pub enum ClipError {
  InvalidId(ClipId),
  NullPointer,
}

/// A possible error when interacting with sources.
#[derive(Debug)]
pub enum SourceError {
  InvalidId(SourceId),
}

/// Represents a backend implementation for the underlying audio API.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide
/// away implementation details. The server is intended to be a low-level
/// implementation abstraction.
pub trait AudioBackend {
  fn new_audio_device(&self) -> Box<dyn AudioDevice>;
  fn new_audio_recorder(&self) -> Box<dyn AudioRecorder>;
}

/// Represents an audio device.
///
/// A device is capable of playing audio clips via audio sources.
pub trait AudioDevice {
  // clips
  fn clip_create(&self) -> Result<ClipId, ClipError>;
  fn clip_write_data(&self, clip: ClipId, data: *const u8, length: usize) -> Result<(), ClipError>;
  fn clip_delete(&self, clip: ClipId) -> Result<(), ClipError>;

  // sources
  fn source_create(&self) -> Result<SourceId, SourceError>;
  fn source_is_playing(&self, source: SourceId) -> bool;
  fn source_get_volume(&self, source: SourceId) -> f32;
  fn source_set_volume(&self, source: SourceId, volume: f32);
  fn source_get_clip(&self, source: SourceId) -> Option<ClipId>;
  fn source_set_clip(&self, source: SourceId, clip: ClipId);
  fn source_play(&self, source: SourceId);
  fn source_delete(&self, source: SourceId) -> Result<(), SourceError>;
}

/// Represents an audio recorder.
///
/// A recorder is capable of recording audio from the host.
pub trait AudioRecorder {}
