//! Audio engine for Surreal.

pub use clips::*;
pub use sampling::*;
pub use sources::*;

mod clips;
mod headless;
mod openal;
mod sampling;
mod sources;

common::impl_arena_index!(ClipId, "Identifies an Audio Clip.");
common::impl_arena_index!(SourceId, "Identifies an Audio Source.");

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
#[rustfmt::skip]
pub trait AudioBackend {
  // clips
  fn clip_create(&self) -> Result<ClipId, ClipError>;
  fn clip_write_data(&self, clip: ClipId, data: *const u8, length: usize) -> Result<(), ClipError>;
  fn clip_delete(&self, clip: ClipId) -> Result<(), ClipError>;

  // sources
  fn source_create(&self) -> Result<SourceId, SourceError>;
  fn source_is_playing(&self, source: SourceId) -> Option<bool>;
  fn source_get_volume(&self, source: SourceId) -> Option<f32>;
  fn source_set_volume(&self, source: SourceId, volume: f32) -> Result<(), SourceError>;
  fn source_get_clip(&self, source: SourceId) -> Option<ClipId>;
  fn source_set_clip(&self, source: SourceId, clip: ClipId) -> Result<(), SourceError>;
  fn source_play(&self, source: SourceId) -> Result<(), SourceError>;
  fn source_delete(&self, source: SourceId) -> Result<(), SourceError>;
}
