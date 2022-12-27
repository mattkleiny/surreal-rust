//! Sprite animations

use std::hash::Hash;

use surreal_core::{
  graphics::{TextureAtlas, TextureRegion},
  utilities::FSM,
};

/// Manages sprite animations and timing.
#[derive(Default)]
pub struct SpriteAnimation<S> {
  sprites: FSM<S, TextureAtlas>,
  frame: u8,
  frame_timer: f32,
  frames_per_second: f32,
}

impl<S: Default + Hash + Eq> SpriteAnimation<S> {
  /// Sets the frames per second of the animation.
  pub fn with_frames_per_second(self, frames_per_second: f32) -> Self {
    Self { frames_per_second, ..self }
  }

  /// Appends a new state/texture atlas pair to the animation.
  pub fn with_sprites(self, state: S, atlas: TextureAtlas) -> Self {
    Self {
      sprites: self.sprites.with(state, atlas),
      ..self
    }
  }

  /// Retrieves the active sprite for the animation, if available.
  pub fn active_sprite(&self) -> Option<TextureRegion> {
    self.sprites.current_data().map(|atlas| atlas.get_region(self.frame as u32, 0))
  }

  /// Sets the current state of the animation.
  pub fn set_state(&mut self, state: S) {
    if self.sprites.set_state(state) {
      self.frame = 0;
    }
  }

  /// Updates the sprite and it's animation state.
  pub fn update(&mut self, delta_time: f32) {
    self.frame_timer += delta_time;

    if self.frame_timer >= 1. / self.frames_per_second {
      if let Some(atlas) = &self.sprites.current_data() {
        self.frame = (self.frame + 1) % atlas.width() as u8;
        self.frame_timer = 0.;
      }
    }
  }
}
