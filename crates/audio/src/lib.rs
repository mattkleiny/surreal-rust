//! Audio engine for Surreal.

#![allow(clippy::new_without_default)]

pub use buffers::*;
pub use clips::*;
pub use sampling::*;
pub use sources::*;

mod buffers;
mod clips;
mod headless;
mod sampling;
mod sources;

common::impl_arena_index!(pub BufferId, "Identifies an Audio Buffer.");
common::impl_arena_index!(pub ClipId, "Identifies an Audio Clip.");
common::impl_arena_index!(pub SourceId, "Identifies an Audio Source.");

common::impl_server!(AudioServer by AudioBackend default headless::HeadlessAudioBackend);

/// Gets the audio server instance.
#[inline(always)]
pub fn audio() -> &'static dyn AudioBackend {
  AudioServer::instance()
}

/// A possible error when interacting with buffers.
#[derive(Debug)]
pub enum BufferError {
  InvalidId(BufferId),
  FailedToCreate,
}

/// A possible error when interacting with clips.
#[derive(Debug)]
pub enum ClipError {
  InvalidId(ClipId),
  FailedToCreate,
  NullPointer,
}

/// A possible error when interacting with sources.
#[derive(Debug)]
pub enum SourceError {
  InvalidId(SourceId),
  FailedToCreate,
}

/// Represents a backend implementation for the underlying audio API.
///
/// This is a high-level abstraction that makes use of 'opaque' handles to hide
/// away implementation details. The server is intended to be a low-level
/// implementation abstraction.
#[rustfmt::skip]
pub trait AudioBackend {
  // buffers
  fn buffer_create(&self) -> Result<BufferId, BufferError>;
  fn buffer_write_data(&self, buffer: BufferId, sample_rate: AudioSampleRate, data: &[u8]) -> Result<(), BufferError>;
  fn buffer_delete(&self, buffer: BufferId) -> Result<(), BufferError>;

  // clips
  fn clip_create(&self) -> Result<ClipId, ClipError>;
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
