//! A lightweight and fast cross-platform audio engine.

pub use clips::*;

mod clips;

/// Represents a fallible result in the audio subsystem.
pub type AudioResult<T> = anyhow::Result<T>;

/// An opaque handle to an underlying resource in the `AudioServer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AudioHandle(u64);

/// A server for the underlying audio subsystem.
///
/// Permits interaction with the underlying audio API through unsafe lower-level abstraction.
pub unsafe trait AudioServer {
  // clips
  fn create_clip(&self) -> AudioHandle;
  fn upload_clip_data<T>(&self, handle: AudioHandle, data: &[T]);
  fn delete_clip(&self, handle: AudioHandle);
}
