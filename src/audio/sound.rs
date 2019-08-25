//! Sound management and playback.

use std::sync::Arc;

use super::*;

/// Represents a sound clip that can be played on-device.
pub struct SoundClip {
  id: Arc<AudioClipId>,
  volume: f32,
}