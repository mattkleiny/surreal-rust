//! Music management and playback.

use std::sync::Arc;

use super::*;

/// Represents a music clip that can be played on-device.
pub struct MusicClip {
  id: Arc<AudioClipId>,
  volume: f32,
}
